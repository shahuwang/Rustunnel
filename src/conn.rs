extern crate crypto;
extern crate bincode;
extern crate rustc_serialize;
use crypto::rc4::Rc4;
use crypto::symmetriccipher::SynchronousStreamCipher;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter, Result, Bytes, Chars, Chain, Tee, Take};
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use bincode::SizeLimit;
use bincode::rustc_serialize::{encode, decode_from};
struct TunConn<'a>{
    conn: &'a TcpStream,
    reader: BufReader<&'a TcpStream>,
    writer: BufWriter<&'a TcpStream>,
    enc: Option<Rc4>,
    dec: Option<Rc4>
}

impl <'a>TunConn<'a>{
    fn set_cipher_key(&mut self, key: &[u8]){
        self.enc = Some(Rc4::new(key));
        self.dec = Some(Rc4::new(key));
    }


    fn write(&mut self, b: &mut [u8]) -> Result<usize>{
        if self.enc.is_some(){
            let w = &mut vec![0; b.len()];
            self.enc.unwrap().process(b, w);
        } 
        return self.writer.write(b);
    }

    fn flush(&mut self) -> Result<()>{
        return self.writer.flush();
    }

    fn set_read_timeout(&mut self, dur: Option<Duration>) -> Result<()>{
        self.conn.set_read_timeout(dur)
    }
}

impl<'a> Read for TunConn<'a>{
    #![allow(unstable)]
    fn read(&mut self, b: &mut [u8]) -> Result<usize>{
        let ret = self.reader.read(b);
        if ret.is_ok() && self.dec.is_some(){
            let size = ret.ok().unwrap();
            if size > 0{
                let mut w = vec![0; size];
                let w1 = &mut w;
                self.dec.unwrap().process(&b[0..size], w1);
                &b[0..size].clone_from_slice(w1);
            }
            return Ok(size);
        }
        return ret;
    }

    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> Result<usize>{
        unimplemented!();
    }

    fn read_to_string(&mut self, buf: &mut String) -> Result<usize>{
        unimplemented!();
        //self.reader.read_to_string(buf)
    }

    fn read_exact(&mut self, buf: &mut [u8]) -> Result<()>{
        unimplemented!();
        // self.reader.read_exact(buf)
    }

    fn by_ref(&mut self) -> &mut Self where Self: Sized{
        unimplemented!();
        // self.reader.by_ref()
    }

    fn bytes(self) -> Bytes<Self> where Self: Sized{
        unimplemented!();
        // self.reader.bytes() 
    }
    
    fn chars(self) -> Chars<Self> where Self: Sized{
        unimplemented!();
        // self.reader.chars()
    }
    fn chain<R: Read>(self, next: R) -> Chain<Self, R> where Self: Sized{
        unimplemented!();
        // self.reader.chain(next)
    }
    fn take(self, limit: u64) -> Take<Self> where Self: Sized{
        unimplemented!();
        // self.reader.take(limit)
    }

    fn tee<W:Write>(self, out: W) -> Tee<Self, W> where Self:Sized{
        unimplemented!();
        // self.reader.tee(out)
    }

}

#[derive(RustcEncodable, RustcDecodable, PartialEq)]
struct Header{
    linkid: u16,
    len: u16,
}

struct Tunnel<'a>{
    tcon: TunConn<'a>,
    mutex: Mutex<u8>,
}

impl<'a>Tunnel<'a>{
    fn new(conn: &'a TcpStream) -> Tunnel{
        let tcon = TunConn{
            conn: conn,
            reader: BufReader::new(conn),
            writer: BufWriter::new(conn),
            enc: None,
            dec: None,
        };
        Tunnel{
            tcon: tcon,
            mutex: Mutex::new(0)
        }
    }

    fn write(&mut self, linkid: u16, data: &mut [u8]) -> Result<usize>{
        let header = Header{
            linkid: linkid,
            len: data.len() as u16
        };
        let mut enc = encode(&header, SizeLimit::Infinite).unwrap();
        self.mutex.lock();
        let ret = self.tcon.write(&mut enc);
        if ret.is_err(){
            return ret;
        }
        let ret2 = self.tcon.write(data);
        return ret2;
    }

    fn read(&mut self){
        let timeout = Duration::new(60 * 10, 0);
        self.mutex.lock();
        self.tcon.set_read_timeout(Some(timeout)); 
        let ret: Header = decode_from(&mut self.tcon, SizeLimit::Infinite).unwrap();
    }
}
