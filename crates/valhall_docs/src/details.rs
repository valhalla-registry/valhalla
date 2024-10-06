use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub(crate) struct CrateRelease {
    pub id: i32,
    pub version: String,
}
