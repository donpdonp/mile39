use std::net::SocketAddr;
use std::net::TcpListener;
use std::str::FromStr;

pub struct Net {
    pub addr: SocketAddr,
    pub listener: TcpListener,
}

pub fn setup(addr: &str) -> Net {
    let addr = SocketAddr::from_str(addr).unwrap();
    let listener = TcpListener::bind(addr).unwrap();
    return Net {
        addr: addr,
        listener: listener,
    };
}
