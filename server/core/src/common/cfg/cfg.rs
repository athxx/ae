use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub base: Server,
    pub web: Web,
    pub cert: Cert,
    pub system: System,
    pub db: Database,
    pub jwt: Jwt,
    pub log: Log,
}

#[derive(Debug, Deserialize)]
pub struct Jwt {
    pub jwt_secret: String,
    pub jwt_ttl: i64,
}

#[derive(Debug, Deserialize)]
pub struct Log {
    pub log_level: String,
    pub dir: String,
    pub file: String,
    pub enable_oper_log: bool,
}


#[derive(Debug, Deserialize)]
pub struct Server {
    pub name: String,
    pub address: String,
    pub ssl: bool,
    pub content_gzip: bool,
    pub cache_time: u64,
    pub cache_method: u32,
    pub api_prefix: String,
}

#[derive(Debug, Deserialize)]
pub struct Database {
    pub link: String,
}