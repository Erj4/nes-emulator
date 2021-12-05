#![feature(exclusive_range_pattern)]
#![feature(fn_traits)]
#![feature(half_open_range_patterns)]
#![feature(unboxed_closures)]
#![warn(clippy::pedantic)]
#![allow(clippy::enum_glob_use)]

use env_logger::Builder;
use log::{debug, LevelFilter};

pub mod cpu;
pub mod memory;

fn main() {
  Builder::new()
    .filter_level(LevelFilter::Debug)
    .parse_env("NES_LOG_LEVEL")
    .init();

  let mut cpu: cpu::Cpu = cpu::Cpu::default();
  cpu.load(&[0x01]);
  cpu.start_from(memory::constant::PROGRAM_ROM_START);
  // cpu.start();
  debug!("Exiting successfully");
}
