pub mod user {
    use std::sync::Arc;

    use axum::{
        extract::{Path, State},
        http::StatusCode,
        response::Redirect,
        Form,
    };

    use crate::{
        config::Config,
        form::UserForm,
        model::{User, UserDatabase},
        template::{EditUserTemplate, UsersTemplate},
    };

    pub async fn list(State(config): State<Arc<Config>>) -> Result<UsersTemplate, StatusCode> {
        tracing::debug!("Retrieving user list.");
        let db = UserDatabase::from_file(&config.users_file)
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        let mut sorted_users = db.users.into_iter().collect::<Vec<_>>();
        sorted_users.sort_by(|a, b| a.0.cmp(&b.0));
        Ok(UsersTemplate {
            users: sorted_users,
        })
    }

    pub async fn edit(
        State(config): State<Arc<Config>>,
        axum::extract::Path(username): Path<String>,
    ) -> Result<EditUserTemplate, StatusCode> {
        tracing::debug!("Editing user {username}.");
        let db = UserDatabase::from_file(&config.users_file)
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        let user = db.users.get(&username).cloned().unwrap_or_default();
        let mut form: UserForm = user.into();
        form.name = username;
        Ok(EditUserTemplate {
            success: None,
            error: None,
            form,
        })
    }

    pub async fn save(
        State(config): State<Arc<Config>>,
        Form(form): Form<UserForm>,
    ) -> Result<EditUserTemplate, StatusCode> {
        tracing::debug!("Saving user.");
        let mut db = UserDatabase::from_file(&config.users_file)
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        if let Some(error) = form.validate() {
            return Ok(EditUserTemplate {
                success: None,
                error: Some(error),
                form,
            });
        }
        let user: User = form.clone().into();
        db.users.insert(form.name.clone(), user);
        db.persist(&config.users_file)
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        Ok(EditUserTemplate {
            success: Some(String::from("User saved.")),
            error: None,
            form,
        })
    }

    pub async fn delete(
        State(config): State<Arc<Config>>,
        Path(username): Path<String>,
    ) -> Result<Redirect, StatusCode> {
        let mut db = UserDatabase::from_file(&config.users_file)
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        db.users.remove(&username);
        db.persist(&config.users_file)
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        Ok(Redirect::permanent("/"))
    }
}
