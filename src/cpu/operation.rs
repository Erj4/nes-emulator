use log::debug;

use self::addressing_mode::Resolvable;
use crate::cpu::{addressing_mode, memory, Cpu, Int};

#[derive(Debug)]
pub enum Operation {
  /// Add with carry
  ADC(Resolvable<Int>),
  /// Bitwise AND with accumulator
  AND(Resolvable<Int>),
  /// Arithmetic shift accumulator left
  ASLAcc,
  /// Arithmetic shift left
  ASL(Resolvable<memory::Address>),
  /// Set flags based on bits
  BIT(Resolvable<Int>),
  /// Branch if plus
  ///
  /// Branch if negative flag clear
  BPL(Resolvable<Int>),
  /// Branch if minus
  ///
  /// Branch if negative flag set
  BMI(Resolvable<Int>),
  /// Branch if overflow flag clear
  BVC(Resolvable<Int>),
  /// Branch if overflow flag set
  BVS(Resolvable<Int>),
  /// Branch if carry flag clear
  BCC(Resolvable<Int>),
  /// Branch if carry flag set
  BCS(Resolvable<Int>),
  /// Branch if not equal
  ///
  /// Branches if zero flag clear
  BNE(Resolvable<Int>),
  /// Branch if equal
  ///
  /// Branch if zero flag set
  BEQ(Resolvable<Int>),
  /// Break
  ///
  /// Triggers non-maskable interrupt (NMI).
  BRK,
  /// Compare to accumulator
  CMP(Resolvable<Int>),
  /// Compare to X register
  CPX(Resolvable<Int>),
  /// Compare to Y register
  CPY(Resolvable<Int>),
  /// Decrement memory
  DEC(Resolvable<memory::Address>),
  /// Bitwise exclusive OR (XOR)
  EOR(Resolvable<Int>),
  /// Set carry processor flag
  SEC,
  /// Clear carry processor flag
  CLC,
  /// Set non-mutable interrupt processor flag
  SEI,
  /// Clear non-mutable interrupt processor flag
  CLI,
  /// Clear overflow processor flag
  CLV,
  /// Set decimal mode processor flag (not implemented on NES)
  SED,
  /// Clear decimal mode processor flag (not implemented on NES)
  CLD,
  /// Increment memory
  INC(Resolvable<memory::Address>),
  /// Jump
  JMP(Resolvable<memory::Address>),
  /// Jump to SubRoutine
  JSR(Resolvable<memory::Address>),
  /// Load to accumulator
  LDA(Resolvable<Int>),
  /// Load to X register
  LDX(Resolvable<Int>),
  /// Load to Y register
  LDY(Resolvable<Int>),
  /// Logical shift right
  LSR(Resolvable<Int>),
  /// No-op
  NOP,
  /// Bitwise OR with accumulator
  ORA(Resolvable<Int>),
  /// Transfer A to X
  TAX,
  /// Transfer X to A
  TXA,
  /// Decrement X
  DEX,
  /// Increment X
  INX,
  /// Transfer A to Y
  TAY,
  /// Transfer Y to A
  TYA,
  /// Decrement Y
  DEY,
  /// Increment Y
  INY,
  /// Rotate accumulator left
  ROLAcc,
  /// Rotate left
  ROL(Resolvable<memory::Address>),
  /// Rotate accumulator left
  RORAcc,
  /// Rotate right
  ROR(Resolvable<memory::Address>),
  /// Return from interrupt
  RTI,
  /// Return from subroutine
  RTS,
  /// Subtract with carry
  SBC(Resolvable<Int>),
  // Store accumulator
  STA(Resolvable<memory::Address>),
  // Store accumulator
  STX(Resolvable<memory::Address>),
  // Store accumulator
  STY(Resolvable<memory::Address>),
}

impl Operation {
  #[allow(clippy::too_many_lines)]
  fn new(opcode: Int, cpu: &mut Cpu) -> Operation {
    use self::{addressing_mode::Resolvable as Addr, Operation::*};
    match opcode {
      // ADC
      0x69 => ADC(Addr::immediate(cpu)),
      0x65 => ADC(Addr::zero_page(cpu)),
      0x75 => ADC(Addr::x_indexed_zero_page(cpu)),
      0x6D => ADC(Addr::absolute(cpu)),
      0x7D => ADC(Addr::x_indexed_absolute(cpu)),
      0x79 => ADC(Addr::y_indexed_absolute(cpu)),
      0x61 => ADC(Addr::x_indexed_indirect(cpu)),
      0x71 => ADC(Addr::indirect_y_indexed(cpu)),
      // AND
      0x29 => AND(Addr::immediate(cpu)),
      0x25 => AND(Addr::zero_page(cpu)),
      0x35 => AND(Addr::x_indexed_zero_page(cpu)),
      0x2D => AND(Addr::absolute(cpu)),
      0x3D => AND(Addr::x_indexed_absolute(cpu)),
      0x39 => AND(Addr::y_indexed_absolute(cpu)),
      0x21 => AND(Addr::x_indexed_indirect(cpu)),
      0x31 => AND(Addr::indirect_y_indexed(cpu)),
      // ASL
      0x0A => ASLAcc,
      0x06 => ASL(Addr::zero_page(cpu)),
      0x16 => ASL(Addr::x_indexed_zero_page(cpu)),
      0x0E => ASL(Addr::absolute(cpu)),
      0x1E => ASL(Addr::x_indexed_absolute(cpu)),
      // BIT
      0x24 => BIT(Addr::zero_page(cpu)),
      0x2C => BIT(Addr::absolute(cpu)),
      // Branch
      0x10 => BPL(Addr::relative(cpu)),
      0x30 => BMI(Addr::relative(cpu)),
      0x50 => BVC(Addr::relative(cpu)),
      0x70 => BVS(Addr::relative(cpu)),
      0x90 => BCC(Addr::relative(cpu)),
      0xB0 => BCS(Addr::relative(cpu)),
      0xD0 => BNE(Addr::relative(cpu)),
      0xF0 => BEQ(Addr::relative(cpu)),
      // BRK
      0x00 => BRK,
      // CMP
      0xC9 => CMP(Addr::immediate(cpu)),
      0xC5 => CMP(Addr::zero_page(cpu)),
      0xD5 => CMP(Addr::x_indexed_zero_page(cpu)),
      0xCD => CMP(Addr::absolute(cpu)),
      0xDD => CMP(Addr::x_indexed_absolute(cpu)),
      0xD9 => CMP(Addr::y_indexed_absolute(cpu)),
      0xC1 => CMP(Addr::x_indexed_indirect(cpu)),
      0xD1 => CMP(Addr::indirect_y_indexed(cpu)),
      // CPX
      0xE0 => CPX(Addr::immediate(cpu)),
      0xE4 => CPX(Addr::zero_page(cpu)),
      0xEC => CPX(Addr::absolute(cpu)),
      // CPY
      0xC0 => CPY(Addr::immediate(cpu)),
      0xC4 => CPY(Addr::zero_page(cpu)),
      0xCC => CPY(Addr::absolute(cpu)),
      // DEC
      0xC6 => DEC(Addr::zero_page(cpu)),
      0xD6 => DEC(Addr::x_indexed_zero_page(cpu)),
      0xCE => DEC(Addr::absolute(cpu)),
      0xDE => DEC(Addr::x_indexed_absolute(cpu)),
      // EOR (XOR)
      0x49 => EOR(Addr::immediate(cpu)),
      0x45 => EOR(Addr::zero_page(cpu)),
      0x55 => EOR(Addr::x_indexed_zero_page(cpu)),
      0x4D => EOR(Addr::absolute(cpu)),
      0x5D => EOR(Addr::x_indexed_absolute(cpu)),
      0x59 => EOR(Addr::y_indexed_absolute(cpu)),
      0x41 => EOR(Addr::x_indexed_indirect(cpu)),
      0x51 => EOR(Addr::indirect_y_indexed(cpu)),
      // Processor status flags set
      0x38 => SEC,
      0x78 => SEI,
      0xF8 => SED,
      // Processor status flags clear
      0x18 => CLC,
      0x58 => CLI,
      0xB8 => CLV,
      0xD8 => CLD,
      // INC
      0xE6 => INC(Addr::zero_page(cpu)),
      0xF6 => INC(Addr::x_indexed_zero_page(cpu)),
      0xEE => INC(Addr::absolute(cpu)),
      0xFE => INC(Addr::x_indexed_absolute(cpu)),
      // JMP
      0x4C => JMP(Addr::absolute(cpu)),
      0x6C => JMP(Addr::indirect(cpu)),
      // JSR
      0x20 => JSR(Addr::absolute(cpu)),
      // LDA
      0xA9 => LDA(Addr::immediate(cpu)),
      0xA5 => LDA(Addr::zero_page(cpu)),
      0xB5 => LDA(Addr::x_indexed_zero_page(cpu)),
      0xAD => LDA(Addr::absolute(cpu)),
      0xBD => LDA(Addr::x_indexed_absolute(cpu)),
      0xB9 => LDA(Addr::y_indexed_absolute(cpu)),
      0xA1 => LDA(Addr::x_indexed_indirect(cpu)),
      0xB1 => LDA(Addr::indirect_y_indexed(cpu)),
      // LDX
      0xA2 => LDX(Addr::immediate(cpu)),
      0xA6 => LDX(Addr::zero_page(cpu)),
      0xB6 => LDX(Addr::y_indexed_zero_page(cpu)),
      0xAE => LDX(Addr::absolute(cpu)),
      0xBE => LDX(Addr::y_indexed_absolute(cpu)),
      // LDY
      0xA0 => LDY(Addr::immediate(cpu)),
      0xA4 => LDY(Addr::zero_page(cpu)),
      0xB4 => LDY(Addr::x_indexed_zero_page(cpu)),
      0xAC => LDY(Addr::absolute(cpu)),
      0xBC => LDY(Addr::x_indexed_absolute(cpu)),
      // LSR
      0x4A => LSR(Addr::accumulator(cpu)),
      0x46 => LSR(Addr::zero_page(cpu)),
      0x56 => LSR(Addr::x_indexed_zero_page(cpu)),
      0x4E => LSR(Addr::absolute(cpu)),
      0x5E => LSR(Addr::x_indexed_absolute(cpu)),
      // NOP
      0xEA => NOP,
      // ORA
      0x09 => ORA(Addr::immediate(cpu)),
      0x05 => ORA(Addr::zero_page(cpu)),
      0x15 => ORA(Addr::x_indexed_zero_page(cpu)),
      0x0D => ORA(Addr::absolute(cpu)),
      0x1D => ORA(Addr::x_indexed_absolute(cpu)),
      0x19 => ORA(Addr::y_indexed_absolute(cpu)),
      0x01 => ORA(Addr::x_indexed_indirect(cpu)),
      0x11 => ORA(Addr::indirect_y_indexed(cpu)),
      // Register X
      0xAA => TAX,
      0x8A => TXA,
      0xCA => DEX,
      0xE8 => INX,
      // Register Y
      0xA8 => TAY,
      0x98 => TYA,
      0x88 => DEY,
      0xC8 => INY,
      // ROL
      0x2A => ROLAcc,
      0x26 => ROL(Addr::zero_page(cpu)),
      0x36 => ROL(Addr::x_indexed_zero_page(cpu)),
      0x2E => ROL(Addr::absolute(cpu)),
      0x3E => ROL(Addr::x_indexed_absolute(cpu)),
      // ROR
      0x6A => RORAcc,
      0x66 => ROR(Addr::zero_page(cpu)),
      0x76 => ROR(Addr::x_indexed_zero_page(cpu)),
      0x6E => ROR(Addr::absolute(cpu)),
      0x7E => ROR(Addr::x_indexed_absolute(cpu)),
      // RTI
      0x40 => RTI,
      // RTS
      0x60 => RTS,
      // SBC
      0xE9 => SBC(Addr::immediate(cpu)),
      0xE5 => SBC(Addr::zero_page(cpu)),
      0xF5 => SBC(Addr::x_indexed_zero_page(cpu)),
      0xED => SBC(Addr::absolute(cpu)),
      0xFD => SBC(Addr::x_indexed_absolute(cpu)),
      0xF9 => SBC(Addr::y_indexed_absolute(cpu)),
      0xE1 => SBC(Addr::x_indexed_indirect(cpu)),
      0xF1 => SBC(Addr::indirect_y_indexed(cpu)),
      // STA
      0x85 => STA(Addr::zero_page(cpu)),
      0x95 => STA(Addr::x_indexed_zero_page(cpu)),
      0x8D => STA(Addr::absolute(cpu)),
      0x9D => STA(Addr::x_indexed_absolute(cpu)),
      0x99 => STA(Addr::y_indexed_absolute(cpu)),
      0x81 => STA(Addr::x_indexed_indirect(cpu)),
      0x91 => STA(Addr::indirect_y_indexed(cpu)),
      // STX
      0x86 => STX(Addr::zero_page(cpu)),
      0x96 => STX(Addr::x_indexed_zero_page(cpu)),
      0x8E => STX(Addr::absolute(cpu)),
      // STY
      0x84 => STY(Addr::zero_page(cpu)),
      0x94 => STY(Addr::x_indexed_zero_page(cpu)),
      0x8C => STY(Addr::absolute(cpu)),
      _ => unimplemented!("opcode {:X?}", opcode),
    }
  }
}

impl Cpu {
  pub fn next_operation(self: &mut Cpu) -> Operation {
    let opcode: Int = self.memory.read(self.register.program_counter);
    self.register.program_counter += 1;
    Operation::new(opcode, self)
  }

  /// # Panics
  pub fn execute(self: &mut Cpu, operation: &Operation) {
    use self::Operation::*;
    debug!(
      "executing operation {:?} at {:#x}",
      operation, self.register.program_counter
    );
    match operation {
      BRK => self.stop = true,
      _ => unimplemented!("operation {:#?}", operation),
    }
  }
}
