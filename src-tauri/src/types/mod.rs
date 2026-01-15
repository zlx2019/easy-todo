use crate::errors::CmdError;

pub mod state;
pub mod store;

pub type CmdResult<T, E = CmdError> = std::result::Result<T, E>;