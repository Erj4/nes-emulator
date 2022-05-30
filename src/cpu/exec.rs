use super::operation::Operation;
use super::registers;

impl super::Cpu {
  /// # Panics
  /// This function panics if it attempts to execute an operation without an implementation
  pub fn execute(&mut self, operation: &Operation) {
    use Operation::*;

    log::debug!(
      "executing operation {:#?} at {:#x}",
      operation,
      &self.register.program_counter
    );

    return match operation {
      Adc(addr) => {
        let (value, overflow) = self.register.accumulator.overflowing_add(addr.get(self));
        self.register.accumulator = value;
        self.register.status.result_status.overflow = registers::Overflow::from(overflow);
      }
      Brk => self.stop = true,
      _ => unimplemented!("operation {:#?}", operation),
    };
  }
}
