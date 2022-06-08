use crate::{cpu, memory};

// pub trait Value: ValueInner + std::fmt::Debug + Clone + Copy + Send + Sync {}
// impl<T: ValueInner + std::fmt::Debug + Clone + Copy + Send + Sync> Value for T {}
// pub trait Location: LocationInner + std::fmt::Debug + Clone + Copy + Send + Sync {}
// impl<T: LocationInner + std::fmt::Debug + Clone + Copy + Send + Sync> Location for T {}
// pub trait Indirection: IndirectionInner + std::fmt::Debug + Clone + Copy + Send + Sync {}
// impl<T: IndirectionInner + std::fmt::Debug + Clone + Copy + Send + Sync> Indirection for T {}

#[derive(Clone, Copy, Debug)]
pub enum Value {
  /// Literal values
  Immediate(cpu::Int),
  Location(Location),
}
impl Value {
  #[must_use]
  pub fn value(self, cpu: &cpu::Cpu) -> cpu::Int {
    use Value::*;
    match self {
      Immediate(value) => value,
      Location(at) => cpu.memory.read(at.location(cpu)),
    }
  }
}

impl From<Location> for Value {
  fn from(at: Location) -> Self {
    Value::Location(at)
  }
}

#[derive(Clone, Copy, Debug)]
pub enum Location {
  ZeroPage(cpu::Int),
  Absolute(memory::Address),
  XIndexedZeroPage(cpu::Int),
  YIndexedZeroPage(cpu::Int),
  XIndexedAbsolute(memory::Address),
  YIndexedAbsolute(memory::Address),
  Relative(cpu::Int),
  Indirect(memory::Address),
  XIndexedIndirect(cpu::Int),
  IndirectYIndexed(cpu::Int),
}

impl Location {
  #[must_use]
  pub fn location(self, cpu: &cpu::Cpu) -> memory::Address {
    use cpu::Int;
    use memory::Address;
    use Location::*;
    match self {
      ZeroPage(addr) => Address::from(addr),
      Absolute(addr) => addr,
      XIndexedZeroPage(addr) => Address::from(Int::wrapping_add(addr, cpu.register.index_x)),
      YIndexedZeroPage(addr) => Address::from(Int::wrapping_add(addr, cpu.register.index_y)),
      XIndexedAbsolute(addr) => addr + Address::from(cpu.register.index_x),
      YIndexedAbsolute(addr) => addr + Address::from(cpu.register.index_y),
      Relative(addr) => Address::wrapping_add(Address::from(addr), cpu.register.program_counter),
      Indirect(addr) => cpu.memory.read_u16(addr),
      XIndexedIndirect(addr) => {
        let addr = Int::wrapping_add(addr, cpu.register.index_x);
        cpu.memory.read_u16(Address::from(addr))
      }
      IndirectYIndexed(addr) => {
        let addr = cpu.memory.read_u16(Address::from(addr));
        Address::wrapping_add(addr, Address::from(cpu.register.index_y))
      }
    }
  }
}
