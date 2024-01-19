#![allow(unused_imports)]
use bpaf::Bpaf;
use std::{
    io::{stdout, Write},
    net::{IpAddr, Ipv4Addr},
    sync::mpsc::{channel, Sender},
};
use tokio::{net::TcpStream, task};

const MAX: u16 = 65535; // max IP port

const IPFALLBACK: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

/* CLI arguments */
#[derive(Debug, Bpaf, Clone)]
#[bpaf(options)]
pub struct Args {
    #[bpaf(long, short, fallback(IPFALLBACK))]
    /// valid IPv4 address that is to be sniffed, falls back to `IPFALLBACK`
    pub address: IpAddr,
    #[bpaf(
        long("start"),
        short('s'),
        fallback(1u16),
        guard(start_port_guard, "Cannot be less than 1...")
    )]
    /// start port for the sniffer
    pub start_port: u16,
    #[bpaf(
        long("end"),
        short('e'),
        fallback(1u16),
        guard(end_port_guard, "Cannot be more than 65535...")
    )]
    /// end port for the sniffer
    pub end_port: u16,
}

fn start_port_guard(port: &u16) -> bool {
    *port > 0
}

fn end_port_guard(port: &u16) -> bool {
    *port <= MAX
}

async fn scan(tx: Sender<u16>, port: u16, addr: IpAddr) {
    match TcpStream::connect(format!("{}:{}", addr, port)).await {
        Ok(_) => {
            print!(".");
            stdout().flush().unwrap();
            tx.send(port).unwrap();
        }
        Err(_) => {}
    }
}

#[tokio::main]
async fn main() {
    let opts: Args = args().run();
    let (tx, rx) = channel();

    for i in opts.start_port..opts.end_port {
        let tx = tx.clone();
        task::spawn(async move {
            scan(tx, i, opts.address).await;
        });
    }

    // collect the ports
    let mut out = vec![];
    drop(tx);
    for p in rx {
        out.push(p);
    }

    println!();
    out.sort();
    for v in out {
        println!("{} is open...", v);
    }
}

#[cfg(test)]
mod test {
    use tokio::net::TcpListener;

    use super::*;
    #[tokio::test]
    async fn test_scan_successful_connection() {
        // start a TCP server on available port
        let listener = TcpListener::bind("127.0.0.1:8880").await.unwrap();
        let addr = listener.local_addr().unwrap();

        // create testing channel
        let (tx, _rx) = channel();

        // connect using scan function
        let result = task::spawn(scan(
            tx,
            addr.port(),
            IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        ))
        .await;

        assert!(result.is_ok());
    }
}
