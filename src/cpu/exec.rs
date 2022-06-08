use super::error::Error;
use super::operation::Operation;
use super::registers;

impl super::Cpu {
  /// Executes the given operation
  ///
  /// # Errors
  /// Forwards errors from executing the operation
  ///
  /// Returns [`Error::UnimplementedOperation`] if the provided operation is not implemented
  pub fn execute(&mut self, operation: Operation) -> Result<(), Error> {
    use Operation::*;

    log::debug!(
      "executing operation {:#?} at {:#x}",
      operation,
      &self.register.program_counter
    );

    match operation {
      Adc(addr) => {
        let (value, overflow) = self.register.accumulator.overflowing_add(addr.value(self));
        self.register.accumulator = value;
        self.register.status.result_status.overflow = registers::Overflow::from(overflow);
      }
      Brk => self.stop = true,
      _ => return Err(Error::UnimplementedOperation(operation)),
    };

    Ok(())
  }
}
