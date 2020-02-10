// Declare create attributes.
#![no_std]
#![feature(const_generics)]

// Declare create modules. A module must be declared here to be built.
mod ringbuffer_impl;
mod traits;

// Export symbols from modules to create API.
pub use ringbuffer_impl::ArrayRingbuffer;
pub use traits::Ringbuffer;
