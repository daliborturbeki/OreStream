pub struct Config {
    pub arete_api_key: String,
    pub database_url: String,
    pub redis_url: String,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            arete_api_key: std::env::var("ARETE_API_KEY").expect("ARETE_API_KEY must be set in .env or environment"),
            database_url: std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env or environment"),
            redis_url: std::env::var("REDIS_URL").unwrap_or("redis://127.0.0.1:6379".into()),
            port: std::env::var("PORT").ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(3000),
        }
    }
}