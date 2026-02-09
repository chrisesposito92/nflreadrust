use polars::prelude::*;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Mutex;
use std::sync::OnceLock;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::config::{CacheMode, get_config};

struct CacheEntry {
    df: DataFrame,
    timestamp: u64,
}

struct MemoryCache {
    entries: HashMap<String, CacheEntry>,
}

impl MemoryCache {
    fn new() -> Self {
        MemoryCache {
            entries: HashMap::new(),
        }
    }
}

static MEMORY_CACHE: OnceLock<Mutex<MemoryCache>> = OnceLock::new();

fn get_memory_cache() -> &'static Mutex<MemoryCache> {
    MEMORY_CACHE.get_or_init(|| Mutex::new(MemoryCache::new()))
}

fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

pub fn make_cache_key(url: &str) -> String {
    format!("{:x}", md5::compute(url))
}

pub fn cache_get(key: &str) -> Option<DataFrame> {
    let config = get_config();
    match config.cache_mode {
        CacheMode::Off => None,
        CacheMode::Memory => {
            let cache = get_memory_cache().lock().unwrap();
            if let Some(entry) = cache.entries.get(key) {
                if now_secs() - entry.timestamp < config.cache_duration {
                    return Some(entry.df.clone());
                }
            }
            None
        }
        CacheMode::Filesystem => {
            let path = cache_file_path(&config.cache_dir, key);
            if path.exists() {
                if let Ok(metadata) = std::fs::metadata(&path) {
                    if let Ok(modified) = metadata.modified() {
                        let age = SystemTime::now()
                            .duration_since(modified)
                            .unwrap_or_default()
                            .as_secs();
                        if age < config.cache_duration {
                            if let Ok(df) = ParquetReader::new(
                                std::fs::File::open(&path).unwrap(),
                            )
                            .finish()
                            {
                                return Some(df);
                            }
                        }
                    }
                }
            }
            None
        }
    }
}

pub fn cache_set(key: &str, df: &DataFrame) {
    let config = get_config();
    match config.cache_mode {
        CacheMode::Off => {}
        CacheMode::Memory => {
            let mut cache = get_memory_cache().lock().unwrap();
            cache.entries.insert(
                key.to_string(),
                CacheEntry {
                    df: df.clone(),
                    timestamp: now_secs(),
                },
            );
        }
        CacheMode::Filesystem => {
            let path = cache_file_path(&config.cache_dir, key);
            if let Some(parent) = path.parent() {
                let _ = std::fs::create_dir_all(parent);
            }
            if let Ok(file) = std::fs::File::create(&path) {
                let _ = ParquetWriter::new(file).finish(&mut df.clone());
            }
        }
    }
}

pub fn clear_cache(pattern: Option<&str>) {
    let config = get_config();

    // Clear memory cache
    {
        let mut cache = get_memory_cache().lock().unwrap();
        match pattern {
            None => cache.entries.clear(),
            Some(pat) => cache.entries.retain(|k, _| !k.contains(pat)),
        }
    }

    // Clear filesystem cache
    if config.cache_mode == CacheMode::Filesystem {
        if config.cache_dir.exists() {
            match pattern {
                None => {
                    let _ = std::fs::remove_dir_all(&config.cache_dir);
                    let _ = std::fs::create_dir_all(&config.cache_dir);
                }
                Some(_pat) => {
                    if let Ok(entries) = std::fs::read_dir(&config.cache_dir) {
                        for entry in entries.flatten() {
                            let name = entry.file_name().to_string_lossy().to_string();
                            if name.contains(_pat) {
                                let _ = std::fs::remove_file(entry.path());
                            }
                        }
                    }
                }
            }
        }
    }
}

fn cache_file_path(cache_dir: &PathBuf, key: &str) -> PathBuf {
    cache_dir.join(format!("{key}.parquet"))
}
