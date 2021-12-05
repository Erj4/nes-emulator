use crate::{cpu, memory};

#[derive(Debug)]
/// Standard addressing modes, which are used to retrieve value(s) from memory
pub enum ResolvableAddress {
  /// Single-byte addressing (allows addresses 0x0000 to 0x00FF)
  ///
  /// Resolves to [`Resolvable::Resolved`] value
  ZeroPage(cpu::Int),
  /// Value in register X added to address - wraps at end of ZeroPage (u8) range
  ///
  /// Resolves to [`ResolvableAddress::ZeroPage`]
  XIndexedZeroPage(cpu::Int),
  /// Same as [`ResolvableAddress::IndexedZeroPageX`], but using register Y - only used for LDX & STX
  ///
  /// Resolves to [`ResolvableAddress::ZeroPage`]
  YIndexedZeroPage(cpu::Int),
  /// Dual-byte addressing
  ///
  /// Resolves to [`Resolvable::Resolved`] value
  Absolute(memory::Address),
  /// Value in register X added to address
  ///
  /// Resolves to [`ResolvableAddress::Absolute`]
  XIndexedAbsolute(memory::Address),
  /// Same as [`ResolvableAddress::IndexedAbsoluteX`], but using register Y
  ///
  /// Resolves to [`ResolvableAddress::Absolute`]
  YIndexedAbsolute(memory::Address),
  /// Adds (signed) operand literal to value of program counter
  ///
  /// Resolves to [`Resolvable::Resolved`] **dual byte** value - attempting to resolve to single-byte value will panic!
  Relative(cpu::Int),
  /// Indirect address - only used for JMP
  ///
  /// Resolves to [`ResolvableAddress::Absolute`]
  Indirect(memory::Address),
  /// Value in register X added to address - wraps at end of ZeroPage (u8) range
  ///
  /// Resolves to [`ResolvableAddress::Indirect`]
  XIndexedIndirect(cpu::Int),
  /// ZeroPage (single byte) address of u16 resolvable address to use as y- Indexedaddress
  ///
  /// Resolves to [`ResolvableAddress::YIndexedAbsolute`]
  IndirectYIndexed(cpu::Int),
}

trait SteppableWithCPU {
  type Output;
  fn step(resolvable: ResolvableAddress, cpu: &cpu::Cpu) -> Resolvable<Self::Output>;
  fn resolve(resolvable: ResolvableAddress, cpu: &cpu::Cpu) -> Self::Output {
    let mut resolvable = resolvable;
    loop {
      match Self::step(resolvable, cpu) {
        Resolvable::Resolved(resolved) => break resolved,
        Resolvable::Resolvable(unresolved) => resolvable = unresolved,
      }
    }
  }
}

#[derive(Debug)]
pub enum Resolvable<V> {
  /// Literal value
  Resolved(V),
  Resolvable(ResolvableAddress),
}

impl SteppableWithCPU for Resolvable<cpu::Int> {
  type Output = cpu::Int;

  fn step(resolvable: ResolvableAddress, cpu: &cpu::Cpu) -> Resolvable<Self::Output> {
    ResolvableAddress::step(resolvable, cpu)
  }
}

impl SteppableWithCPU for Resolvable<memory::Address> {
  type Output = memory::Address;

  fn step(resolvable: ResolvableAddress, cpu: &cpu::Cpu) -> Resolvable<Self::Output> {
    ResolvableAddress::step_u16(resolvable, cpu)
  }
}

impl ResolvableAddress {
  fn step_unresolved(self, cpu: &cpu::Cpu) -> ResolvableAddress {
    use ResolvableAddress::*;
    match self {
      XIndexedZeroPage(address) => {
        let address = address + cpu.register.index_x;
        ZeroPage(address)
      }
      YIndexedZeroPage(address) => {
        let address = address + cpu.register.index_y;
        ZeroPage(address)
      }
      XIndexedAbsolute(address) => {
        let address = address + memory::Address::from(cpu.register.index_x);
        Absolute(address)
      }
      YIndexedAbsolute(address) => {
        let address = address + memory::Address::from(cpu.register.index_y);
        Absolute(address)
      }
      Indirect(address) => {
        let absolute_address = cpu.memory.read_u16(address);
        Absolute(absolute_address)
      }
      XIndexedIndirect(address) => {
        let address = memory::Address::from(address + cpu.register.index_x);
        Indirect(address)
      }
      IndirectYIndexed(address) => {
        let address = cpu.memory.read_u16(memory::Address::from(address));
        YIndexedAbsolute(address)
      }
      _ => unreachable!(
        "Addressing mode {:#?} is not implemented as unresolved",
        self
      ),
    }
  }
}

impl ResolvableAddress {
  #[must_use]
  pub fn step(resolvable: ResolvableAddress, cpu: &cpu::Cpu) -> Resolvable<cpu::Int> {
    use ResolvableAddress::*;

    use self::Resolvable::*;

    match resolvable {
      ZeroPage(address) => Resolved(cpu.memory[memory::Address::from(address)]),
      Absolute(address) => Resolved(cpu.memory[address]),
      _ => Resolvable(Self::step_unresolved(resolvable, cpu)),
    }
  }

  #[must_use]
  pub fn step_u16(resolvable: ResolvableAddress, cpu: &cpu::Cpu) -> Resolvable<memory::Address> {
    use ResolvableAddress::*;

    use self::Resolvable::*;
    match resolvable {
      ZeroPage(address) => Resolved(cpu.memory.read_u16(memory::Address::from(address))),
      Absolute(address) => Resolved(cpu.memory.read_u16(address)),
      _ => Resolvable(Self::step_unresolved(resolvable, cpu)),
    }
  }
}

impl Resolvable<cpu::Int> {
  pub fn immediate(cpu: &mut cpu::Cpu) -> Self {
    Self::Resolved(cpu.next_int())
  }
}

impl Resolvable<u8> {
  pub fn accumulator(cpu: &mut cpu::Cpu) -> Self {
    Self::Resolved(cpu.register.accumulator)
  }
}

impl<T> Resolvable<T> {
  pub fn zero_page(cpu: &mut cpu::Cpu) -> Self {
    Self::Resolvable(ResolvableAddress::ZeroPage(cpu.next_int()))
  }

  pub fn x_indexed_zero_page(cpu: &mut cpu::Cpu) -> Self {
    Self::Resolvable(ResolvableAddress::XIndexedZeroPage(cpu.next_int()))
  }

  pub fn y_indexed_zero_page(cpu: &mut cpu::Cpu) -> Self {
    Self::Resolvable(ResolvableAddress::YIndexedZeroPage(cpu.next_int()))
  }

  pub fn absolute(cpu: &mut cpu::Cpu) -> Self {
    Self::Resolvable(ResolvableAddress::Absolute(cpu.next_address()))
  }

  pub fn x_indexed_absolute(cpu: &mut cpu::Cpu) -> Self {
    Self::Resolvable(ResolvableAddress::XIndexedAbsolute(cpu.next_address()))
  }

  pub fn y_indexed_absolute(cpu: &mut cpu::Cpu) -> Self {
    Self::Resolvable(ResolvableAddress::YIndexedAbsolute(cpu.next_address()))
  }

  pub fn relative(cpu: &mut cpu::Cpu) -> Self {
    Self::Resolvable(ResolvableAddress::Relative(cpu.next_int()))
  }

  pub fn indirect(cpu: &mut cpu::Cpu) -> Self {
    Self::Resolvable(ResolvableAddress::Indirect(cpu.next_address()))
  }

  pub fn x_indexed_indirect(cpu: &mut cpu::Cpu) -> Self {
    Self::Resolvable(ResolvableAddress::XIndexedIndirect(cpu.next_int()))
  }

  pub fn indirect_y_indexed(cpu: &mut cpu::Cpu) -> Self {
    Self::Resolvable(ResolvableAddress::IndirectYIndexed(cpu.next_int()))
  }
}
