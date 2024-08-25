use askama::Template;

#[derive(Template)]
#[template(path = "account/login.html")]
pub(crate) struct LoginTemplate {}

#[derive(Template)]
#[template(path = "account/register.html")]
pub(crate) struct RegisterTemplate {}

pub async fn login_handler() -> LoginTemplate {
    LoginTemplate {}
}

pub async fn register_handler() -> RegisterTemplate {
    RegisterTemplate {}
}
