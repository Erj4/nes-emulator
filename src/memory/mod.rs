use std::{
  fmt,
  ops::{Index, IndexMut},
};

use crate::cpu;

pub mod constant;

#[derive(Debug)]
pub enum Location {
  ProgramRom(Address),
  Ram(Address),
}

pub type Address = u16;

pub struct Nes {
  pub program_rom: [cpu::Int; constant::PROGRAM_ROM_SIZE as usize],
  pub ram: [cpu::Int; constant::RAM_SIZE as usize],
}

impl fmt::Debug for Nes {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.debug_struct("Cpu6502")
      .field("program_rom", &format_args!("{:X?}", &self.program_rom))
      .field("ram", &format_args!("{:X?}", &self.ram))
      .finish()
  }
}

impl Nes {
  #[must_use]
  /// # Panics
  /// This function will panic if it receives an address that is outside of the known memory region ranges
  pub fn resolve_address(address: Address) -> Location {
    use crate::memory::{
      constant::{PROGRAM_ROM_START, RAM_END, RAM_START},
      Location::*,
    };

    match address {
      RAM_START.. if address < RAM_END => Ram(address - RAM_START),
      constant::PROGRAM_ROM_START.. => ProgramRom(address - PROGRAM_ROM_START),
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

impl Default for Nes {
  #[inline]
  fn default() -> Self {
    #[allow(clippy::uninit_assumed_init)] // No guarantees that emulator memory is initialised
    Nes {
      program_rom: [0; constant::PROGRAM_ROM_SIZE as usize],
      ram: [0; constant::RAM_SIZE as usize],
    }
  }
}

impl Index<Address> for Nes {
  type Output = cpu::Int;

  fn index(&self, address: Address) -> &Self::Output {
    use crate::memory::Location::*;

    let location = Self::resolve_address(address);
    match location {
      Ram(ram_address) => &self.ram[ram_address as usize],
      ProgramRom(rom_address) => &self.program_rom[rom_address as usize],
    }
  }
}

impl IndexMut<Address> for Nes {
  fn index_mut(&mut self, address: Address) -> &mut Self::Output {
    use crate::memory::Location::*;

    let location = Self::resolve_address(address);
    match location {
      Ram(ram_address) => &mut self.ram[ram_address as usize],
      ProgramRom(rom_address) => &mut self.program_rom[rom_address as usize],
    }
  }
}
