extern crate ringbuffer;

#[test]
fn test_size_zero() {
    // Expectation: A Ringbuffer with zero size must report errors on push/pop free must return 0
    let mut rbuf = ringbuffer::Ringbuffer::<i32, 0>::new();

    assert_eq!(rbuf.push(1), Err(1));
    assert_eq!(rbuf.pop(), Err(()));
    assert_eq!(rbuf.free(), 0);
}
