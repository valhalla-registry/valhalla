use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, FromRow)]
pub struct Account {
    pub id: String,
    pub email: String,
    pub name: String,
    pub password: Option<String>,
}
