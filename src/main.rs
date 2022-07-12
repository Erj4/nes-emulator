#![warn(clippy::pedantic)]
#![allow(clippy::enum_glob_use)]
#![warn(clippy::print_stdout, clippy::print_stderr)]
#![warn(clippy::self_named_module_files)]
#![warn(clippy::fn_to_numeric_cast_any)]
#![warn(clippy::let_underscore_must_use)]
// #![warn(clippy::missing_docs_in_private_items)]
#![warn(clippy::shadow_unrelated)]
#![warn(clippy::str_to_string)]
#![warn(clippy::string_to_string)]
#![warn(clippy::unneeded_field_pattern)]
#![warn(clippy::unwrap_in_result)]
// #![warn(clippy::unwrap_used)]

use std::fs::File;

use clap::Parser;
use env_logger::Builder;
use log::LevelFilter;

mod cli;
pub mod cpu;
pub mod memory;

fn main() -> anyhow::Result<()> {
  let args = cli::Cli::parse();

  let log_builder = &mut Builder::new();
  log_builder.filter_level(LevelFilter::Warn);

  if let Some(filter) = args.log {
    log_builder.filter_level(filter);
  }

  log_builder.init();

  let mut cpu: cpu::Cpu = cpu::Cpu::default();

  if let Some(path) = &args.file {
    let mut file = File::open(&path)?;
    cpu.load_from(&mut file)?;
  };

  match args.start_address {
    None => cpu.start()?,
    Some(address) => cpu.start_from(address)?,
  };

  Ok(())
}
