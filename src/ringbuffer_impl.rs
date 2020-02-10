// Import Ringbuffer Trait from top level module
use crate::traits::Ringbuffer;


/// Array based implementation of Ringbuffer
pub struct ArrayRingbuffer<T, const N: usize> {
    buffer: [T; N],    // Array used to store objects
    rpos: usize,       // Current read position
    wpos: usize,       // Current write position
    used: usize,       // Number of used slots in buffer
}

impl<T: Copy, const N: usize> ArrayRingbuffer<T, { N }> {
    /// Creator method for ArrayRingbuffer
    ///
    /// # Returns
    /// * ArrayRingbuffer object ready to use.
    pub fn new() -> Self {
        Self {
            buffer: unsafe { core::mem::zeroed() },
            rpos: 0,
            wpos: 0,
            used: 0,
        }
    }
}

impl<T: Copy, const N: usize> Ringbuffer for ArrayRingbuffer<T, { N }> {
    type Item = T;

    fn push(&mut self, val: T) -> Result<(), T> {
        if self.used >= N {
            return Err(val);
        }

        self.buffer[self.wpos] = val;
        self.used += 1;
        self.wpos += 1;

        if self.wpos >= N {
            self.wpos = 0;
        }
        Ok(())
    }

    fn pop(&mut self) -> Result<T, ()> {
        if self.used <= 0 {
            return Err(());
        }

        let val = self.buffer[self.rpos];

        self.used -= 1;
        self.rpos += 1;

        if self.rpos >= N {
            self.rpos = 0
        }

        Ok(val)
    }

    fn free(&self) -> usize {
        self.size() - self.used
    }

    fn size(&self) -> usize {
        N
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    extern crate test;
    use test::Bencher;

    #[test]
    fn test_push() {
        // Expected Behavior:
        // A ring buffer of size 5 must allow push 5 times before returning
        // Err. Err must contain the value push was called with.
        let mut rbuf = ArrayRingbuffer::<u32, 5>::new();
        assert_eq!(rbuf.push(1), Ok(()));
        assert_eq!(rbuf.push(2), Ok(()));
        assert_eq!(rbuf.push(3), Ok(()));
        assert_eq!(rbuf.push(4), Ok(()));
        assert_eq!(rbuf.push(5), Ok(()));
        assert_eq!(rbuf.push(6), Err(6));
    }

    #[test]
    fn test_pop() {
        // Expected Behavior:
        // Pop on an empty Ringbuffer must return Err. If values were
        // pushed into the Ringbuffer, pop must return the values keeping the
        // order the values were pushed.
        let mut rbuf = ArrayRingbuffer::<u32, 5>::new();

        assert_eq!(rbuf.pop(), Err(()));

        assert_eq!(rbuf.push(1), Ok(()));
        assert_eq!(rbuf.pop(), Ok(1));
        assert_eq!(rbuf.pop(), Err(()));

        assert_eq!(rbuf.push(1), Ok(()));
        assert_eq!(rbuf.push(2), Ok(()));
        assert_eq!(rbuf.pop(), Ok(1));
        assert_eq!(rbuf.pop(), Ok(2));
        assert_eq!(rbuf.pop(), Err(()));

        assert_eq!(rbuf.push(1), Ok(()));
        assert_eq!(rbuf.push(2), Ok(()));
        assert_eq!(rbuf.push(3), Ok(()));
        assert_eq!(rbuf.push(4), Ok(()));
        assert_eq!(rbuf.push(5), Ok(()));
        assert_eq!(rbuf.pop(), Ok(1));
        assert_eq!(rbuf.pop(), Ok(2));
        assert_eq!(rbuf.pop(), Ok(3));
        assert_eq!(rbuf.pop(), Ok(4));
        assert_eq!(rbuf.pop(), Ok(5));
        assert_eq!(rbuf.pop(), Err(()));
    }

    #[test]
    fn test_free() {
        // Expected Behavior:
        // With each successful push, free must decrease by 1
        // With each successful pop, free must increase by 1
        // If push or pop failed , free must not change.
        let mut rbuf = ArrayRingbuffer::<u32, 3>::new();

        assert_eq!(rbuf.free(), 3);
        let _ = rbuf.pop();
        assert_eq!(rbuf.free(), 3);

        rbuf.push(1).unwrap();
        assert_eq!(rbuf.free(), 2);

        rbuf.push(1).unwrap();
        assert_eq!(rbuf.free(), 1);

        rbuf.push(1).unwrap();
        assert_eq!(rbuf.free(), 0);

        rbuf.pop().unwrap();
        assert_eq!(rbuf.free(), 1);

        rbuf.pop().unwrap();
        assert_eq!(rbuf.free(), 2);

        rbuf.pop().unwrap();
        assert_eq!(rbuf.free(), 3);
    }

    #[test]
    fn test_size() {
        // Expected Behavior:
        // Size must always return the number of elements the type was constructed with
        assert_eq!(ArrayRingbuffer::<u32, 0>::new().size(), 0);
        assert_eq!(ArrayRingbuffer::<u32, 1>::new().size(), 1);
        assert_eq!(ArrayRingbuffer::<u32, 2>::new().size(), 2);
    }

    #[test]
    fn test_custom_type() {
        #[derive(Copy, Clone, Debug, PartialEq)]
        struct CustomType {
            i: i32,
            f: f32,
        }

        let val1 = CustomType { i: 2, f: 3.14 };
        let val2 = CustomType { i: 42, f: -1.32 };

        let mut rbuf = ArrayRingbuffer::<CustomType, 2>::new();

        assert_eq!(rbuf.push(val1), Ok(()));
        assert_eq!(rbuf.push(val2), Ok(()));

        assert_eq!(rbuf.pop(), Ok(val1));
        assert_eq!(rbuf.pop(), Ok(val2));
    }

    #[bench]
    fn benchmark_push_pop_ints(b: &mut Bencher) {
        let mut rbuf = ArrayRingbuffer::<usize, 100>::new();

        b.iter(|| {
            let size = rbuf.size();

            for i in 0..size {
                rbuf.push(i).unwrap();
            }

            for _ in 0..size {
                rbuf.pop().unwrap();
            }
        });
    }
}
