#[derive(Debug)]
pub enum ReportableError {
    DatabaseError(tokio_postgres::Error),
    Discord(serenity::Error),
    InternalError(&'static str)
}

impl std::fmt::Display for ReportableError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", *self)
    }
}

impl std::error::Error for ReportableError {}

macro_rules! impl_from_wrap {
    ($t:ty, $n:ident) => {
        impl From<$t> for ReportableError {
            fn from(v: $t) -> Self {
                Self::$n(v)
            }
        }
    };
}

macro_rules! impl_from {
    ($t:ty, $n:ident) => {
        impl From<$t> for ReportableError {
            fn from(_: $t) -> Self {
                Self::$n
            }
        }
    };
}

impl_from_wrap! { tokio_postgres::Error, DatabaseError }
impl_from_wrap! { serenity::Error, Discord}
impl_from_wrap! { &'static str, InternalError }