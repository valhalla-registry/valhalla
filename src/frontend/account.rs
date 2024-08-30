use askama::Template;
use axum::response::Redirect;

#[derive(Template)]
#[template(path = "account/login.html")]
pub(crate) struct LoginTemplate {}

#[derive(Template)]
#[template(path = "account/register.html")]
pub(crate) struct RegisterTemplate {}

#[derive(Template)]
#[template(path = "account/register.html")] // fixme
pub(crate) struct ProfileTemplate {}

pub async fn login_handler() -> LoginTemplate {
    LoginTemplate {}
}

pub async fn register_handler() -> RegisterTemplate {
    RegisterTemplate {}
}

pub async fn profile_handler() -> Result<ProfileTemplate, Redirect> {
    Err(Redirect::to("/account/login"))
}

pub async fn token_handler() -> Result<ProfileTemplate, Redirect> {
    Err(Redirect::to("/account/login"))
}
