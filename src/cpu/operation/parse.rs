use crate::cpu::{addressing_mode, Cpu, Operation};

impl Cpu {
  /// Get the next operation to execute, moving the program counter forward
  /// # Panics
  /// This function will panic if it receives an opcode that is not defined
  #[allow(clippy::too_many_lines)]
  pub fn next_operation(&mut self) -> Operation {
    use addressing_mode::Location::*;
    use addressing_mode::{Value, Value::*};
    use Operation::*;

    let opcode = self.next_int();
    match opcode {
      // ADC
      0x69 => Adc(Immediate(self.next_int())),
      0x65 => Adc(Value::from(ZeroPage(self.next_int()))),
      0x75 => Adc(Value::from(XIndexedZeroPage(self.next_int()))),
      0x6D => Adc(Value::from(Absolute(self.next_address()))),
      0x7D => Adc(Value::from(XIndexedAbsolute(self.next_address()))),
      0x79 => Adc(Value::from(YIndexedAbsolute(self.next_address()))),
      0x61 => Adc(Value::from(XIndexedIndirect(self.next_int()))),
      0x71 => Adc(Value::from(IndirectYIndexed(self.next_int()))),
      // AND
      0x29 => And(Immediate(self.next_int())),
      0x25 => And(Value::from(ZeroPage(self.next_int()))),
      0x35 => And(Value::from(XIndexedZeroPage(self.next_int()))),
      0x2D => And(Value::from(Absolute(self.next_address()))),
      0x3D => And(Value::from(XIndexedAbsolute(self.next_address()))),
      0x39 => And(Value::from(YIndexedAbsolute(self.next_address()))),
      0x21 => And(Value::from(XIndexedIndirect(self.next_int()))),
      0x31 => And(Value::from(IndirectYIndexed(self.next_int()))),
      // ASL
      0x0A => ASLAcc,
      0x06 => Asl(ZeroPage(self.next_int())),
      0x16 => Asl(XIndexedZeroPage(self.next_int())),
      0x0E => Asl(Absolute(self.next_address())),
      0x1E => Asl(XIndexedAbsolute(self.next_address())),
      // BIT
      0x24 => Bit(Value::from(ZeroPage(self.next_int()))),
      0x2C => Bit(Value::from(Absolute(self.next_address()))),
      // Branch
      0x10 => Bpl(Value::from(Relative(self.next_int()))),
      0x30 => Bmi(Value::from(Relative(self.next_int()))),
      0x50 => Bvc(Value::from(Relative(self.next_int()))),
      0x70 => Bvs(Value::from(Relative(self.next_int()))),
      0x90 => Bcc(Value::from(Relative(self.next_int()))),
      0xB0 => Bcs(Value::from(Relative(self.next_int()))),
      0xD0 => Bne(Value::from(Relative(self.next_int()))),
      0xF0 => Beq(Value::from(Relative(self.next_int()))),
      // BRK
      0x00 => Brk,
      // CMP
      0xC9 => Cmp(Immediate(self.next_int())),
      0xC5 => Cmp(Value::from(ZeroPage(self.next_int()))),
      0xD5 => Cmp(Value::from(XIndexedZeroPage(self.next_int()))),
      0xCD => Cmp(Value::from(Absolute(self.next_address()))),
      0xDD => Cmp(Value::from(XIndexedAbsolute(self.next_address()))),
      0xD9 => Cmp(Value::from(YIndexedAbsolute(self.next_address()))),
      0xC1 => Cmp(Value::from(XIndexedIndirect(self.next_int()))),
      0xD1 => Cmp(Value::from(IndirectYIndexed(self.next_int()))),
      // CPX
      0xE0 => Cpx(Immediate(self.next_int())),
      0xE4 => Cpx(Value::from(ZeroPage(self.next_int()))),
      0xEC => Cpx(Value::from(Absolute(self.next_address()))),
      // CPY
      0xC0 => Cpy(Immediate(self.next_int())),
      0xC4 => Cpy(Value::from(ZeroPage(self.next_int()))),
      0xCC => Cpy(Value::from(Absolute(self.next_address()))),
      // DEC
      0xC6 => Dec(ZeroPage(self.next_int())),
      0xD6 => Dec(XIndexedZeroPage(self.next_int())),
      0xCE => Dec(Absolute(self.next_address())),
      0xDE => Dec(XIndexedAbsolute(self.next_address())),
      // EOR (XOR)
      0x49 => Eor(Immediate(self.next_int())),
      0x45 => Eor(Value::from(ZeroPage(self.next_int()))),
      0x55 => Eor(Value::from(XIndexedZeroPage(self.next_int()))),
      0x4D => Eor(Value::from(Absolute(self.next_address()))),
      0x5D => Eor(Value::from(XIndexedAbsolute(self.next_address()))),
      0x59 => Eor(Value::from(YIndexedAbsolute(self.next_address()))),
      0x41 => Eor(Value::from(XIndexedIndirect(self.next_int()))),
      0x51 => Eor(Value::from(IndirectYIndexed(self.next_int()))),
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
      0xE6 => Inc(ZeroPage(self.next_int())),
      0xF6 => Inc(XIndexedZeroPage(self.next_int())),
      0xEE => Inc(Absolute(self.next_address())),
      0xFE => Inc(XIndexedAbsolute(self.next_address())),
      // JMP
      0x4C => Jmp(Absolute(self.next_address())),
      0x6C => Jmp(Indirect(self.next_address())),
      // JSR
      0x20 => Jsr(Absolute(self.next_address())),
      // LDA
      0xA9 => Lda(Immediate(self.next_int())),
      0xA5 => Lda(Value::from(ZeroPage(self.next_int()))),
      0xB5 => Lda(Value::from(XIndexedZeroPage(self.next_int()))),
      0xAD => Lda(Value::from(Absolute(self.next_address()))),
      0xBD => Lda(Value::from(XIndexedAbsolute(self.next_address()))),
      0xB9 => Lda(Value::from(YIndexedAbsolute(self.next_address()))),
      0xA1 => Lda(Value::from(XIndexedIndirect(self.next_int()))),
      0xB1 => Lda(Value::from(IndirectYIndexed(self.next_int()))),
      // LDX
      0xA2 => Ldx(Immediate(self.next_int())),
      0xA6 => Ldx(Value::from(ZeroPage(self.next_int()))),
      0xB6 => Ldx(Value::from(YIndexedZeroPage(self.next_int()))),
      0xAE => Ldx(Value::from(Absolute(self.next_address()))),
      0xBE => Ldx(Value::from(YIndexedAbsolute(self.next_address()))),
      // LDY
      0xA0 => Ldy(Immediate(self.next_int())),
      0xA4 => Ldy(Value::from(ZeroPage(self.next_int()))),
      0xB4 => Ldy(Value::from(XIndexedZeroPage(self.next_int()))),
      0xAC => Ldy(Value::from(Absolute(self.next_address()))),
      0xBC => Ldy(Value::from(XIndexedAbsolute(self.next_address()))),
      // LSR
      0x4A => Lsr(Immediate(self.register.accumulator)),
      0x46 => Lsr(Value::from(ZeroPage(self.next_int()))),
      0x56 => Lsr(Value::from(XIndexedZeroPage(self.next_int()))),
      0x4E => Lsr(Value::from(Absolute(self.next_address()))),
      0x5E => Lsr(Value::from(XIndexedAbsolute(self.next_address()))),
      // NOP
      0xEA => Nop,
      // ORA
      0x09 => Ora(Immediate(self.next_int())),
      0x05 => Ora(Value::from(ZeroPage(self.next_int()))),
      0x15 => Ora(Value::from(XIndexedZeroPage(self.next_int()))),
      0x0D => Ora(Value::from(Absolute(self.next_address()))),
      0x1D => Ora(Value::from(XIndexedAbsolute(self.next_address()))),
      0x19 => Ora(Value::from(YIndexedAbsolute(self.next_address()))),
      0x01 => Ora(Value::from(XIndexedIndirect(self.next_int()))),
      0x11 => Ora(Value::from(IndirectYIndexed(self.next_int()))),
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
      0x26 => Rol(ZeroPage(self.next_int())),
      0x36 => Rol(XIndexedZeroPage(self.next_int())),
      0x2E => Rol(Absolute(self.next_address())),
      0x3E => Rol(XIndexedAbsolute(self.next_address())),
      // ROR
      0x6A => RorAcc,
      0x66 => Ror(ZeroPage(self.next_int())),
      0x76 => Ror(XIndexedZeroPage(self.next_int())),
      0x6E => Ror(Absolute(self.next_address())),
      0x7E => Ror(XIndexedAbsolute(self.next_address())),
      // RTI
      0x40 => Rti,
      // RTS
      0x60 => Rts,
      // SBC
      0xE9 => Sbc(Immediate(self.next_int())),
      0xE5 => Sbc(Value::from(ZeroPage(self.next_int()))),
      0xF5 => Sbc(Value::from(XIndexedZeroPage(self.next_int()))),
      0xED => Sbc(Value::from(Absolute(self.next_address()))),
      0xFD => Sbc(Value::from(XIndexedAbsolute(self.next_address()))),
      0xF9 => Sbc(Value::from(YIndexedAbsolute(self.next_address()))),
      0xE1 => Sbc(Value::from(XIndexedIndirect(self.next_int()))),
      0xF1 => Sbc(Value::from(IndirectYIndexed(self.next_int()))),
      // STA
      0x85 => Sta(ZeroPage(self.next_int())),
      0x95 => Sta(XIndexedZeroPage(self.next_int())),
      0x8D => Sta(Absolute(self.next_address())),
      0x9D => Sta(XIndexedAbsolute(self.next_address())),
      0x99 => Sta(YIndexedAbsolute(self.next_address())),
      0x81 => Sta(XIndexedIndirect(self.next_int())),
      0x91 => Sta(IndirectYIndexed(self.next_int())),
      // STX
      0x86 => Stx(ZeroPage(self.next_int())),
      0x96 => Stx(XIndexedZeroPage(self.next_int())),
      0x8E => Stx(Absolute(self.next_address())),
      // STY
      0x84 => Sty(ZeroPage(self.next_int())),
      0x94 => Sty(XIndexedZeroPage(self.next_int())),
      0x8C => Sty(Absolute(self.next_address())),
      _ => unimplemented!("opcode {:X?}", opcode),
    }
  }
}
