use std::{convert::TryFrom, path::PathBuf};

use clap::Parser;
use thiserror::Error;

use crate::memory;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct Cli {
  #[clap(long, env = "NES_LOG_LEVEL")]
  pub log: Option<log::LevelFilter>,
  /// Initial address to set program counter to
  ///
  /// This may be any evalexpr expression which evaluates to a valid memory address integer.
  ///
  /// The following variables are exposed to be used in the expression: rom, rom_size, ram, ram_size
  #[clap(short, long, parse(try_from_str = eval_address_expression))]
  pub start_address: Option<memory::Address>,
  /// Program file to load to ROM
  #[clap(name = "FILE", parse(from_os_str))]
  pub file: Option<PathBuf>,
}

#[derive(Error, Debug)]
enum AddressExprError {
  #[error(
    "address {address:#X?} is outside range (expected {:#X?} <= address <= {:#X?})",
    memory::Address::MIN,
    memory::Address::MAX
  )]
  AddressOutOfRange {
    address: evalexpr::IntType,
    source: std::num::TryFromIntError,
  },
  #[error(transparent)]
  AddressExpressionError(#[from] evalexpr::EvalexprError),
}

fn eval_address_expression(
  expression: &str,
) -> std::result::Result<memory::Address, AddressExprError> {
  use AddressExprError::*;
  let context: evalexpr::HashMapContext = evalexpr::context_map! {
    "ram" => evalexpr::IntType::from(memory::constant::RAM_START),
    "ram_size" => evalexpr::IntType::from(memory::constant::RAM_SIZE),
    "rom" => evalexpr::IntType::from(memory::constant::PROGRAM_ROM_START),
    "rom_size" => evalexpr::IntType::from(memory::constant::PROGRAM_ROM_SIZE)
  }?;
  let result = evalexpr::eval_int_with_context(expression, &context)?;
  memory::Address::try_from(result).map_err(|source| AddressOutOfRange {
    address: result,
    source,
  })
}
