use super::{addressing_mode, Operation};
use crate::cpu::Cpu;

impl Operation {
  /// Get the next operation to execute, moving the program counter forward
  /// # Panics
  /// This function will panic if it receives an opcode that is not defined
  #[allow(clippy::too_many_lines)]
  pub fn next(cpu: &mut Cpu) -> Operation {
    use addressing_mode::Location::*;
    use addressing_mode::{Value, Value::*};
    use Operation::*;

    let opcode = cpu.next_int();
    match opcode {
      // ADC
      0x69 => Adc(Immediate(cpu.next_int())),
      0x65 => Adc(Value::from(ZeroPage(cpu.next_int()))),
      0x75 => Adc(Value::from(XIndexedZeroPage(cpu.next_int()))),
      0x6D => Adc(Value::from(Absolute(cpu.next_address()))),
      0x7D => Adc(Value::from(XIndexedAbsolute(cpu.next_address()))),
      0x79 => Adc(Value::from(YIndexedAbsolute(cpu.next_address()))),
      0x61 => Adc(Value::from(XIndexedIndirect(cpu.next_int()))),
      0x71 => Adc(Value::from(IndirectYIndexed(cpu.next_int()))),
      // AND
      0x29 => And(Immediate(cpu.next_int())),
      0x25 => And(Value::from(ZeroPage(cpu.next_int()))),
      0x35 => And(Value::from(XIndexedZeroPage(cpu.next_int()))),
      0x2D => And(Value::from(Absolute(cpu.next_address()))),
      0x3D => And(Value::from(XIndexedAbsolute(cpu.next_address()))),
      0x39 => And(Value::from(YIndexedAbsolute(cpu.next_address()))),
      0x21 => And(Value::from(XIndexedIndirect(cpu.next_int()))),
      0x31 => And(Value::from(IndirectYIndexed(cpu.next_int()))),
      // ASL
      0x0A => ASLAcc,
      0x06 => Asl(ZeroPage(cpu.next_int())),
      0x16 => Asl(XIndexedZeroPage(cpu.next_int())),
      0x0E => Asl(Absolute(cpu.next_address())),
      0x1E => Asl(XIndexedAbsolute(cpu.next_address())),
      // BIT
      0x24 => Bit(Value::from(ZeroPage(cpu.next_int()))),
      0x2C => Bit(Value::from(Absolute(cpu.next_address()))),
      // Branch
      0x10 => Bpl(Value::from(Relative(cpu.next_int()))),
      0x30 => Bmi(Value::from(Relative(cpu.next_int()))),
      0x50 => Bvc(Value::from(Relative(cpu.next_int()))),
      0x70 => Bvs(Value::from(Relative(cpu.next_int()))),
      0x90 => Bcc(Value::from(Relative(cpu.next_int()))),
      0xB0 => Bcs(Value::from(Relative(cpu.next_int()))),
      0xD0 => Bne(Value::from(Relative(cpu.next_int()))),
      0xF0 => Beq(Value::from(Relative(cpu.next_int()))),
      // BRK
      0x00 => Brk,
      // CMP
      0xC9 => Cmp(Immediate(cpu.next_int())),
      0xC5 => Cmp(Value::from(ZeroPage(cpu.next_int()))),
      0xD5 => Cmp(Value::from(XIndexedZeroPage(cpu.next_int()))),
      0xCD => Cmp(Value::from(Absolute(cpu.next_address()))),
      0xDD => Cmp(Value::from(XIndexedAbsolute(cpu.next_address()))),
      0xD9 => Cmp(Value::from(YIndexedAbsolute(cpu.next_address()))),
      0xC1 => Cmp(Value::from(XIndexedIndirect(cpu.next_int()))),
      0xD1 => Cmp(Value::from(IndirectYIndexed(cpu.next_int()))),
      // CPX
      0xE0 => Cpx(Immediate(cpu.next_int())),
      0xE4 => Cpx(Value::from(ZeroPage(cpu.next_int()))),
      0xEC => Cpx(Value::from(Absolute(cpu.next_address()))),
      // CPY
      0xC0 => Cpy(Immediate(cpu.next_int())),
      0xC4 => Cpy(Value::from(ZeroPage(cpu.next_int()))),
      0xCC => Cpy(Value::from(Absolute(cpu.next_address()))),
      // DEC
      0xC6 => Dec(ZeroPage(cpu.next_int())),
      0xD6 => Dec(XIndexedZeroPage(cpu.next_int())),
      0xCE => Dec(Absolute(cpu.next_address())),
      0xDE => Dec(XIndexedAbsolute(cpu.next_address())),
      // EOR (XOR)
      0x49 => Eor(Immediate(cpu.next_int())),
      0x45 => Eor(Value::from(ZeroPage(cpu.next_int()))),
      0x55 => Eor(Value::from(XIndexedZeroPage(cpu.next_int()))),
      0x4D => Eor(Value::from(Absolute(cpu.next_address()))),
      0x5D => Eor(Value::from(XIndexedAbsolute(cpu.next_address()))),
      0x59 => Eor(Value::from(YIndexedAbsolute(cpu.next_address()))),
      0x41 => Eor(Value::from(XIndexedIndirect(cpu.next_int()))),
      0x51 => Eor(Value::from(IndirectYIndexed(cpu.next_int()))),
      // Processor status flags set
      0x38 => Sec,
      0x78 => Sei,
      0xF8 => Set,
      // Processor status flags clear
      0x18 => Clc,
      0x58 => Cli,
      0xB8 => Clv,
      0xD8 => Cld,
      // INC
      0xE6 => Inc(ZeroPage(cpu.next_int())),
      0xF6 => Inc(XIndexedZeroPage(cpu.next_int())),
      0xEE => Inc(Absolute(cpu.next_address())),
      0xFE => Inc(XIndexedAbsolute(cpu.next_address())),
      // JMP
      0x4C => Jmp(Absolute(cpu.next_address())),
      0x6C => Jmp(Indirect(cpu.next_address())),
      // JSR
      0x20 => Jsr(Absolute(cpu.next_address())),
      // LDA
      0xA9 => Lda(Immediate(cpu.next_int())),
      0xA5 => Lda(Value::from(ZeroPage(cpu.next_int()))),
      0xB5 => Lda(Value::from(XIndexedZeroPage(cpu.next_int()))),
      0xAD => Lda(Value::from(Absolute(cpu.next_address()))),
      0xBD => Lda(Value::from(XIndexedAbsolute(cpu.next_address()))),
      0xB9 => Lda(Value::from(YIndexedAbsolute(cpu.next_address()))),
      0xA1 => Lda(Value::from(XIndexedIndirect(cpu.next_int()))),
      0xB1 => Lda(Value::from(IndirectYIndexed(cpu.next_int()))),
      // LDX
      0xA2 => Ldx(Immediate(cpu.next_int())),
      0xA6 => Ldx(Value::from(ZeroPage(cpu.next_int()))),
      0xB6 => Ldx(Value::from(YIndexedZeroPage(cpu.next_int()))),
      0xAE => Ldx(Value::from(Absolute(cpu.next_address()))),
      0xBE => Ldx(Value::from(YIndexedAbsolute(cpu.next_address()))),
      // LDY
      0xA0 => Ldy(Immediate(cpu.next_int())),
      0xA4 => Ldy(Value::from(ZeroPage(cpu.next_int()))),
      0xB4 => Ldy(Value::from(XIndexedZeroPage(cpu.next_int()))),
      0xAC => Ldy(Value::from(Absolute(cpu.next_address()))),
      0xBC => Ldy(Value::from(XIndexedAbsolute(cpu.next_address()))),
      // LSR
      0x4A => Lsr(Immediate(cpu.register.accumulator)),
      0x46 => Lsr(Value::from(ZeroPage(cpu.next_int()))),
      0x56 => Lsr(Value::from(XIndexedZeroPage(cpu.next_int()))),
      0x4E => Lsr(Value::from(Absolute(cpu.next_address()))),
      0x5E => Lsr(Value::from(XIndexedAbsolute(cpu.next_address()))),
      // NOP
      0xEA => Nop,
      // ORA
      0x09 => Ora(Immediate(cpu.next_int())),
      0x05 => Ora(Value::from(ZeroPage(cpu.next_int()))),
      0x15 => Ora(Value::from(XIndexedZeroPage(cpu.next_int()))),
      0x0D => Ora(Value::from(Absolute(cpu.next_address()))),
      0x1D => Ora(Value::from(XIndexedAbsolute(cpu.next_address()))),
      0x19 => Ora(Value::from(YIndexedAbsolute(cpu.next_address()))),
      0x01 => Ora(Value::from(XIndexedIndirect(cpu.next_int()))),
      0x11 => Ora(Value::from(IndirectYIndexed(cpu.next_int()))),
      // Register X
      0xAA => Tax,
      0x8A => Txa,
      0xCA => Dex,
      0xE8 => Inx,
      // Register Y
      0xA8 => Tay,
      0x98 => Tya,
      0x88 => Dey,
      0xC8 => Iny,
      // ROL
      0x2A => RolAcc,
      0x26 => Rol(ZeroPage(cpu.next_int())),
      0x36 => Rol(XIndexedZeroPage(cpu.next_int())),
      0x2E => Rol(Absolute(cpu.next_address())),
      0x3E => Rol(XIndexedAbsolute(cpu.next_address())),
      // ROR
      0x6A => RorAcc,
      0x66 => Ror(ZeroPage(cpu.next_int())),
      0x76 => Ror(XIndexedZeroPage(cpu.next_int())),
      0x6E => Ror(Absolute(cpu.next_address())),
      0x7E => Ror(XIndexedAbsolute(cpu.next_address())),
      // RTI
      0x40 => Rti,
      // RTS
      0x60 => Rts,
      // SBC
      0xE9 => Sbc(Immediate(cpu.next_int())),
      0xE5 => Sbc(Value::from(ZeroPage(cpu.next_int()))),
      0xF5 => Sbc(Value::from(XIndexedZeroPage(cpu.next_int()))),
      0xED => Sbc(Value::from(Absolute(cpu.next_address()))),
      0xFD => Sbc(Value::from(XIndexedAbsolute(cpu.next_address()))),
      0xF9 => Sbc(Value::from(YIndexedAbsolute(cpu.next_address()))),
      0xE1 => Sbc(Value::from(XIndexedIndirect(cpu.next_int()))),
      0xF1 => Sbc(Value::from(IndirectYIndexed(cpu.next_int()))),
      // STA
      0x85 => Sta(ZeroPage(cpu.next_int())),
      0x95 => Sta(XIndexedZeroPage(cpu.next_int())),
      0x8D => Sta(Absolute(cpu.next_address())),
      0x9D => Sta(XIndexedAbsolute(cpu.next_address())),
      0x99 => Sta(YIndexedAbsolute(cpu.next_address())),
      0x81 => Sta(XIndexedIndirect(cpu.next_int())),
      0x91 => Sta(IndirectYIndexed(cpu.next_int())),
      // STX
      0x86 => Stx(ZeroPage(cpu.next_int())),
      0x96 => Stx(XIndexedZeroPage(cpu.next_int())),
      0x8E => Stx(Absolute(cpu.next_address())),
      // STY
      0x84 => Sty(ZeroPage(cpu.next_int())),
      0x94 => Sty(XIndexedZeroPage(cpu.next_int())),
      0x8C => Sty(Absolute(cpu.next_address())),
      _ => unimplemented!("opcode {:X?}", opcode),
    }
  }
}
