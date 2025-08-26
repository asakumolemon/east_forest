
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub idle_timeout: std::time::Duration,
    pub max_lifetime: std::time::Duration,
}

impl DatabaseConfig {
    pub fn new(host: String, port: u16, username: String, password: String, database: String, max_connections: u32, min_connections: u32, idle_timeout: std::time::Duration, max_lifetime: std::time::Duration) -> Self {
        Self {
            host,
            port,
            username,
            password,
            database,
            max_connections,
            min_connections,
            idle_timeout,
            max_lifetime,
        }
    }

    pub fn get_url(&self) -> String { 
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username,
            self.password,
            self.host,
            self.port,
            self.database
        )
    }
}
impl Default for DatabaseConfig { 
    fn default() -> Self {
        Self {
            host: dotenvy::var("DB_HOST").unwrap_or("localhost".to_string()),
            port: dotenvy::var("DB_PORT").unwrap_or("5432".to_string()).parse().unwrap(),
            username: dotenvy::var("DB_USERNAME").unwrap_or("postgres".to_string()),
            password: dotenvy::var("DB_PASSWORD").unwrap_or("123456".to_string()),
            database: dotenvy::var("DB_DATABASE").unwrap_or("east_forest".to_string()),
            max_connections: dotenvy::var("DB_MAX_CONNECTIONS").unwrap_or("10".to_string()).parse().unwrap(),
            min_connections: dotenvy::var("DB_MIN_CONNECTIONS").unwrap_or("1".to_string()).parse().unwrap(),
            idle_timeout: std::time::Duration::from_secs(
                dotenvy::var("DB_IDLE_TIMEOUT").unwrap_or("5".to_string()).parse().unwrap()
            ),
            max_lifetime: std::time::Duration::from_secs(
                dotenvy::var("DB_MAX_LIFETIME").unwrap_or("60".to_string()).parse().unwrap()
            ),
        }
    }
}