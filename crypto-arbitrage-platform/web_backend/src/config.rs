use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub database: DatabaseConfig,
    pub app: AppConfig,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Self {
            database: DatabaseConfig {
                host: std::env::var("DB_HOST").unwrap_or_else(|_| "localhost".into()),
                port: std::env::var("DB_PORT")
                    .ok()
                    .and_then(|p| p.parse().ok())
                    .unwrap_or(5432),
                username: std::env::var("DB_USER").unwrap_or_else(|_| "postgres".into()),
                password: std::env::var("DB_PASSWORD").unwrap_or_else(|_| "postgres".into()),
                name: std::env::var("DB_NAME").unwrap_or_else(|_| "crypto_arbitrage".into()),
            },
            app: AppConfig {
                host: std::env::var("APP_HOST").unwrap_or_else(|_| "0.0.0.0".into()),
                port: std::env::var("APP_PORT")
                    .ok()
                    .and_then(|p| p.parse().ok())
                    .unwrap_or(8080),
            },
        })
    }
}

#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub name: String,
}

impl DatabaseConfig {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.name
        )
    }
}

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
}
