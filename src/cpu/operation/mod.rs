pub mod parse;

use log::debug;

use self::addressing_mode::Resolvable;
use super::{addressing_mode, memory, Cpu, Int};

#[derive(Debug)]
pub enum Operation {
  /// Add with carry
  Adc(Resolvable<Int>),
  /// Bitwise AND with accumulator
  And(Resolvable<Int>),
  /// Arithmetic shift accumulator left
  ASLAcc,
  /// Arithmetic shift left
  Asl(Resolvable<memory::Address>),
  /// Set flags based on bits
  Bit(Resolvable<Int>),
  /// Branch if plus
  ///
  /// Branch if negative flag clear
  Bpl(Resolvable<Int>),
  /// Branch if minus
  ///
  /// Branch if negative flag set
  Bmi(Resolvable<Int>),
  /// Branch if overflow flag clear
  Bvc(Resolvable<Int>),
  /// Branch if overflow flag set
  Bvs(Resolvable<Int>),
  /// Branch if carry flag clear
  Bcc(Resolvable<Int>),
  /// Branch if carry flag set
  Bcs(Resolvable<Int>),
  /// Branch if not equal
  ///
  /// Branches if zero flag clear
  Bne(Resolvable<Int>),
  /// Branch if equal
  ///
  /// Branch if zero flag set
  Beq(Resolvable<Int>),
  /// Break
  ///
  /// Triggers non-maskable interrupt (NMI).
  Brk,
  /// Compare to accumulator
  Cmp(Resolvable<Int>),
  /// Compare to X register
  Cpx(Resolvable<Int>),
  /// Compare to Y register
  Cpy(Resolvable<Int>),
  /// Decrement memory
  Dec(Resolvable<memory::Address>),
  /// Bitwise exclusive OR (XOR)
  Eor(Resolvable<Int>),
  /// Set carry processor flag
  Sec,
  /// Clear carry processor flag
  Clc,
  /// Set non-mutable interrupt processor flag
  Sei,
  /// Clear non-mutable interrupt processor flag
  Cli,
  /// Clear overflow processor flag
  Clv,
  /// Set decimal mode processor flag (not implemented on NES)
  Set,
  /// Clear decimal mode processor flag (not implemented on NES)
  Cld,
  /// Increment memory
  Inc(Resolvable<memory::Address>),
  /// Jump
  Jmp(Resolvable<memory::Address>),
  /// Jump to SubRoutine
  Jsr(Resolvable<memory::Address>),
  /// Load to accumulator
  Lda(Resolvable<Int>),
  /// Load to X register
  Ldx(Resolvable<Int>),
  /// Load to Y register
  Ldy(Resolvable<Int>),
  /// Logical shift right
  Lsr(Resolvable<Int>),
  /// No-op
  Nop,
  /// Bitwise OR with accumulator
  Ora(Resolvable<Int>),
  /// Transfer A to X
  Tax,
  /// Transfer X to A
  Txa,
  /// Decrement X
  Dex,
  /// Increment X
  Inx,
  /// Transfer A to Y
  Tay,
  /// Transfer Y to A
  Tya,
  /// Decrement Y
  Dey,
  /// Increment Y
  Iny,
  /// Rotate accumulator left
  RolAcc,
  /// Rotate left
  Rol(Resolvable<memory::Address>),
  /// Rotate accumulator left
  RorAcc,
  /// Rotate accumulator right
  Ror(Resolvable<memory::Address>),
  /// Return from interrupt
  Rti,
  /// Return from subroutine
  Rts,
  /// Subtract with carry
  Sbc(Resolvable<Int>),
  // Store accumulator
  Sta(Resolvable<memory::Address>),
  // Store accumulator
  Stx(Resolvable<memory::Address>),
  // Store accumulator
  Sty(Resolvable<memory::Address>),
}

impl Cpu {
  pub fn next_operation(self: &mut Cpu) -> Operation {
    let _opcode: Int = self.next_int();
    Operation::new(self)
  }

  /// # Panics
  /// This function panics if it attempts to execute an operation without a
  pub fn execute(self: &mut Cpu, operation: &Operation) {
    use Operation::*;
    debug!(
      "executing operation {:?} at {:#x}",
      operation, self.register.program_counter
    );
    match operation {
      Brk => self.stop = true,
      _ => unimplemented!("operation {:#?}", operation),
    }
  }
}
