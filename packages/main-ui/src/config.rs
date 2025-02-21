#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct FirebaseConfig {
    pub api_key: String,
    pub auth_domain: String,
    pub project_id: String,
    pub storage_bucket: String,
    pub messaging_sender_id: String,
    pub app_id: String,
    pub measurement_id: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct KakaoConfig {
    pub client_id: &'static str,
    pub redirect_uri: &'static str,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct KlaytnConfig {
    pub endpoint: &'static str,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ContractConfig {
    pub shop: &'static str,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Config {
    pub env: &'static str,
    pub domain: &'static str,
    pub main_api_endpoint: &'static str,
    pub nft_metadata_base_url: &'static str,
    pub firebase: FirebaseConfig,
    pub klaytn: KlaytnConfig,
    pub contracts: ContractConfig,
    pub kakao: KakaoConfig,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            env: option_env!("ENV").expect("You must set ENV"),
            domain: option_env!("DOMAIN").expect("You must set DOMAIN"),
            main_api_endpoint: option_env!("MAIN_API_ENDPOINT")
                .unwrap_or("https://api.incheon.world"),
            nft_metadata_base_url: option_env!("NFT_BASE_URI").expect("You must set NFT_BASE_URI"),
            firebase: FirebaseConfig {
                api_key: option_env!("FIREBASE_API_KEY")
                    .expect("You must set FIREBASE_API_KEY")
                    .to_string(),
                auth_domain: option_env!("FIREBASE_AUTH_DOMAIN")
                    .expect("You must set FIREBASE_AUTH_DOMAIN")
                    .to_string(),
                project_id: option_env!("FIREBASE_PROJECT_ID")
                    .expect("You must set FIREBASE_PROJECT_ID")
                    .to_string(),
                storage_bucket: option_env!("FIREBASE_STORAGE_BUCKET")
                    .expect("You must set FIREBASE_STORAGE_BUCKET")
                    .to_string(),
                messaging_sender_id: option_env!("FIREBASE_MESSAGING_SENDER_ID")
                    .expect("You must set FIREBASE_MESSAGING_SENDER_ID")
                    .to_string(),
                app_id: option_env!("FIREBASE_APP_ID")
                    .expect("You must set FIREBASE_APP_ID")
                    .to_string(),
                measurement_id: option_env!("FIREBASE_MEASUREMENT_ID")
                    .expect("You must set FIREBASE_MEASUREMENT_ID")
                    .to_string(),
            },
            klaytn: KlaytnConfig {
                endpoint: option_env!("KLAYTN_ENDPOINT").expect("You must set KLAYTN_ENDPOINT"),
            },
            contracts: ContractConfig {
                shop: option_env!("CONTRACT_SHOP").expect("You must set CONTRACT_SHOP"),
            },
            kakao: KakaoConfig {
                client_id: option_env!("KAKAO_CLIENT_ID").expect("You must set KAKAO_CLIENT_ID"),
                redirect_uri: option_env!("KAKAO_REDIRECT_URI")
                    .expect("You must set KAKAO_REDIRECT_URI"),
            },
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
