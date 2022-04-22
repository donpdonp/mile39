use std::io::BufRead;
use std::io::BufReader;
use std::io::Lines;
use std::net::TcpStream;

pub struct Peer {
    pub stream: TcpStream,
}

pub fn setup(stream: TcpStream) {
    let peer = Peer { stream: stream };
    peer.notice();
    for line in peer.feed_lines(){
        println!("next line {}", line.unwrap());
    }
}

impl Peer {
    pub fn notice(&self) {
        println!(
            "connected from {} to {}",
            self.stream.peer_addr().unwrap(),
            self.stream.local_addr().unwrap()
        )
    }
    pub fn feed_lines(self) -> Lines<BufReader<TcpStream>> {
        BufReader::new(self.stream).lines()
    }
}
