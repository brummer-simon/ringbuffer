// Declare crate attributes.
#![no_std]
#![feature(const_generics)]
#![feature(test)]

// Declare crate modules. A module must be declared here to be built.
mod traits;
mod ringbuffer_impl;

// Export symbols from modules to crate API.
pub use traits::Ringbuffer;
pub use ringbuffer_impl::ArrayRingbuffer;
