use std::env;
use dotenvy::dotenv;
use tokio::sync::OnceCell;

// Define struct for server config
#[derive(Debug)]
struct ServerConfig {
    host: String,
    port: u16,
}


// Define struct to represent db config
#[derive(Debug)]
struct DatabaseConfig {
    url: String,
}

// Struct that aggregates server and database configuration
#[derive(Debug)]
pub struct Config {
    server: ServerConfig,
    db: DatabaseConfig
}

impl Config {
    pub fn db_url(&self) -> &str {
        &self.db.url
    }

    pub fn server_host(&self) -> &str {
        &self.server.host
    }

    pub fn server_port(&self) -> &u16 {
        &self.server.port
    }

}


// Storing application configuration on Onecell
pub static  CONFIG: OnceCell<Config> = OnceCell::const_new();

// Inititalization of configuration asynchronously
async fn init_config() -> Config {
    dotenv().ok();

    let server_config = ServerConfig {
        host: env::var("HOST").unwrap_or_else(|_| String::from("127.0.0.1")),
        port: env::var("PORT")
            .unwrap_or_else(|_| String::from("3000"))
            .parse::<u16>()
            .unwrap(),
    };

    let database_config = DatabaseConfig {
        url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
    };

    Config {
        server: server_config,
        db: database_config
    }

}

pub async fn config() -> &'static Config { 
    CONFIG.get_or_init(init_config).await
}