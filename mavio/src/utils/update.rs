/// <sup>`⚠`</sup>
/// A trait, that updates internal state of a type using an instance of another type.
pub trait TryUpdateFrom<T> {
    /// Error, that may be thrown during update.
    type Error;

    /// <sup>`⚠`</sup>
    /// Performs the update.
    ///
    /// **⚠** If you are going to reimplement the blanket implementation, then it is mandatory, that
    /// all checks related to the possibility of update should be performed before the update.
    ///
    /// Returns [`Self::Error`], when update is impossible.
    fn try_update_from(&mut self, value: T) -> Result<(), Self::Error> {
        self.check_try_update_from(&value)?;
        unsafe {
            self.update_from_unchecked(value);
        }
        Ok(())
    }

    /// <sup>`⚠`</sup>
    /// Checks, that update is possible.
    ///
    /// Returns [`Self::Error`], when update is impossible.
    fn check_try_update_from(&self, value: &T) -> Result<(), Self::Error>;

    /// <sup>`⚠`</sup>
    /// Performs the update without checking, whether update is possible.
    ///
    /// **⚠** This method should not be used directly. Use [`Self::try_update_from`] instead.
    ///
    /// # Panics
    ///
    /// This method should panic, when update is not possible. It is marked as `unsafe` since
    /// Rust type system can't guarantee, that the contract will be fulfilled by implementor.
    unsafe fn update_from_unchecked(&mut self, value: T);
}
