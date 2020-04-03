
use std::net::SocketAddr;
use std::path::Path;
use crate::{
	Result,
	aerr
};
use super::tcpserver::TcpServer;
use super::unixserver::UnixServer;
use super::Server;

pub fn server_from_address(typename: &str, text: &str) -> Result<Box<dyn Server>> {
	match typename {
		"inet" => Ok(Box::new(TcpServer::new(&text.parse()?)?)),
		"unix" => Ok(Box::new(UnixServer::new(Path::new(text))?)),
		"abstract" => Ok(Box::new(UnixServer::new(Path::new(&format!("\0{}", text)))?)),
		_ => Err(aerr!("Invalid address type"))
	}
}
