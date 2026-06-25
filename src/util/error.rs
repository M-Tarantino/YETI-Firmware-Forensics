use std::fmt;

pub type YetiResult<T> = Result<T, YetiError>;

#[derive(Debug)]
pub enum YetiError {
    Io(std::io::Error),
    Database(rusqlite::Error),
    Vfs(String),
    Scan(String),
    Interface(String),
    Network(String),
}

impl std::error::Error for YetiError {}

impl fmt::Display for YetiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            YetiError::Io(e) => write!(f, "Hardware/IO Exception: {}", e),
            YetiError::Database(e) => write!(f, "Signature DB Exception: {}", e),
            YetiError::Vfs(s) => write!(f, "Virtual FS Failure: {}", s),
            YetiError::Scan(s) => write!(f, "Forensic Scan Failure: {}", s),
            YetiError::Interface(s) => write!(f, "UI Exception: {}", s),
            YetiError::Network(s) => write!(f, "Distributed Node Failure: {}", s),
        }
    }
}

impl From<std::io::Error> for YetiError { fn from(e: std::io::Error) -> Self { Self::Io(e) } }
impl From<rusqlite::Error> for YetiError { fn from(e: rusqlite::Error) -> Self { Self::Database(e) } }