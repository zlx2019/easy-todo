pub mod errors;
use crate::base::errors::CmdError;
pub type CmdResult<T, E = CmdError> = std::result::Result<T, E>;