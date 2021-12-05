#![allow(clippy::identity_op)] // Ignored due to clippy bug

use log::info;

pub mod addressing_mode;
pub mod operation;
pub mod registers;

use crate::memory;

pub type Cpu = NES;
pub type Int = u8;

#[derive(Debug, Default)]
pub struct NES {
  pub register: registers::NES,
  pub memory:   memory::NES,
  stop:         bool,
}

impl NES {
  pub fn load(&mut self, program: &[Int]) {
    self.memory.program_rom[.. program.len()].copy_from_slice(program);
    self
      .memory
      .write_u16(memory::constant::PROGRAM_COUNTER_RESET, 0x8000);
  }

  pub fn reset(&mut self) {
    self.register = registers::NES::default();
    self.register.program_counter = self
      .memory
      .read_u16(memory::constant::PROGRAM_COUNTER_RESET);
  }

  pub fn start(&mut self) {
    self.reset();
    self.resume()
  }

  pub fn start_from(&mut self, instructions: memory::Address) {
    self.reset();
    self.register.program_counter = instructions;
    self.resume()
  }

  pub fn resume(&mut self) {
    loop {
      let operation = self.next_operation();
      self.execute(&operation);
      if self.stop {
        break;
      }
      info!("Execution has stopped")
    }
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
