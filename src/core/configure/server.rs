use serde::Deserialize;
use std::net::{AddrParseError, SocketAddr};

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub addr: String,
    pub port: u16,
}

impl ServerConfig {
    pub fn get_addr(&self) -> String {
        format!("{}:{}", self.addr, self.port)
    }

    pub fn get_http_addr(&self) -> String {
        format!("http://{}:{}", self.addr, self.port)
    }
    pub fn get_socket_addr(&self) -> Result<SocketAddr, AddrParseError> {
        self.get_addr().parse()
    }
}
