use std::net::{SocketAddr, SocketAddrV4};

use quick_error::quick_error;

pub type Result<T> = std::result::Result<T, Error>;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        JsonError(err: serde_json::Error) {
            from()
        }

        NetworkError(err: std::io::Error, addr: Option<SocketAddr>) {
            context(addr: Option<SocketAddr>, err: std::io::Error)
                -> (err, addr)
            context(addr: SocketAddr, err: std::io::Error)
                -> (err, Some(addr))
            context(addr: SocketAddrV4, err: std::io::Error)
                -> (err, Some(addr.into()))
        }
    }
}
