use std::sync::mpsc::{Sender, Receiver, channel};
pub struct IdAllocator{
    sender: Sender<u16>,
    recv: Receiver<u16>,
}

impl IdAllocator{
    pub fn new() -> IdAllocator{
        let (tx, rx) = channel();
        let max = u16::max_value();
        for id in 1..max{
            tx.send(id);
        }
        IdAllocator{
            sender: tx,
            recv: rx
        }
    }

    pub fn acquire(&self) -> u16{
        self.recv.recv().unwrap()
    }

    pub fn release(&self, id: u16){
        self.sender.send(id);
    }
}
