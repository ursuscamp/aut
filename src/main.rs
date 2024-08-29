use std::{
    collections::HashMap,
    env,
    path::{Path, PathBuf},
};

use anyhow::{anyhow, Context};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let config = Config::new()?;
    let db = UserDatabase::from_file(&config.users_file)?;

    Ok(())
}

struct Config {
    host: String,
    port: String,
    passwd: String,
    users_file: PathBuf,
}

impl Config {
    pub fn new() -> anyhow::Result<Config> {
        tracing::info!("Loading config from environment");
        let host = env::var("AUT_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
        let port = env::var("AUT_PORT").unwrap_or_else(|_| "7799".to_string());
        let passwd = env::var("AUT_ADMIN_PASS").unwrap_or_else(|_| "admin123".to_string());
        let users_file = env::var("AUT_USERS_FILE").context("AUT_USERS_FILE")?;
        tracing::debug!("Config loaded successfully.");

        Ok(Config {
            host,
            port,
            passwd,
            users_file: users_file.into(),
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(default)]
struct User {
    disabled: bool,
    displayname: String,
    email: String,
    password: String,
    groups: Vec<String>,
}

impl User {
    pub fn password_hash(&self) -> anyhow::Result<PasswordHash> {
        match PasswordHash::new(&self.password) {
            Ok(pw) => Ok(pw),
            Err(_) => anyhow::bail!(
                "User {} (email {}) has invalid password hash",
                self.displayname,
                self.email
            ),
        }
    }

    pub fn verify_password(&self, passwd: &str) -> anyhow::Result<bool> {
        let hash = self.password_hash()?;
        let valid = Argon2::default()
            .verify_password(passwd.as_bytes(), &hash)
            .is_ok();
        Ok(valid)
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(default)]
struct UserDatabase {
    users: HashMap<String, User>,
}

impl UserDatabase {
    pub fn from_file(path: &Path) -> anyhow::Result<UserDatabase> {
        let db_str = std::fs::read_to_string(path).context("Reading db file")?;
        let db: UserDatabase = serde_yaml::from_str(&db_str).context("Parsing db file")?;
        Ok(db)
    }
}
