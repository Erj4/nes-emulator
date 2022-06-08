use crate::cpu::Operation;
use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
  #[error("unimplemented operation: {0}")]
  UnimplementedOperation(Operation),
  #[error("io error: {0}")]
  IoError(io::Error),
}

impl From<io::Error> for Error {
  fn from(err: io::Error) -> Self {
    Self::IoError(err)
  }
}
