#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Anyhow(#[from] anyhow::Error),

    #[error("IO Error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Database Error: {0}")]
    SqlxError(#[from] sqlx::Error),

    #[error("CommandError: {0}")]
    CommandError(#[from] rustwide::cmd::CommandError),
}
