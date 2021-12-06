#![feature(exclusive_range_pattern)]
#![feature(fn_traits)]
#![feature(half_open_range_patterns)]
#![feature(type_ascription)]
#![feature(unboxed_closures)]
#![warn(clippy::pedantic)]
#![allow(clippy::enum_glob_use)]

use std::{error::Error, fs::File};

use env_logger::Builder;
use log::{info, warn, LevelFilter};
use structopt::StructOpt;

mod cli;
pub mod cpu;
pub mod memory;

fn main() -> Result<(), Box<dyn Error>> {
  let args = cli::Cli::from_args();

  let log_builder = &mut Builder::new();
  log_builder.filter_level(LevelFilter::Warn);

  if let Some(filter) = args.log {
    log_builder.filter(None, filter);
  }

  log_builder.init();

  let mut cpu: cpu::Cpu = cpu::Cpu::default();

  if let Some(path) = args.file {
    let file = File::open(path)?;
    cpu.load_file(file)?;
  };
  match args.start_address {
    None => cpu.start(),
    Some(address) => {
      info!("starting from address {:#X}", address);
      cpu.start_from(address)
    }
  };
  info!("exiting successfully");

  Ok(())
}
