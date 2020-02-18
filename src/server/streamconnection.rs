

use std::io;
use std::io::{Read, Write};
use std::mem::transmute;


pub struct StreamConnection<T: Read+Write> {
	pub stream: T,
	buffer: Vec<u8>
}

impl <T: Read+Write> StreamConnection<T> {
	
	pub fn new(stream: T) -> StreamConnection<T> {
		StreamConnection {
			stream,
			buffer: Vec::new()
		}
	}
	
	pub fn read(&mut self) -> Result<(Vec<String>, bool), io::Error> {
		let mut buf = [0; 2048];
		let mut closed = false;
		loop {
			match self.stream.read(&mut buf) {
				Err(e) => {
					if e.kind() == io::ErrorKind::WouldBlock {
						break;
					} else {
						return Err(e);
					}
				}
				Ok(0) => {
					closed = true;
					break;
				}
				Ok(i) => {
					self.buffer.extend_from_slice(&buf[..i]);
// 					messages.push(String::from_utf8_lossy(&buf[..i]).to_string());
				}
			}
		}
		let mut messages = Vec::new();
		while self.buffer.len() >= 4 {
			let mut header: [u8; 4] = [0;4];
			header.copy_from_slice(&self.buffer[..4]);
			let mlen: usize = u32::from_be(unsafe { transmute(header) }) as usize;
			if self.buffer.len() - 4 < mlen {
				break;
			}
			let rest = self.buffer.split_off(4+mlen);
			let message = String::from_utf8_lossy(&self.buffer[4..]).to_string();
			messages.push(message);
			self.buffer = rest;
		}
		Ok((messages, closed))
	}
	
	pub fn send(&mut self, text: &str) -> Result<(), io::Error> {
		let bytes: &[u8] = text.as_bytes();
		let len: u32 = bytes.len() as u32;
		let header: [u8; 4] = unsafe { transmute(len.to_be()) };
		self.stream.write_all(&header)?;
		self.stream.write_all(bytes)
	}
	
}
