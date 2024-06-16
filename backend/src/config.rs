pub struct RedisConfig {
    pub host: String,
    pub port: u16,
    pub password: Option<String>,
}

pub fn get_redis_config() -> RedisConfig {
    RedisConfig {
        host: "localhost".to_string(),
        port: 6379,
        password: None,
    }
}
