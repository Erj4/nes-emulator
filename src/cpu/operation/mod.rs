pub mod parse;

use super::addressing_mode::{Location, Value};
use strum::Display;

#[derive(Clone, Copy, Debug, Display)]
pub enum Operation {
  /// Add with carry
  Adc(Value),
  /// Bitwise AND with accumulator
  And(Value),
  /// Arithmetic shift accumulator left
  ASLAcc,
  /// Arithmetic shift left
  Asl(Location),
  /// Set flags based on bits
  Bit(Value),
  /// Branch if plus
  ///
  /// Branch if negative flag clear
  Bpl(Value),
  /// Branch if minus
  ///
  /// Branch if negative flag set
  Bmi(Value),
  /// Branch if overflow flag clear
  Bvc(Value),
  /// Branch if overflow flag set
  Bvs(Value),
  /// Branch if carry flag clear
  Bcc(Value),
  /// Branch if carry flag set
  Bcs(Value),
  /// Branch if not equal
  ///
  /// Branches if zero flag clear
  Bne(Value),
  /// Branch if equal
  ///
  /// Branch if zero flag set
  Beq(Value),
  /// Break
  ///
  /// Triggers non-maskable interrupt (NMI).
  Brk,
  /// Compare to accumulator
  Cmp(Value),
  /// Compare to X register
  Cpx(Value),
  /// Compare to Y register
  Cpy(Value),
  /// Decrement memory
  Dec(Location),
  /// Bitwise exclusive OR (XOR)
  Eor(Value),
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
  Inc(Location),
  /// Jump
  Jmp(Location),
  /// Jump to SubRoutine
  Jsr(Location),
  /// Load to accumulator
  Lda(Value),
  /// Load to X register
  Ldx(Value),
  /// Load to Y register
  Ldy(Value),
  /// Logical shift right
  Lsr(Value),
  /// No-op
  Nop,
  /// Bitwise OR with accumulator
  Ora(Value),
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
  Rol(Location),
  /// Rotate accumulator left
  RorAcc,
  /// Rotate accumulator right
  Ror(Location),
  /// Return from interrupt
  Rti,
  /// Return from subroutine
  Rts,
  /// Subtract with carry
  Sbc(Value),
  // Store accumulator
  Sta(Location),
  // Store accumulator
  Stx(Location),
  // Store accumulator
  Sty(Location),
}
