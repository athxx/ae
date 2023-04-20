use dotenv::dotenv;
use fast_log;
use once_cell::sync::Lazy;
use std::env;
use utils::envx::{env_bool, env_str};

pub static ALL: Lazy<Config> = Lazy::new(|| Config {
    debug: !cfg!(debug_assertions) && !env_bool("debug"),
    port_api: env_str("port_api"),
    port_auth: env_str("port_api"),
    redis_url: env_str("redis_url"),
    db_url: env_str("db_url"),
    db_auth: env_str("db_auth"),
    db_name: env_str("db_name"),
    db_show_log: env_bool("db_show_log"),
    db_max_lifetime: env_bool("db_max_lifetime"),
    db_max_open_conns: env_bool("db_max_open_conns"),
    db_max_idle_conns: env_bool("db_max_idle_conns"),
    log_dir: env_str("log_dir"),
    log_level: env_str("log_level"),
    jwt_secret: env_str("jwt_secret"),
    sms_ak: env_str("sms_ak"),
    sms_sk: env_str("sms_sk"),
    mail_ak: env_str("mail_ak"),
    mail_sk: env_str("mail_sk"),
    mail_smtp: env_str("mail_smtp"),
    oss_ak: env_str("oss_ak"),
    oss_sk: env_str("oss_sk"),
    oss_bucket: env_str("oss_bucket"),
    oss_endpoint: env_str("oss_endpoint"),
    queue_url: env_str("queue_url"),
    search_url: env_str("search_url"),
});

pub struct Config {
    pub debug: bool,
    // PORTS
    pub port_api: String,
    pub port_auth: String,
    // cache type: file, memory, REDIS
    pub cache_adpt: String,
    pub cache_url: String,
    // redis
    pub redis_url: String,

    // DATABASE
    pub db_url: String,
    pub db_auth: String,
    pub db_name: String,
    pub db_show_log: bool,
    pub db_max_lifetime: bool,
    pub db_max_open_conns: bool,
    pub db_max_idle_conns: bool,
    // LOGGER
    pub log_dir: String,
    pub log_level: String,
    // JWT
    pub jwt_ttl: u32,
    pub jwt_secret: String,
    // SMS
    pub sms_ak: String,
    pub sms_sk: String,
    // EMAIL
    pub mail_ak: String,
    pub mail_sk: String,
    pub mail_smtp: String,
    // OSS
    pub oss_ak: String,
    pub oss_sk: String,
    pub oss_bucket: String,
    pub oss_endpoint: String,
    // QUEUE
    pub queue_url: String,
    // SEARCH, elasticsearch
    pub search_url: String,
}

pub fn init() {
    dotenv().ok();
    let _vars = env::vars();
    fast_log::init(fast_log::Config::new().console()).unwrap();
}
