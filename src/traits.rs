pub trait Ringbuffer {
    type Item;

    fn push(&mut self, val: Self::Item) -> Result<(), Self::Item>;
    fn pop(&mut self) -> Result<Self::Item, ()>;
    fn free(&self) -> usize;
    fn size(&self) -> usize;
}
