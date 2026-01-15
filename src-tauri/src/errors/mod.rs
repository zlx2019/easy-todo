use serde::Serialize;
use serde_repr::Serialize_repr;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CmdError {
    #[error("{0}")]
    PlainText(&'static str),
    #[error("Unknown error")]
    Unknown(#[from] anyhow::Error),
}

impl CmdError {
    pub fn error_code(&self) -> ErrorCode {
        match self {
            _ => ErrorCode::Unknown,
        }
    }
}

/// 错误码
#[derive(Debug, Serialize_repr)]
#[repr(u32)]
pub enum ErrorCode {
    Unknown = 0,
}


/// Error json 序列化
impl Serialize for CmdError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        #[derive(Serialize)]
        struct Output {
            error_code: ErrorCode,
            error_message: String
        }
        Output{
            error_code: self.error_code(),
            error_message: self.to_string(),
        }.serialize(serializer)
    }
}
