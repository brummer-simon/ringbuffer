/// cargo doc comment: This Trait specifies the Ringbuffer Interface.
/// Even Markdown is allowed here!
pub trait Ringbuffer {
    /// Object type to store within Ringbuffer
    type Item;

    /// Store a Item in the Ringbuffer
    ///
    /// # Arguments
    /// * val - The value to store.
    ///
    /// # Returns
    /// * Ok(())   - if val was stored successfully in Ringbuffer.
    /// * Err(val) - if given value could not be stored in Ringbuffer.
    fn push(&mut self, val: Self::Item) -> Result<(), Self::Item>;

    /// Get Item from Ringbuffer
    ///
    /// # Returns
    /// * Ok(val) - if Ringbuffer was not empty.
    /// * Err(()) - if Ringbuffer was empty.
    fn pop(&mut self) -> Result<Self::Item, ()>;

    /// Get number of objects left to store.
    fn free(&self) -> usize;

    /// Get maximum capacity of Ringbuffer.
    fn size(&self) -> usize;
}
