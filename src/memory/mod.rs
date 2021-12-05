use std::{
  fmt,
  mem::MaybeUninit,
  ops::{Index, IndexMut},
};

pub mod constant;

use crate::cpu;

#[derive(Debug)]
pub enum Location {
  ProgramRom(Address),
  Ram(Address),
}

pub type Address = u16;

pub struct NES {
  pub program_rom: [cpu::Int; constant::PROGRAM_ROM_SIZE as usize],
  pub ram:         [cpu::Int; constant::RAM_SIZE as usize],
}

impl fmt::Debug for NES {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.debug_struct("Cpu6502")
      .field("program_rom", &format_args!("{:X?}", &self.program_rom))
      .field("ram", &format_args!("{:X?}", &self.ram))
      .finish()
  }
}

impl NES {
  #[must_use]
  /// # Panics
  pub fn resolve_address(address: Address) -> Location {
    use constant::{PROGRAM_ROM_START, RAM_END, RAM_START};
    use Location::*;

    match address {
      RAM_START .. RAM_END => Ram(address - RAM_START),
      constant::PROGRAM_ROM_START .. => ProgramRom(address - PROGRAM_ROM_START),
      _ => todo!("Memory location {:#4X}", address),
    }
  }

  #[must_use]
  pub fn read(&self, addr: u16) -> cpu::Int {
    self[addr]
  }

  pub fn write(&mut self, addr: u16, data: cpu::Int) {
    self[addr] = data;
  }

  #[must_use]
  pub fn read_u16(&self, addr: u16) -> u16 {
    u16::from_le_bytes([self[addr], self[addr + 1]])
  }

  pub fn write_u16(&mut self, addr: u16, data: u16) {
    let [fst, snd] = u16::to_le_bytes(data);
    self[addr] = fst;
    self[addr] = snd;
  }
}

impl Default for NES {
  #[inline]
  fn default() -> Self {
    #[allow(clippy::uninit_assumed_init)] // No guarantees that emulator memory is initialised
    Self {
      program_rom: unsafe { MaybeUninit::uninit().assume_init() },
      ram:         unsafe { MaybeUninit::uninit().assume_init() },
    }
  }
}

impl Index<Address> for NES {
  type Output = cpu::Int;

  fn index(&self, address: Address) -> &Self::Output {
    use Location::*;

    let location = Self::resolve_address(address);
    match location {
      Ram(address) => &self.ram[address as usize],
      ProgramRom(address) => &self.program_rom[address as usize],
    }
  }
}

impl IndexMut<Address> for NES {
  fn index_mut(&mut self, address: Address) -> &mut Self::Output {
    use Location::*;

    let location = Self::resolve_address(address);
    match location {
      Ram(address) => &mut self.ram[address as usize],
      ProgramRom(address) => &mut self.program_rom[address as usize],
    }
  }
}