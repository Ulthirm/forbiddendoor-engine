use colored::Colorize;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqlitePool;
use std::{
    fs,
    io::{self, Write},
    str::FromStr,
};
use sqlx::{Pool, Sqlite};
use tracing::{level_filters::LevelFilter};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub logging: Logging,
    pub discord: Discord,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Logging {
    #[serde(default = "default_logging_level")]
    pub level: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Discord {
    #[serde(default = "default_discord")]
    pub bot_token: String,
}

fn default_logging_level() -> String {
    "Debug".to_string()
}

fn default_discord() -> String {
    "".to_string()
}

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    let config_path = "config.toml";
    match fs::read_to_string(config_path) {
        Ok(config_str) => match toml::from_str::<Config>(&config_str) {
            Ok(config) => {
                verify_config(&config);
                config
            }
            Err(e) => {
                println!(
                    "{}{}{}",
                    "Warn:".yellow().bold(),
                    "Failed to parse config: ",
                    e
                );
                repair_config(config_str).expect("Failed to repair config");
                let repaired_config_str =
                    fs::read_to_string(config_path).expect("Failed to read repaired config");
                toml::from_str(&repaired_config_str).expect("Failed to parse repaired config")
            }
        },
        Err(e) => {
            if e.kind() == io::ErrorKind::NotFound {
                create_config().expect("Failed to create config");
            } else {
                panic!("Failed to read config file: {}", e);
            }
            let new_config_str =
                fs::read_to_string(config_path).expect("Failed to read new config");
            toml::from_str(&new_config_str).expect("Failed to parse new config")
        }
    }
});

pub async fn get_sqlite_pool() -> Pool<Sqlite> {
    SqlitePool::connect("sqlite:config.db").await.expect("Failed to connect to SQLite database")
}

pub async fn get_config() -> &'static Config {
    &CONFIG
}

fn create_config() -> io::Result<()> {
    println!(
        "{}{}",
        "Info:".green().bold(),
        "Creating a new config file..."
    );

    let mut config_file = fs::File::create("config.toml")?;

    let config_data = r#"[logging]
    level = "Info"

    [discord]
    bot_token = ""
    "#;

    let config_bytes = config_data.as_bytes();
    config_file.write_all(config_bytes)?;
    Ok(())
}

pub fn get_logging_config() -> LevelFilter {
    let log_level_str = &CONFIG.logging.level;
    let level_filter = LevelFilter::from_str(log_level_str).unwrap_or_else(|_| {
        eprintln!(
            "Warn: Unable to parse log level from config: {}. Defaulting to 'Debug'",
            log_level_str
        );
        LevelFilter::DEBUG
    });

    println!(
        "{}{}{:?}",
        "Info:".green().bold(),
        "Logging level: ",
        level_filter
    );

    level_filter
}

pub async fn get_server_settings(pool: &SqlitePool, setting: &str) -> String {
    let row: (String,) = sqlx::query_as("SELECT value FROM settings WHERE key = ?")
        .bind(setting)
        .fetch_one(pool)
        .await
        .expect("Failed to fetch setting from SQLite database");

    row.0
}

pub async fn set_server_settings(pool: &SqlitePool, setting: &str, value: &str) {
    sqlx::query("INSERT OR REPLACE INTO settings (key, value) VALUES (?, ?)")
        .bind(setting)
        .bind(value)
        .execute(pool)
        .await
        .expect("Failed to insert or replace setting in SQLite database");
}

fn verify_config(config: &Config) {
    let log_level_str = &config.logging.level;
    if log_level_str.is_empty() {
        println!("{}{}","Warn:".yellow().bold(), "Empty log level found in config\n This is not a valid log level and will be defaulted to 'Debug'");
    }

    let discord_token = &config.discord.bot_token;
    if discord_token.is_empty() {
        println!("{}{}","ERROR:".red().bold(), "Empty discord token found in config\n This is not a valid token and will be ignored.\n This means the bot will not work.");
    }
}

fn repair_config(config_str: String) -> io::Result<()> {
    println!(
        "{}{}",
        "Warn:".yellow().bold(),
        "Repairing the Config file..."
    );

    let current_config_str = config_str;

    let logging: Logging = toml::from_str(&current_config_str).unwrap_or_else(|_| Logging {
        level: default_logging_level(),
    });

    let discord: Discord = toml::from_str(&current_config_str).unwrap_or_else(|_| Discord {
        bot_token: default_discord(),
    });

    let rebuilt_config = Config {
        logging: logging,
        discord: discord,
    };

    let mut file = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("config.toml")?;

    writeln!(file, "{}", toml::to_string(&rebuilt_config).unwrap())?;
    Ok(())
}
