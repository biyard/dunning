#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Config {
    pub env: &'static str,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            env: option_env!("ENV").expect("You must set ENV"),
        }
    }
}

static mut CONFIG: Option<Config> = None;

#[allow(static_mut_refs)]
pub fn get() -> &'static Config {
    unsafe {
        if CONFIG.is_none() {
            CONFIG = Some(Config::default());
        }
        CONFIG.as_ref().unwrap()
    }
}

pub fn log_level() -> tracing::Level {
    match option_env!("RUST_LOG") {
        Some("trace") => tracing::Level::TRACE,
        Some("debug") => tracing::Level::DEBUG,
        Some("info") => tracing::Level::INFO,
        Some("warn") => tracing::Level::WARN,
        Some("error") => tracing::Level::ERROR,
        _ => tracing::Level::INFO,
    }
}
