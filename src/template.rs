use askama::Template;

use crate::{form::UserForm, model::User};

#[derive(Debug, Template)]
#[template(path = "list_users.html")]
pub struct UsersTemplate {
    pub users: Vec<(String, User)>,
}

#[derive(Debug, Template)]
#[template(path = "edit_user.html")]
pub struct EditUserTemplate {
    pub success: Option<String>,
    pub error: Option<String>,
    pub form: UserForm,
}
