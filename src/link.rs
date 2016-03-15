use std::net::{TcpStream};
use hub::Hub;
struct Link{
    id: u16,
    conn: TcpStream,
    hub: &'a Hub,
}
