///
pub type Result<T> = core::result::Result<T, EvaluateError>;

///
#[derive(Debug, Copy, Clone)]
pub enum EvaluateError {
    ///
    UnknownError,
}
