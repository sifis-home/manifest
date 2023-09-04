use thiserror::Error;

/// Error types.
#[derive(Debug, Error)]
pub enum Error {
    /// Path format.
    #[error("{0}")]
    FormatPath(&'static str),
    /// A more generic I/O error.
    #[error("I/O error")]
    Io(#[from] std::io::Error),
    #[error("Object error")]
    /// A object output error.
    ObjectOutput(#[from] object::Error),
    #[error("Ureq error")]
    /// A ureq error.
    UreqOutput(#[from] Box<ureq::Error>),
    #[error("Json error")]
    /// A Json output error.
    JsonOutput(#[from] serde_json::Error),
}

/// A specialized `Result` type.
pub type Result<T> = ::std::result::Result<T, Error>;
