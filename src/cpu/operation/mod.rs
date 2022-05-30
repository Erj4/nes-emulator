pub mod parse;

use super::addressing_mode::{Location, Value};

#[derive(Debug)]
pub enum Operation {
  /// Add with carry
  Adc(Box<dyn Value>),
  /// Bitwise AND with accumulator
  And(Box<dyn Value>),
  /// Arithmetic shift accumulator left
  ASLAcc,
  /// Arithmetic shift left
  Asl(Box<dyn Location>),
  /// Set flags based on bits
  Bit(Box<dyn Value>),
  /// Branch if plus
  ///
  /// Branch if negative flag clear
  Bpl(Box<dyn Value>),
  /// Branch if minus
  ///
  /// Branch if negative flag set
  Bmi(Box<dyn Value>),
  /// Branch if overflow flag clear
  Bvc(Box<dyn Value>),
  /// Branch if overflow flag set
  Bvs(Box<dyn Value>),
  /// Branch if carry flag clear
  Bcc(Box<dyn Value>),
  /// Branch if carry flag set
  Bcs(Box<dyn Value>),
  /// Branch if not equal
  ///
  /// Branches if zero flag clear
  Bne(Box<dyn Value>),
  /// Branch if equal
  ///
  /// Branch if zero flag set
  Beq(Box<dyn Value>),
  /// Break
  ///
  /// Triggers non-maskable interrupt (NMI).
  Brk,
  /// Compare to accumulator
  Cmp(Box<dyn Value>),
  /// Compare to X register
  Cpx(Box<dyn Value>),
  /// Compare to Y register
  Cpy(Box<dyn Value>),
  /// Decrement memory
  Dec(Box<dyn Location>),
  /// Bitwise exclusive OR (XOR)
  Eor(Box<dyn Value>),
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
  Inc(Box<dyn Location>),
  /// Jump
  Jmp(Box<dyn Location>),
  /// Jump to SubRoutine
  Jsr(Box<dyn Location>),
  /// Load to accumulator
  Lda(Box<dyn Value>),
  /// Load to X register
  Ldx(Box<dyn Value>),
  /// Load to Y register
  Ldy(Box<dyn Value>),
  /// Logical shift right
  Lsr(Box<dyn Value>),
  /// No-op
  Nop,
  /// Bitwise OR with accumulator
  Ora(Box<dyn Value>),
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
  Rol(Box<dyn Location>),
  /// Rotate accumulator left
  RorAcc,
  /// Rotate accumulator right
  Ror(Box<dyn Location>),
  /// Return from interrupt
  Rti,
  /// Return from subroutine
  Rts,
  /// Subtract with carry
  Sbc(Box<dyn Value>),
  // Store accumulator
  Sta(Box<dyn Location>),
  // Store accumulator
  Stx(Box<dyn Location>),
  // Store accumulator
  Sty(Box<dyn Location>),
}
