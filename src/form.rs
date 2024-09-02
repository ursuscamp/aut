use argon2::{password_hash::SaltString, Argon2, ParamsBuilder, PasswordHasher};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};

use crate::model::User;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct UserForm {
    pub name: String,
    pub disabled: Option<String>,
    pub displayname: String,
    pub email: String,
    pub password: String,
    pub confirm_password: String,
    pub groups: String,
}

impl UserForm {
    pub fn hashed_password(&self) -> String {
        let salt = SaltString::generate(&mut OsRng);
        //let a2 = Argon2::default();
        let a2 = Argon2::new(
            argon2::Algorithm::Argon2id,
            argon2::Version::V0x13,
            ParamsBuilder::new()
                .m_cost(65536)
                .t_cost(3)
                .p_cost(4)
                .build()
                .unwrap(),
        );
        let password_hash = a2
            .hash_password(self.password.as_bytes(), &salt)
            .unwrap()
            .to_string();
        password_hash
    }

    pub fn validate(&self) -> Option<String> {
        if self.name.is_empty() {
            return Some("Name must be present.".into());
        }

        if self.displayname.is_empty() {
            return Some("Display name must be present.".into());
        }

        if self.password.is_empty() {
            return Some("Password must be supplied.".into());
        }

        if self.password != self.confirm_password {
            return Some("Passwords do not match.".into());
        }

        None
    }
}

impl From<User> for UserForm {
    fn from(value: User) -> Self {
        #[rustfmt::skip]
        let User { disabled, displayname, email, password: _, groups } = value;
        UserForm {
            name: String::new(),
            disabled: if disabled {
                Some(String::from("disabled"))
            } else {
                None
            },
            displayname,
            email,
            password: String::new(),
            confirm_password: String::new(),
            groups: groups.join(" "),
        }
    }
}
