use crate::{cpu, memory};

#[derive(Debug, Default)]
pub struct Nes {
  pub program_counter: memory::Address,
  pub stack_pointer: cpu::Int,
  pub accumulator: cpu::Int,
  pub index_x: cpu::Int,
  pub index_y: cpu::Int,
  pub status: StatusRegister,
}

#[derive(Debug, Default)]
/// CPU status flags
///
/// WARNING: some flags (especially N & Z) may be affected by commands in ways that do not follow their defined purpose.
///
#[must_use]
pub struct StatusRegister {
  pub result_status: ResultStatus,
  pub interrupt_status: InterruptStatus,
  /// Whether arithmetic should treat values as binary-coded decimal rather than binary values
  ///
  /// Decimal mode is not supported on NES chips.
  pub decimal_mode: NumberMode,
}

#[derive(Debug, Default)]
pub struct InterruptStatus {
  /// Whether *maskable* interrupts should be disabled
  ///
  /// It can be explicitly set using the 'Set Interrupt Disable' (SEI) instruction and cleared with 'Clear Interrupt Disable' (CLI).
  pub enabled: bool,
  /// Whether a BRK instruction has been executed and an interrupt has been generated to process it
  pub break_command: bool,
}

#[derive(Debug, Default)]
pub struct ResultStatus {
  /// Whether result of last operation was zero
  pub zero: bool,
  /// Whether result of last operation was "negative"
  ///
  /// This flag is true if the most-significant bit of the result was one.
  pub negative: bool,
  /// Whether last operation caused an *unsigned* integer overflow
  ///
  /// This condition is set during arithmetic, comparison and during logical shifts.
  /// It can be explicitly set using the 'Set Carry Flag' (SEC) instruction and cleared with 'Clear Carry Flag' (CLC).
  pub carry: Carry,
  /// Whether last operation caused a *signed* integer overflow
  ///
  ///
  /// For example 0x40 (64) + 0x40 (64) = 0x80 (-128) => `overflow=true`.
  ///
  /// Cleared with 'Clear Overflow' (CLV) instruction.
  /// Only affected by ADC, BIT, CLV, PLP, RTI, and SBC instructions.
  pub overflow: Overflow,
}
#[derive(Debug)]
pub enum Carry {
  NoOverflow,
  UnsignedOverflow,
}
impl Default for Carry {
  fn default() -> Self {
    Self::NoOverflow
  }
}

#[derive(Debug)]
pub enum NumberMode {
  Binary,
  BinaryCodedDecimal,
}
impl Default for NumberMode {
  fn default() -> Self {
    Self::Binary
  }
}

#[derive(Debug)]
pub enum Overflow {
  NoOverflow,
  UnsignedOverflow,
}
impl Default for Overflow {
  fn default() -> Self {
    Self::NoOverflow
  }
}
