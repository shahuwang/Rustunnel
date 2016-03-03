use std::sync::{Arc, Mutex};
use id_allocator::IdAllocator;
struct Client<'a>{
    laddr: String,
    backend: String,
    secret: String,
    tunnels: i32,
    alloc: &'a IdAllocator
}

const dailTimeoutSeconds:i32 = 5;

impl<'a> Client<'a>{
    fn new(listen: String, backend: String,
           secret: String, tunnels: i32, alloc: &'a IdAllocator) -> Client{
        let client = Client{
            laddr: listen,
            backend: backend,
            secret: secret,
            tunnels: tunnels,
            alloc: alloc
        };
        return client;
    }

    fn start(){
        
    }
}
