#[derive(Debug, Copy, Clone)]
pub enum EvaluateError {
    UnknownError
}

pub type Result<T> = std::result::Result<T, EvaluateError>;
