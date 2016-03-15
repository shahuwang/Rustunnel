#![feature(io)]
extern crate crypto;
extern crate bincode;
extern crate rustc_serialize;
mod client;
mod id_allocator;
mod conn;
mod hub;
mod link_buffer;
// use std::io::prelude::*;
// use std::io::BufWriter;
// use std::net::TcpStream;

// This code is editable and runnable!
fn main() {
   // let conn = TcpStream::connect("220.181.57.217:80").unwrap(); 
   // let mut stream = BufWriter::new(&conn);

   // for i in 1..10 {
   //     println!("helo");
   //    stream.write(&[i]).unwrap();
   //  }
}

