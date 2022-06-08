#![allow(clippy::identity_op)] // Ignored due to clippy bug

pub mod addressing_mode;
pub mod error;
pub mod exec;
pub mod operation;
pub mod registers;

use std::io::Read;

use log::info;

use crate::memory;
use operation::Operation;

pub type Cpu = Nes;
pub type Int = u8;

#[derive(Debug, Default)]
pub struct Nes {
  pub register: registers::Nes,
  pub memory: memory::Nes,
  stop: bool,
}

impl Nes {
  /// Reads a set of bytes into the ROM
  ///
  /// # Errors
  /// Forwards any errors encountered while reading the file
  pub fn load_from(&mut self, from: &mut dyn Read) -> anyhow::Result<usize> {
    let result = from.read(&mut self.memory.program_rom)?;
    self
      .memory
      .write_u16(memory::constant::PROGRAM_COUNTER_RESET, 0x8000);

    Ok(result)
  }

  pub fn load(&mut self, program: &[Int]) {
    self.memory.program_rom[..program.len()].copy_from_slice(program);
    self
      .memory
      .write_u16(memory::constant::PROGRAM_COUNTER_RESET, 0x8000);
  }

  pub fn reset(&mut self) {
    self.register = registers::Nes::default();
    self.register.program_counter = self
      .memory
      .read_u16(memory::constant::PROGRAM_COUNTER_RESET);
  }

  /// # Errors
  /// See [`Cpu::resume`]
  pub fn start(&mut self) -> anyhow::Result<()> {
    self.reset();
    self.resume()
  }

  /// # Errors
  /// See [`Cpu::resume`]
  pub fn start_from(&mut self, instructions: memory::Address) -> anyhow::Result<()> {
    self.reset();
    self.register.program_counter = instructions;
    self.resume()
  }

  /// # Errors
  /// Returns any [`error::Error`] that occurs during execution
  pub fn resume(&mut self) -> anyhow::Result<()> {
    loop {
      let operation = self.next_operation();
      self.execute(operation)?;
      self.execute(operation)?;
      if self.stop {
        break;
      }
    }
    info!("execution has stopped");

    Ok(())
  }

  fn next_int(&mut self) -> Int {
    let result = self.memory.read(self.register.program_counter);
    self.register.program_counter += 1;
    result
  }

  fn next_address(&mut self) -> memory::Address {
    let result = self.memory.read_u16(self.register.program_counter);
    self.register.program_counter += 2;
    result
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn load_from() {
    let mut cpu = Cpu::default();

    let size = crate::memory::constant::PROGRAM_ROM_SIZE as usize;
    let values = vec![1; size];

    assert_eq!(size, cpu.load_from(&mut values.as_slice()).unwrap());
  }

  #[test]
  fn load_from_small() {
    let mut cpu = Cpu::default();

    let size = crate::memory::constant::PROGRAM_ROM_SIZE as usize - 1;
    let values = vec![1; size];

    assert_eq!(size, cpu.load_from(&mut values.as_slice()).unwrap());
  }

  #[test]
  fn load_from_too_large() {
    let mut cpu = Cpu::default();

    let rom_size = crate::memory::constant::PROGRAM_ROM_SIZE as usize;
    let values = vec![1; rom_size + 1];

    assert_eq!(rom_size, cpu.load_from(&mut values.as_slice()).unwrap());
  }
}
