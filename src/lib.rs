#![no_std]
#![feature(const_generics)]

// Interface
/*
pub trait RingBufferIf<T>
{
    fn push(&mut self, val: T) -> Result<(), T>;
    fn pop(&mut self) -> Result<T, ()>;
    fn free(&self) -> usize;
}
*/

// Implementation
pub struct Ringbuffer<T, const N: usize> {
    buffer: [T; N],
    rpos: usize,
    wpos: usize,
    used: usize,
}

impl<T: Copy, const N: usize> Ringbuffer<T, { N }> {
    pub fn new() -> Self {
        Self {
            buffer: unsafe { core::mem::zeroed() },
            rpos: 0,
            wpos: 0,
            used: 0,
        }
    }

    pub fn push(&mut self, val: T) -> Result<(), T> {
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

    pub fn pop(&mut self) -> Result<T, ()> {
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

    pub fn free(&self) -> usize {
        N - self.used
    }
}

#[cfg(test)]
mod tests {
    use super::Ringbuffer;

    #[test]
    fn test_push() {
        // Expected Behavior:
        // A ring buffer of size 5 must allow push 5 times before returning
        // Err. Err must contain the value push was called with.
        let mut rbuf = Ringbuffer::<u32, 5>::new();
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
        let mut rbuf = Ringbuffer::<u32, 5>::new();

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
        let mut rbuf = Ringbuffer::<u32, 5>::new();

        assert_eq!(rbuf.free(), 5);
        let _ = rbuf.pop();
        assert_eq!(rbuf.free(), 5);

        rbuf.push(1).unwrap();
        assert_eq!(rbuf.free(), 4);

        rbuf.push(1).unwrap();
        assert_eq!(rbuf.free(), 3);

        rbuf.push(1).unwrap();
        assert_eq!(rbuf.free(), 2);

        rbuf.push(1).unwrap();
        assert_eq!(rbuf.free(), 1);

        rbuf.push(1).unwrap();
        assert_eq!(rbuf.free(), 0);

        let _ = rbuf.push(1);
        assert_eq!(rbuf.free(), 0);

        rbuf.pop().unwrap();
        assert_eq!(rbuf.free(), 1);

        rbuf.pop().unwrap();
        assert_eq!(rbuf.free(), 2);

        rbuf.pop().unwrap();
        assert_eq!(rbuf.free(), 3);

        rbuf.push(1).unwrap();
        assert_eq!(rbuf.free(), 2);
    }
}
