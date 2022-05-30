use crate::cpu::{addressing_mode, Cpu, Operation};

impl Cpu {
  /// Get the next operation to execute, moving the program counter forward
  /// # Panics
  /// This function will panic if it receives an opcode that is not defined
  #[allow(clippy::too_many_lines)]
  pub fn next_operation(&mut self) -> Operation {
    use Operation::*;

    let opcode = self.next_int();
    match opcode {
      // ADC
      0x69 => Adc(Box::new(addressing_mode::Immediate(self.next_int()))),
      0x65 => Adc(Box::new(addressing_mode::ZeroPage(self.next_int()))),
      0x75 => Adc(Box::new(addressing_mode::XIndexedZeroPage(self.next_int()))),
      0x6D => Adc(Box::new(addressing_mode::Absolute(self.next_address()))),
      0x7D => Adc(Box::new(addressing_mode::XIndexedAbsolute(
        self.next_address(),
      ))),
      0x79 => Adc(Box::new(addressing_mode::YIndexedAbsolute(
        self.next_address(),
      ))),
      0x61 => Adc(Box::new(addressing_mode::XIndexedIndirect(self.next_int()))),
      0x71 => Adc(Box::new(addressing_mode::IndirectYIndexed(self.next_int()))),
      // AND
      0x29 => And(Box::new(addressing_mode::Immediate(self.next_int()))),
      0x25 => And(Box::new(addressing_mode::ZeroPage(self.next_int()))),
      0x35 => And(Box::new(addressing_mode::XIndexedZeroPage(self.next_int()))),
      0x2D => And(Box::new(addressing_mode::Absolute(self.next_address()))),
      0x3D => And(Box::new(addressing_mode::XIndexedAbsolute(
        self.next_address(),
      ))),
      0x39 => And(Box::new(addressing_mode::YIndexedAbsolute(
        self.next_address(),
      ))),
      0x21 => And(Box::new(addressing_mode::XIndexedIndirect(self.next_int()))),
      0x31 => And(Box::new(addressing_mode::IndirectYIndexed(self.next_int()))),
      // ASL
      0x0A => ASLAcc,
      0x06 => Asl(Box::new(addressing_mode::ZeroPage(self.next_int()))),
      0x16 => Asl(Box::new(addressing_mode::XIndexedZeroPage(self.next_int()))),
      0x0E => Asl(Box::new(addressing_mode::Absolute(self.next_address()))),
      0x1E => Asl(Box::new(addressing_mode::XIndexedAbsolute(
        self.next_address(),
      ))),
      // BIT
      0x24 => Bit(Box::new(addressing_mode::ZeroPage(self.next_int()))),
      0x2C => Bit(Box::new(addressing_mode::Absolute(self.next_address()))),
      // Branch
      0x10 => Bpl(Box::new(addressing_mode::Relative(self.next_int()))),
      0x30 => Bmi(Box::new(addressing_mode::Relative(self.next_int()))),
      0x50 => Bvc(Box::new(addressing_mode::Relative(self.next_int()))),
      0x70 => Bvs(Box::new(addressing_mode::Relative(self.next_int()))),
      0x90 => Bcc(Box::new(addressing_mode::Relative(self.next_int()))),
      0xB0 => Bcs(Box::new(addressing_mode::Relative(self.next_int()))),
      0xD0 => Bne(Box::new(addressing_mode::Relative(self.next_int()))),
      0xF0 => Beq(Box::new(addressing_mode::Relative(self.next_int()))),
      // BRK
      0x00 => Brk,
      // CMP
      0xC9 => Cmp(Box::new(addressing_mode::Immediate(self.next_int()))),
      0xC5 => Cmp(Box::new(addressing_mode::ZeroPage(self.next_int()))),
      0xD5 => Cmp(Box::new(addressing_mode::XIndexedZeroPage(self.next_int()))),
      0xCD => Cmp(Box::new(addressing_mode::Absolute(self.next_address()))),
      0xDD => Cmp(Box::new(addressing_mode::XIndexedAbsolute(
        self.next_address(),
      ))),
      0xD9 => Cmp(Box::new(addressing_mode::YIndexedAbsolute(
        self.next_address(),
      ))),
      0xC1 => Cmp(Box::new(addressing_mode::XIndexedIndirect(self.next_int()))),
      0xD1 => Cmp(Box::new(addressing_mode::IndirectYIndexed(self.next_int()))),
      // CPX
      0xE0 => Cpx(Box::new(addressing_mode::Immediate(self.next_int()))),
      0xE4 => Cpx(Box::new(addressing_mode::ZeroPage(self.next_int()))),
      0xEC => Cpx(Box::new(addressing_mode::Absolute(self.next_address()))),
      // CPY
      0xC0 => Cpy(Box::new(addressing_mode::Immediate(self.next_int()))),
      0xC4 => Cpy(Box::new(addressing_mode::ZeroPage(self.next_int()))),
      0xCC => Cpy(Box::new(addressing_mode::Absolute(self.next_address()))),
      // DEC
      0xC6 => Dec(Box::new(addressing_mode::ZeroPage(self.next_int()))),
      0xD6 => Dec(Box::new(addressing_mode::XIndexedZeroPage(self.next_int()))),
      0xCE => Dec(Box::new(addressing_mode::Absolute(self.next_address()))),
      0xDE => Dec(Box::new(addressing_mode::XIndexedAbsolute(
        self.next_address(),
      ))),
      // EOR (XOR)
      0x49 => Eor(Box::new(addressing_mode::Immediate(self.next_int()))),
      0x45 => Eor(Box::new(addressing_mode::ZeroPage(self.next_int()))),
      0x55 => Eor(Box::new(addressing_mode::XIndexedZeroPage(self.next_int()))),
      0x4D => Eor(Box::new(addressing_mode::Absolute(self.next_address()))),
      0x5D => Eor(Box::new(addressing_mode::XIndexedAbsolute(
        self.next_address(),
      ))),
      0x59 => Eor(Box::new(addressing_mode::YIndexedAbsolute(
        self.next_address(),
      ))),
      0x41 => Eor(Box::new(addressing_mode::XIndexedIndirect(self.next_int()))),
      0x51 => Eor(Box::new(addressing_mode::IndirectYIndexed(self.next_int()))),
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
      0xE6 => Inc(Box::new(addressing_mode::ZeroPage(self.next_int()))),
      0xF6 => Inc(Box::new(addressing_mode::XIndexedZeroPage(self.next_int()))),
      0xEE => Inc(Box::new(addressing_mode::Absolute(self.next_address()))),
      0xFE => Inc(Box::new(addressing_mode::XIndexedAbsolute(
        self.next_address(),
      ))),
      // JMP
      0x4C => Jmp(Box::new(addressing_mode::Absolute(self.next_address()))),
      0x6C => Jmp(Box::new(addressing_mode::Indirect(self.next_address()))),
      // JSR
      0x20 => Jsr(Box::new(addressing_mode::Absolute(self.next_address()))),
      // LDA
      0xA9 => Lda(Box::new(addressing_mode::Immediate(self.next_int()))),
      0xA5 => Lda(Box::new(addressing_mode::ZeroPage(self.next_int()))),
      0xB5 => Lda(Box::new(addressing_mode::XIndexedZeroPage(self.next_int()))),
      0xAD => Lda(Box::new(addressing_mode::Absolute(self.next_address()))),
      0xBD => Lda(Box::new(addressing_mode::XIndexedAbsolute(
        self.next_address(),
      ))),
      0xB9 => Lda(Box::new(addressing_mode::YIndexedAbsolute(
        self.next_address(),
      ))),
      0xA1 => Lda(Box::new(addressing_mode::XIndexedIndirect(self.next_int()))),
      0xB1 => Lda(Box::new(addressing_mode::IndirectYIndexed(self.next_int()))),
      // LDX
      0xA2 => Ldx(Box::new(addressing_mode::Immediate(self.next_int()))),
      0xA6 => Ldx(Box::new(addressing_mode::ZeroPage(self.next_int()))),
      0xB6 => Ldx(Box::new(addressing_mode::YIndexedZeroPage(self.next_int()))),
      0xAE => Ldx(Box::new(addressing_mode::Absolute(self.next_address()))),
      0xBE => Ldx(Box::new(addressing_mode::YIndexedAbsolute(
        self.next_address(),
      ))),
      // LDY
      0xA0 => Ldy(Box::new(addressing_mode::Immediate(self.next_int()))),
      0xA4 => Ldy(Box::new(addressing_mode::ZeroPage(self.next_int()))),
      0xB4 => Ldy(Box::new(addressing_mode::XIndexedZeroPage(self.next_int()))),
      0xAC => Ldy(Box::new(addressing_mode::Absolute(self.next_address()))),
      0xBC => Ldy(Box::new(addressing_mode::XIndexedAbsolute(
        self.next_address(),
      ))),
      // LSR
      0x4A => Lsr(Box::new(addressing_mode::Immediate(
        self.register.accumulator,
      ))),
      0x46 => Lsr(Box::new(addressing_mode::ZeroPage(self.next_int()))),
      0x56 => Lsr(Box::new(addressing_mode::XIndexedZeroPage(self.next_int()))),
      0x4E => Lsr(Box::new(addressing_mode::Absolute(self.next_address()))),
      0x5E => Lsr(Box::new(addressing_mode::XIndexedAbsolute(
        self.next_address(),
      ))),
      // NOP
      0xEA => Nop,
      // ORA
      0x09 => Ora(Box::new(addressing_mode::Immediate(self.next_int()))),
      0x05 => Ora(Box::new(addressing_mode::ZeroPage(self.next_int()))),
      0x15 => Ora(Box::new(addressing_mode::XIndexedZeroPage(self.next_int()))),
      0x0D => Ora(Box::new(addressing_mode::Absolute(self.next_address()))),
      0x1D => Ora(Box::new(addressing_mode::XIndexedAbsolute(
        self.next_address(),
      ))),
      0x19 => Ora(Box::new(addressing_mode::YIndexedAbsolute(
        self.next_address(),
      ))),
      0x01 => Ora(Box::new(addressing_mode::XIndexedIndirect(self.next_int()))),
      0x11 => Ora(Box::new(addressing_mode::IndirectYIndexed(self.next_int()))),
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
      0x26 => Rol(Box::new(addressing_mode::ZeroPage(self.next_int()))),
      0x36 => Rol(Box::new(addressing_mode::XIndexedZeroPage(self.next_int()))),
      0x2E => Rol(Box::new(addressing_mode::Absolute(self.next_address()))),
      0x3E => Rol(Box::new(addressing_mode::XIndexedAbsolute(
        self.next_address(),
      ))),
      // ROR
      0x6A => RorAcc,
      0x66 => Ror(Box::new(addressing_mode::ZeroPage(self.next_int()))),
      0x76 => Ror(Box::new(addressing_mode::XIndexedZeroPage(self.next_int()))),
      0x6E => Ror(Box::new(addressing_mode::Absolute(self.next_address()))),
      0x7E => Ror(Box::new(addressing_mode::XIndexedAbsolute(
        self.next_address(),
      ))),
      // RTI
      0x40 => Rti,
      // RTS
      0x60 => Rts,
      // SBC
      0xE9 => Sbc(Box::new(addressing_mode::Immediate(self.next_int()))),
      0xE5 => Sbc(Box::new(addressing_mode::ZeroPage(self.next_int()))),
      0xF5 => Sbc(Box::new(addressing_mode::XIndexedZeroPage(self.next_int()))),
      0xED => Sbc(Box::new(addressing_mode::Absolute(self.next_address()))),
      0xFD => Sbc(Box::new(addressing_mode::XIndexedAbsolute(
        self.next_address(),
      ))),
      0xF9 => Sbc(Box::new(addressing_mode::YIndexedAbsolute(
        self.next_address(),
      ))),
      0xE1 => Sbc(Box::new(addressing_mode::XIndexedIndirect(self.next_int()))),
      0xF1 => Sbc(Box::new(addressing_mode::IndirectYIndexed(self.next_int()))),
      // STA
      0x85 => Sta(Box::new(addressing_mode::ZeroPage(self.next_int()))),
      0x95 => Sta(Box::new(addressing_mode::XIndexedZeroPage(self.next_int()))),
      0x8D => Sta(Box::new(addressing_mode::Absolute(self.next_address()))),
      0x9D => Sta(Box::new(addressing_mode::XIndexedAbsolute(
        self.next_address(),
      ))),
      0x99 => Sta(Box::new(addressing_mode::YIndexedAbsolute(
        self.next_address(),
      ))),
      0x81 => Sta(Box::new(addressing_mode::XIndexedIndirect(self.next_int()))),
      0x91 => Sta(Box::new(addressing_mode::IndirectYIndexed(self.next_int()))),
      // STX
      0x86 => Stx(Box::new(addressing_mode::ZeroPage(self.next_int()))),
      0x96 => Stx(Box::new(addressing_mode::XIndexedZeroPage(self.next_int()))),
      0x8E => Stx(Box::new(addressing_mode::Absolute(self.next_address()))),
      // STY
      0x84 => Sty(Box::new(addressing_mode::ZeroPage(self.next_int()))),
      0x94 => Sty(Box::new(addressing_mode::XIndexedZeroPage(self.next_int()))),
      0x8C => Sty(Box::new(addressing_mode::Absolute(self.next_address()))),
      _ => unimplemented!("opcode {:X?}", opcode),
    }
  }
}
