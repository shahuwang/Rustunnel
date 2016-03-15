use std::sync::{Mutex, Condvar};
struct LinkBuffer{
    start: usize,
    end: usize,
    buf: Vec<Box<[u8]>>,
    cond: Condvar,
    mutex: Mutex<u8>,
    closed: bool
}

impl LinkBuffer{
    fn buffer_len(&self) -> usize{
        return (self.end + self.buf.capacity() - self.start) % self.buf.capacity();
    }

    fn len(&self) -> usize{
        self.mutex.lock();
        return self.buffer_len();
    }

    fn close(&mut self) -> bool{
        self.mutex.lock();
        if self.closed{
            return false;
        }
        self.closed = true;
        self.cond.notify_all();
        return true;
    }

    fn put(&mut self, data: [u8]) -> bool{
        self.mutex.lock();
        if self.closed{
            return false;
        }
        let old_cap = self.buf.capacity();
        if (self.end + 1) % old_cap == self.start{
            let buf: Vec<Box<[u8]>> = Vec::with_capacity(old_cap * 2);
            if self.end > self.start{
               buf.extend_from_slice(self.buf[self.start..self.end]); 
            }
        }
    }
}
