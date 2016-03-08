extern crate crypto;
use crypto::rc4::Rc4;
use std::io::prelude::*;
use std::io::BufWriter;
use std::net::TcpStream;
use std::result::Result;
struct TunConn<'a>{
    conn: &'a TcpStream,
    reader: &'a BufRead,
    writer: &'a BufWriter<TcpStream>,
    enc: Rc4,
    dec: Rc4
}

impl <'a>TunConn<'a>{
    fn set_cipher_key(&mut self, key: &[u8]){
        self.enc = Rc4::new(key);
        self.dec = Rc4::new(key);
    }

    fn read(&mut self, b: Vec<u8>) -> Result<i32, &str>{

    }
}
