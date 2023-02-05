use thiserror::Error;

#[derive(Error, Debug)]
pub enum ReportableError {
    #[error("Error occured in HTTP request")]
    HttpError(#[from] reqwest::Error),
    #[error("Error occured in database interaction")]
    DatabaseError(#[from] tokio_postgres::Error),
    #[error("Error occured in getting database handle")]
    DatabasePoolError(#[from] deadpool_postgres::PoolError),
    #[error("Error occured in discord interaction")]
    Discord(#[from] serenity::Error),
    #[error("Internal Error occured: {0}")]
    InternalError(&'static str),
    #[error("{0}")]
    UserError(&'static str),
}
