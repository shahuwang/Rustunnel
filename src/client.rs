use std::sync::{Arc, Mutex};
use std::net::{TcpListener, TcpStream};
use id_allocator::IdAllocator;
struct Client<'a>{
    laddr: String,
    backend: String,
    secret: String,
    tunnels: i32,
    alloc: &'a IdAllocator,
    mutex: Mutex<u8>
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
            alloc: alloc,
            mutex: Mutex::new(0)
        };
        return client;
    }

    fn start(&mut self){
        let listener = TcpListener::bind(self.laddr.as_str()).unwrap();
        loop{
            match listener.accept(){
                Err(e) =>{
                    println!("{}", e);
                },
                Ok((stream, remoteAddr))=>{
                    println!("connect from {}:{}",
                             remoteAddr.ip(), remoteAddr.port());

                } 
            }
        }
    }
}
