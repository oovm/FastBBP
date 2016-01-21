///
pub type Result<T> = std::result::Result<T, EvaluateError>;

///
#[derive(Debug, Copy, Clone)]
pub enum EvaluateError {
    ///
    UnknownError,
}
