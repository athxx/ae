use std::sync::OnceLock;
use utils::envx::{env_bool, env_i32, env_str};

static ALL: OnceLock<Config> = OnceLock::new();

pub struct Config {
    pub debug: bool,
    // PORTS
    pub port_api: String,
    pub port_auth: String,
    // REDIS
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
    pub jwt_ttl: i32,
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

    // KAFKA
    pub kafka_comsumer_url: String,
    pub kafka_producer_url: String,

    // SEARCH, elasticsearch
    pub search_url: String,
}

pub fn get_or_init() -> &'static Config {
    ALL.get_or_init(|| -> Config {
        dotenvy::dotenv().ok();
        Config {
            debug: !cfg!(debug_assertions) && !env_bool("debug"),
            port_api: env_str("port_api"),
            port_auth: env_str("port_api"),

            // REDIS
            redis_url: env_str("redis_url"),

            // DATABASE
            db_url: env_str("db_url"),
            db_auth: env_str("db_auth"),
            db_name: env_str("db_name"),
            db_show_log: env_bool("db_show_log"),
            db_max_lifetime: env_bool("db_max_lifetime"),
            db_max_open_conns: env_bool("db_max_open_conns"),
            db_max_idle_conns: env_bool("db_max_idle_conns"),

            // LOG
            log_dir: env_str("log_dir"),
            log_level: env_str("log_level"),

            // JWT
            jwt_ttl: env_i32("jwt_ttl"),
            jwt_secret: env_str("jwt_secret"),

            // SMS
            sms_ak: env_str("sms_ak"),
            sms_sk: env_str("sms_sk"),

            // MAIL
            mail_ak: env_str("mail_ak"),
            mail_sk: env_str("mail_sk"),
            mail_smtp: env_str("mail_smtp"),

            // OSS
            oss_ak: env_str("oss_ak"),
            oss_sk: env_str("oss_sk"),
            oss_bucket: env_str("oss_bucket"),
            oss_endpoint: env_str("oss_endpoint"),

            // KAFKA
            kafka_comsumer_url: env_str("kafka_comsumer_url"),
            kafka_producer_url: env_str("kafka_producer_url"),
            search_url: env_str("search_url"),
        }
    })
}
