use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CacheMode {
    Memory,
    Filesystem,
    Off,
}

impl CacheMode {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "memory" => CacheMode::Memory,
            "filesystem" => CacheMode::Filesystem,
            "off" => CacheMode::Off,
            _ => CacheMode::Memory,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Config {
    pub cache_mode: CacheMode,
    pub cache_dir: PathBuf,
    pub cache_duration: u64,
    pub verbose: bool,
    pub timeout: u64,
    pub user_agent: String,
}

impl Default for Config {
    fn default() -> Self {
        let cache_dir = dirs::cache_dir()
            .unwrap_or_else(|| PathBuf::from("/tmp"))
            .join("nflreadrust");

        Config {
            cache_mode: CacheMode::Memory,
            cache_dir,
            cache_duration: 86400,
            verbose: false,
            timeout: 120,
            user_agent: format!("nflverse/nflreadrust {}", env!("CARGO_PKG_VERSION")),
        }
    }
}

impl Config {
    pub fn from_env() -> Self {
        let mut config = Config::default();

        if let Ok(val) = std::env::var("NFLREADRUST_CACHE") {
            config.cache_mode = CacheMode::from_str(&val);
        }
        if let Ok(val) = std::env::var("NFLREADRUST_CACHE_DIR") {
            config.cache_dir = PathBuf::from(val);
        }
        if let Ok(val) = std::env::var("NFLREADRUST_CACHE_DURATION") {
            if let Ok(n) = val.parse() {
                config.cache_duration = n;
            }
        }
        if let Ok(val) = std::env::var("NFLREADRUST_VERBOSE") {
            config.verbose = val == "1" || val.to_lowercase() == "true";
        }
        if let Ok(val) = std::env::var("NFLREADRUST_TIMEOUT") {
            if let Ok(n) = val.parse() {
                config.timeout = n;
            }
        }
        if let Ok(val) = std::env::var("NFLREADRUST_USER_AGENT") {
            config.user_agent = val;
        }

        config
    }
}

static GLOBAL_CONFIG: OnceLock<Mutex<Config>> = OnceLock::new();

pub fn get_config() -> Config {
    GLOBAL_CONFIG
        .get_or_init(|| Mutex::new(Config::from_env()))
        .lock()
        .unwrap()
        .clone()
}

pub fn update_config(config: Config) {
    let global = GLOBAL_CONFIG.get_or_init(|| Mutex::new(Config::from_env()));
    let mut guard = global.lock().unwrap();
    *guard = config;
}
