use crate::{cpu, memory};

pub trait Value: ValueInner + std::fmt::Debug {}
impl<T: ValueInner + std::fmt::Debug> Value for T {}
pub trait Location: LocationInner + std::fmt::Debug {}
impl<T: LocationInner + std::fmt::Debug> Location for T {}
pub trait Indirection: IndirectionInner + std::fmt::Debug {}
impl<T: IndirectionInner + std::fmt::Debug> Indirection for T {}

pub trait ValueInner {
  fn get(&self, cpu: &cpu::Cpu) -> cpu::Int;
}

pub trait LocationInner {
  fn address(&self, cpu: &cpu::Cpu) -> memory::Address;
}
impl dyn LocationInner {
  pub fn set(&self, cpu: &mut cpu::Cpu, value: cpu::Int) {
    let address = self.address(cpu);
    cpu.memory[address] = value;
  }
}
impl<T: LocationInner> ValueInner for T {
  fn get(&self, cpu: &cpu::Cpu) -> cpu::Int {
    cpu.memory[self.address(cpu)]
  }
}

pub trait IndirectionInner {
  type To: LocationInner;
  fn unwrap(&self, cpu: &cpu::Cpu) -> Self::To;
}
impl<T: IndirectionInner> LocationInner for T {
  fn address(&self, cpu: &cpu::Cpu) -> memory::Address {
    self.unwrap(cpu).address(cpu)
  }
}

/// Literal values
#[derive(Debug)]
pub struct Immediate(pub cpu::Int);
impl ValueInner for Immediate {
  fn get(&self, _: &cpu::Cpu) -> cpu::Int {
    self.0
  }
}

/// Single-byte addressing (allows addresses 0x0000 to 0x00FF)
#[derive(Debug)]
pub struct ZeroPage(pub cpu::Int);
impl LocationInner for ZeroPage {
  fn address(&self, _: &cpu::Cpu) -> memory::Address {
    memory::Address::from(self.0)
  }
}

/// Dual-byte addressing
#[derive(Debug)]
pub struct Absolute(pub memory::Address);
impl LocationInner for Absolute {
  fn address(&self, _: &cpu::Cpu) -> memory::Address {
    self.0
  }
}

/// Value in register X added to offset - wraps at end of [`ZeroPage`] range
#[derive(Debug)]
pub struct XIndexedZeroPage(pub cpu::Int);
impl IndirectionInner for XIndexedZeroPage {
  type To = ZeroPage;
  fn unwrap(&self, cpu: &cpu::Cpu) -> Self::To {
    ZeroPage(self.0 + cpu.register.index_x)
  }
}

/// Value in register X added to offset - wraps at end of [`ZeroPage`] range
///
/// Only used for [`Operation::Ldx`] & [`Operation::Stx`]
#[derive(Debug)]
pub struct YIndexedZeroPage(pub cpu::Int);
impl IndirectionInner for YIndexedZeroPage {
  type To = ZeroPage;
  fn unwrap(&self, cpu: &cpu::Cpu) -> Self::To {
    ZeroPage(self.0 + cpu.register.index_y)
  }
}
/// Value in register X added to address
#[derive(Debug)]
pub struct XIndexedAbsolute(pub memory::Address);
impl IndirectionInner for XIndexedAbsolute {
  type To = Absolute;
  fn unwrap(&self, cpu: &cpu::Cpu) -> Absolute {
    Absolute(self.0 + memory::Address::from(cpu.register.index_x))
  }
}

/// Value in register Y added to address
#[derive(Debug)]
pub struct YIndexedAbsolute(pub memory::Address);
impl IndirectionInner for YIndexedAbsolute {
  type To = Absolute;
  fn unwrap(&self, cpu: &cpu::Cpu) -> Absolute {
    Absolute(self.0 + memory::Address::from(cpu.register.index_y))
  }
}

/// Adds (signed) operand literal to value of program counter
#[derive(Debug)]
pub struct Relative(pub cpu::Int);
impl IndirectionInner for Relative {
  type To = Absolute;
  fn unwrap(&self, cpu: &cpu::Cpu) -> Absolute {
    Absolute(memory::Address::from(self.0) + cpu.register.program_counter)
  }
}

/// Indirect address - only used for JMP
#[derive(Debug)]
pub struct Indirect(pub memory::Address);
impl IndirectionInner for Indirect {
  type To = Absolute;
  fn unwrap(&self, cpu: &cpu::Cpu) -> Absolute {
    Absolute(cpu.memory.read_u16(self.0))
  }
}

/// Value in register X added to address - wraps at end of [`ZeroPage`] (u8) range
#[derive(Debug)]
pub struct XIndexedIndirect(pub cpu::Int);
impl IndirectionInner for XIndexedIndirect {
  type To = Indirect;
  fn unwrap(&self, cpu: &cpu::Cpu) -> Indirect {
    Indirect(memory::Address::from(self.0 + cpu.register.index_x))
  }
}

/// [`ZeroPage`] (single byte) location of address to use as y-indexed absolute address
#[derive(Debug)]
pub struct IndirectYIndexed(pub cpu::Int);
impl IndirectionInner for IndirectYIndexed {
  type To = YIndexedAbsolute;
  fn unwrap(&self, cpu: &cpu::Cpu) -> YIndexedAbsolute {
    let address = cpu.memory.read_u16(memory::Address::from(self.0));
    YIndexedAbsolute(address + memory::Address::from(cpu.register.index_y))
  }
}
