use conn::Tunnel;
use std::collections::HashMap;
enum LINK_STATUS{
    LINK_DATA,
    LINK_CREATE,
    LINK_CLOSE,
    LINK_CLOSE_RECV,
    LINK_CLOSE_SEND
}

struct Cmd{
    cmd: u8,
    linkid: u16
}

struct Hub{
    tunnel: Tunnel,
}
