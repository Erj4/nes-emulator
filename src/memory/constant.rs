use crate::memory::Address;

// _END locations are EXCLUSIVE

pub const RAM_START: Address = 0x0000;
pub const RAM_SIZE: Address = 0x0800;
pub const RAM_END: Address = RAM_START + RAM_SIZE;
pub const PROGRAM_ROM_START: Address = 0x8000;
pub const PROGRAM_ROM_SIZE: Address = 0x8000; // ROM runs to end of memory (0xFFFF inclusive)

/// Contains address of non-maskable interrupt handler
/// Value is u16, so should be read with [`NES::read_u16`]
pub const NON_MASKABLE_INTERRUPT: Address = 0xFFFA;

/// Contains address to set program counter to on reset interrupt or load
/// This is a u16 value, so should be read with [`NES::read_u16`]
pub const PROGRAM_COUNTER_RESET: Address = 0xFFFC;

/// Contains address of (maskable) interrupt handler, also triggered by BRK instruction
/// Value is u16, so should be read with [`NES::read_u16`]
pub const INTERRUPT: Address = 0xFFFE;
