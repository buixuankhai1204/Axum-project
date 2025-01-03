use crate::core::configure;
use crate::core::configure::env::get_env_source;
use jsonwebtoken::{DecodingKey, EncodingKey};
use std::sync::LazyLock;
use std::time::Duration;

pub const MAX_RETRY: u32 = 10;
pub const ENV_PREFIX: &str = "APP";
pub const CODE_LEN: usize = 5;
pub const CLIENT_TIMEOUT: Duration = Duration::from_secs(120);
pub const EXPIRE_SESSION_CODE_SECS: Duration = Duration::from_secs(2000);
pub const EXPIRE_INVITATION_CODE_SECS: Duration = Duration::from_secs(86000);
pub const EXPIRE_BLOCKED_EMAIL_SECS: Duration = Duration::from_secs(300);
pub const EXPIRE_FORGET_PASS_CODE_SECS: Duration = Duration::from_secs(300);
pub const EXPIRE_BEARER_TOKEN_SECS: Duration = Duration::from_secs(1800);
pub const EXPIRE_REFRESH_TOKEN_SECS: Duration = Duration::from_secs(86400);
pub const QUEUE_EMPTY_DELAY_SECS: Duration = Duration::from_secs(60);
pub const COMPLETE_TASK_DELAY_SECS: Duration = Duration::from_secs(10);
pub const CHECK_EMAIL_MESSAGE: &str = "Please check you email.";
pub const AUTHORIZATION: &str = "Authorization";
pub const BEARER: &str = "Bearer";
pub const APP_DOMAIN: &str = "";
pub const APP_EMAIL_ADDR: &str = "";
pub const MINIMUM_DELAY_TIME: Duration = Duration::from_millis(120);
// pub static IMAGES_PATH: Lazy<PathBuf> = Lazy::new(|| get_static_dir().unwrap().join("images"));
// pub static APP_IMAGE: Lazy<PathBuf> = Lazy::new(|| get_static_dir().unwrap().join("images/logo.jpg"));
pub static CONFIG: LazyLock<configure::AppConfig> =
    LazyLock::new(|| configure::AppConfig::read(get_env_source(ENV_PREFIX)).unwrap());
// pub static HTTP: Lazy<reqwest::Client> = Lazy::new(|| HttpClient::build_from_config(&CONFIG).unwrap());
// pub static REDIS: Lazy<RedisClient> = Lazy::new(|| RedisClient::build_from_config(&CONFIG).unwrap());
// pub static EMAIL: Lazy<EmailClient> = Lazy::new(|| EmailClient::build_from_config(&CONFIG).unwrap());
pub static REFRESH_TOKEN_ENCODE_KEY: LazyLock<EncodingKey> = LazyLock::new(|| {
    let key = CONFIG.secret.read_private_refresh_key().unwrap();
    EncodingKey::from_rsa_pem(key.as_bytes()).unwrap()
});
pub static REFRESH_TOKEN_DECODE_KEY: LazyLock<DecodingKey> = LazyLock::new(|| {
    let key = CONFIG.secret.read_public_refresh_key().unwrap();
    DecodingKey::from_rsa_pem(key.as_bytes()).unwrap()
});
pub static ACCESS_TOKEN_ENCODE_KEY: LazyLock<EncodingKey> = LazyLock::new(|| {
    let key = CONFIG.secret.read_private_access_key().unwrap();
    EncodingKey::from_rsa_pem(key.as_bytes()).unwrap()
});
pub static ACCESS_TOKEN_DECODE_KEY: LazyLock<DecodingKey> = LazyLock::new(|| {
    let key = CONFIG.secret.read_public_access_key().unwrap();
    DecodingKey::from_rsa_pem(key.as_bytes()).unwrap()
});
// pub static API_DOC: LazyLock<utoipa::openapi::OpenApi> = LazyLock::new(ApiDoc::openapi);
// pub static TEMPLATE_ENGIN: Lazy<TemplateEngine> = Lazy::new(|| {
//     let path = get_static_dir().unwrap().join("template/**/*").into_os_string().into_string().unwrap();
//     TemplateEngine::new(&path).unwrap()
// });
