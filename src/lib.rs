#![warn(clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]

mod error;
mod types;

use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

use quick_error::ResultExt;
use tokio::net::UdpSocket;

use crate::error::Result;
pub use crate::{error::Error, types::Notification};

pub const DEFAULT_ADDR: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
pub const DEFAULT_PORT: u16 = 42069;

pub struct XSOverlayNotifier {
    socket: UdpSocket,
}

impl XSOverlayNotifier {
    pub async fn new() -> Result<Self> {
        Self::with_addr(SocketAddrV4::new(DEFAULT_ADDR, DEFAULT_PORT).into()).await
    }

    pub async fn with_addr(socket_addr: SocketAddr) -> Result<Self> {
        let bind_addr = SocketAddrV4::new(DEFAULT_ADDR, 0);
        let socket = UdpSocket::bind(&bind_addr).await.context(bind_addr)?;

        socket.connect(&socket_addr).await.context(socket_addr)?;

        Ok(Self { socket })
    }

    pub async fn send(&self, notification: &Notification) -> Result<()> {
        let vec = serde_json::to_vec(notification)?;

        self.socket
            .send(&vec)
            .await
            .context(self.socket.peer_addr().ok())?;

        Ok(())
    }
}
