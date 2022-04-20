use std::net::TcpStream;

pub struct Peer {
    pub stream: TcpStream,
}

pub fn setup(stream: TcpStream) {
    let peer = Peer { stream: stream };
    peer.connect();
}

impl Peer {
    pub fn connect(self) {
        println!(
            "connected from {} to {}",
            self.stream.peer_addr().unwrap(),
            self.stream.local_addr().unwrap()
        )
    }
}
