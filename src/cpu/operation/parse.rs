use super::{addressing_mode::Resolvable as Addr, Cpu, Operation};

impl Operation {
  /// Convert an opcode Int value and its following arguments to an Operation
  /// # Panics
  /// This function will panic if it recieves an opcode that is not defined
  #[allow(clippy::too_many_lines)]
  pub fn new(cpu: &mut Cpu) -> Operation {
    use Operation::*;

    let opcode = cpu.next_int();
    match opcode {
      // ADC
      0x69 => Adc(Addr::immediate(cpu)),
      0x65 => Adc(Addr::zero_page(cpu)),
      0x75 => Adc(Addr::x_indexed_zero_page(cpu)),
      0x6D => Adc(Addr::absolute(cpu)),
      0x7D => Adc(Addr::x_indexed_absolute(cpu)),
      0x79 => Adc(Addr::y_indexed_absolute(cpu)),
      0x61 => Adc(Addr::x_indexed_indirect(cpu)),
      0x71 => Adc(Addr::indirect_y_indexed(cpu)),
      // AND
      0x29 => And(Addr::immediate(cpu)),
      0x25 => And(Addr::zero_page(cpu)),
      0x35 => And(Addr::x_indexed_zero_page(cpu)),
      0x2D => And(Addr::absolute(cpu)),
      0x3D => And(Addr::x_indexed_absolute(cpu)),
      0x39 => And(Addr::y_indexed_absolute(cpu)),
      0x21 => And(Addr::x_indexed_indirect(cpu)),
      0x31 => And(Addr::indirect_y_indexed(cpu)),
      // ASL
      0x0A => ASLAcc,
      0x06 => Asl(Addr::zero_page(cpu)),
      0x16 => Asl(Addr::x_indexed_zero_page(cpu)),
      0x0E => Asl(Addr::absolute(cpu)),
      0x1E => Asl(Addr::x_indexed_absolute(cpu)),
      // BIT
      0x24 => Bit(Addr::zero_page(cpu)),
      0x2C => Bit(Addr::absolute(cpu)),
      // Branch
      0x10 => Bpl(Addr::relative(cpu)),
      0x30 => Bmi(Addr::relative(cpu)),
      0x50 => Bvc(Addr::relative(cpu)),
      0x70 => Bvs(Addr::relative(cpu)),
      0x90 => Bcc(Addr::relative(cpu)),
      0xB0 => Bcs(Addr::relative(cpu)),
      0xD0 => Bne(Addr::relative(cpu)),
      0xF0 => Beq(Addr::relative(cpu)),
      // BRK
      0x00 => Brk,
      // CMP
      0xC9 => Cmp(Addr::immediate(cpu)),
      0xC5 => Cmp(Addr::zero_page(cpu)),
      0xD5 => Cmp(Addr::x_indexed_zero_page(cpu)),
      0xCD => Cmp(Addr::absolute(cpu)),
      0xDD => Cmp(Addr::x_indexed_absolute(cpu)),
      0xD9 => Cmp(Addr::y_indexed_absolute(cpu)),
      0xC1 => Cmp(Addr::x_indexed_indirect(cpu)),
      0xD1 => Cmp(Addr::indirect_y_indexed(cpu)),
      // CPX
      0xE0 => Cpx(Addr::immediate(cpu)),
      0xE4 => Cpx(Addr::zero_page(cpu)),
      0xEC => Cpx(Addr::absolute(cpu)),
      // CPY
      0xC0 => Cpy(Addr::immediate(cpu)),
      0xC4 => Cpy(Addr::zero_page(cpu)),
      0xCC => Cpy(Addr::absolute(cpu)),
      // DEC
      0xC6 => Dec(Addr::zero_page(cpu)),
      0xD6 => Dec(Addr::x_indexed_zero_page(cpu)),
      0xCE => Dec(Addr::absolute(cpu)),
      0xDE => Dec(Addr::x_indexed_absolute(cpu)),
      // EOR (XOR)
      0x49 => Eor(Addr::immediate(cpu)),
      0x45 => Eor(Addr::zero_page(cpu)),
      0x55 => Eor(Addr::x_indexed_zero_page(cpu)),
      0x4D => Eor(Addr::absolute(cpu)),
      0x5D => Eor(Addr::x_indexed_absolute(cpu)),
      0x59 => Eor(Addr::y_indexed_absolute(cpu)),
      0x41 => Eor(Addr::x_indexed_indirect(cpu)),
      0x51 => Eor(Addr::indirect_y_indexed(cpu)),
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
      0xE6 => Inc(Addr::zero_page(cpu)),
      0xF6 => Inc(Addr::x_indexed_zero_page(cpu)),
      0xEE => Inc(Addr::absolute(cpu)),
      0xFE => Inc(Addr::x_indexed_absolute(cpu)),
      // JMP
      0x4C => Jmp(Addr::absolute(cpu)),
      0x6C => Jmp(Addr::indirect(cpu)),
      // JSR
      0x20 => Jsr(Addr::absolute(cpu)),
      // LDA
      0xA9 => Lda(Addr::immediate(cpu)),
      0xA5 => Lda(Addr::zero_page(cpu)),
      0xB5 => Lda(Addr::x_indexed_zero_page(cpu)),
      0xAD => Lda(Addr::absolute(cpu)),
      0xBD => Lda(Addr::x_indexed_absolute(cpu)),
      0xB9 => Lda(Addr::y_indexed_absolute(cpu)),
      0xA1 => Lda(Addr::x_indexed_indirect(cpu)),
      0xB1 => Lda(Addr::indirect_y_indexed(cpu)),
      // LDX
      0xA2 => Ldx(Addr::immediate(cpu)),
      0xA6 => Ldx(Addr::zero_page(cpu)),
      0xB6 => Ldx(Addr::y_indexed_zero_page(cpu)),
      0xAE => Ldx(Addr::absolute(cpu)),
      0xBE => Ldx(Addr::y_indexed_absolute(cpu)),
      // LDY
      0xA0 => Ldy(Addr::immediate(cpu)),
      0xA4 => Ldy(Addr::zero_page(cpu)),
      0xB4 => Ldy(Addr::x_indexed_zero_page(cpu)),
      0xAC => Ldy(Addr::absolute(cpu)),
      0xBC => Ldy(Addr::x_indexed_absolute(cpu)),
      // LSR
      0x4A => Lsr(Addr::accumulator(cpu)),
      0x46 => Lsr(Addr::zero_page(cpu)),
      0x56 => Lsr(Addr::x_indexed_zero_page(cpu)),
      0x4E => Lsr(Addr::absolute(cpu)),
      0x5E => Lsr(Addr::x_indexed_absolute(cpu)),
      // NOP
      0xEA => Nop,
      // ORA
      0x09 => Ora(Addr::immediate(cpu)),
      0x05 => Ora(Addr::zero_page(cpu)),
      0x15 => Ora(Addr::x_indexed_zero_page(cpu)),
      0x0D => Ora(Addr::absolute(cpu)),
      0x1D => Ora(Addr::x_indexed_absolute(cpu)),
      0x19 => Ora(Addr::y_indexed_absolute(cpu)),
      0x01 => Ora(Addr::x_indexed_indirect(cpu)),
      0x11 => Ora(Addr::indirect_y_indexed(cpu)),
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
      0x26 => Rol(Addr::zero_page(cpu)),
      0x36 => Rol(Addr::x_indexed_zero_page(cpu)),
      0x2E => Rol(Addr::absolute(cpu)),
      0x3E => Rol(Addr::x_indexed_absolute(cpu)),
      // ROR
      0x6A => RorAcc,
      0x66 => Ror(Addr::zero_page(cpu)),
      0x76 => Ror(Addr::x_indexed_zero_page(cpu)),
      0x6E => Ror(Addr::absolute(cpu)),
      0x7E => Ror(Addr::x_indexed_absolute(cpu)),
      // RTI
      0x40 => Rti,
      // RTS
      0x60 => Rts,
      // SBC
      0xE9 => Sbc(Addr::immediate(cpu)),
      0xE5 => Sbc(Addr::zero_page(cpu)),
      0xF5 => Sbc(Addr::x_indexed_zero_page(cpu)),
      0xED => Sbc(Addr::absolute(cpu)),
      0xFD => Sbc(Addr::x_indexed_absolute(cpu)),
      0xF9 => Sbc(Addr::y_indexed_absolute(cpu)),
      0xE1 => Sbc(Addr::x_indexed_indirect(cpu)),
      0xF1 => Sbc(Addr::indirect_y_indexed(cpu)),
      // STA
      0x85 => Sta(Addr::zero_page(cpu)),
      0x95 => Sta(Addr::x_indexed_zero_page(cpu)),
      0x8D => Sta(Addr::absolute(cpu)),
      0x9D => Sta(Addr::x_indexed_absolute(cpu)),
      0x99 => Sta(Addr::y_indexed_absolute(cpu)),
      0x81 => Sta(Addr::x_indexed_indirect(cpu)),
      0x91 => Sta(Addr::indirect_y_indexed(cpu)),
      // STX
      0x86 => Stx(Addr::zero_page(cpu)),
      0x96 => Stx(Addr::x_indexed_zero_page(cpu)),
      0x8E => Stx(Addr::absolute(cpu)),
      // STY
      0x84 => Sty(Addr::zero_page(cpu)),
      0x94 => Sty(Addr::x_indexed_zero_page(cpu)),
      0x8C => Sty(Addr::absolute(cpu)),
      _ => unimplemented!("opcode {:X?}", opcode),
    }
  }
}
