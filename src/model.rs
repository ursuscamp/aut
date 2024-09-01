use std::collections::HashMap;

use anyhow::Context;
use serde::{Deserialize, Serialize};

use crate::form::UserForm;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(default)]
pub struct User {
    pub disabled: bool,
    pub displayname: String,
    pub email: String,
    pub password: String,
    pub groups: Vec<String>,
}

impl From<UserForm> for User {
    fn from(value: UserForm) -> Self {
        let password = value.hashed_password();
        let UserForm {
            name: _,
            disabled,
            displayname,
            email,
            password: _,
            confirm_password: _,
            groups,
        } = value;
        User {
            disabled: disabled.is_some(),
            displayname,
            email,
            password,
            groups: groups.split_whitespace().map(ToString::to_string).collect(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UserDatabase {
    pub users: HashMap<String, User>,
}

impl UserDatabase {
    pub fn from_file(path: &std::path::Path) -> anyhow::Result<UserDatabase> {
        let db_str = std::fs::read_to_string(path).context("Reading db file")?;
        let db: UserDatabase = serde_yaml::from_str(&db_str).context("Parsing db file")?;
        Ok(db)
    }

    pub fn persist(&self, path: &std::path::Path) -> anyhow::Result<()> {
        let db_str = serde_yaml::to_string(&self)?;
        std::fs::write(path, db_str)?;
        Ok(())
    }
}
