#[derive(Debug, Copy, Clone)]
pub enum DocumentError {
    UnknownError,
}

pub type Result<T> = std::result::Result<T, DocumentError>;
