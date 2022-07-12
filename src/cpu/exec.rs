use tracing::{debug, instrument};

use super::error::Error;
use super::operation::addressing_mode::Value;
use super::operation::Operation;

impl super::Cpu {
  fn set_accumulator(&mut self, result: super::Int) {
    self.register.accumulator = result;
    self.register.status.result_status.zero = result == 0;
  }

  /// Executes the given operation
  ///
  /// # Errors
  /// Forwards errors from executing the operation
  ///
  /// Returns [`Error::UnimplementedOperation`] if the provided operation is not implemented
  #[instrument]
  pub fn execute(&mut self, operation: Operation) -> Result<(), Error> {
    use Operation::*;
    debug!(?operation);
    match operation {
      Adc(value) => self.add_with_carry(value),
      And(value) => self.and(value),
      Brk => self.stop = true,
      _ => return Err(Error::UnimplementedOperation(operation)),
    };

    Ok(())
  }

  fn add_with_carry<T: Into<Value>>(&mut self, source: T) {
    let accumulator = self.register.accumulator;
    let source: Value = source.into();
    let value = source.value(self);
    debug!(accumulator, value, ?source);

    let (mut result, mut carry) = accumulator.overflowing_add(value);
    if self.register.status.result_status.overflow {
      let (carry_result, carry_carry) = result.overflowing_add(1);
      result = carry_result;
      carry |= carry_carry;
    }

    self.set_accumulator(result);
    self.register.status.result_status.carry = carry;
    // Overflow if both inputs' sign bits differ from the result sign
    let overflow = (accumulator ^ result) & (value ^ result) & 0x80;
    self.register.status.result_status.overflow = overflow != 0;
  }

  fn and<T: Into<Value>>(&mut self, source: T) {
    let accumulator = self.register.accumulator;
    let source: Value = source.into();
    let value = source.value(self);
    let result = accumulator & value;
    self.set_accumulator(result);
  }
}

#[cfg(test)]
mod tests {
  use crate::cpu::{self, operation::addressing_mode::Value};
  use test_case::test_case;

  #[test_case(80, 16, true => (96, false, false))]
  #[test_case(80, 80, true => (160, false, true))]
  #[test_case(80, 144, true => (224, false, false))]
  #[test_case(80, 208, true => (32, true, false))]
  #[test_case(208, 16, true => (224, false, false))]
  #[test_case(208, 80, true => (32, true, false))]
  #[test_case(208, 144, true => (96, true, true))]
  #[test_case(208, 208, true => (160, true, false))]
  fn add_with_carry(accumulator: u8, value: u8, carry: bool) -> (u8, bool, bool) {
    let mut cpu = cpu::Cpu::default();
    cpu.register.accumulator = accumulator;
    cpu.register.status.result_status.carry = carry;

    cpu.add_with_carry(Value::Immediate(value));

    (
      cpu.register.accumulator,
      cpu.register.status.result_status.carry,
      cpu.register.status.result_status.overflow,
    )
  }
}
