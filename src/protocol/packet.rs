extern crate atoi;
use binaryutils::binary_stream::{BinaryStream, BinaryStreamStruct};
use crate::utils::internet_address::InternetAddress;
use itoa::fmt;
use binaryutils::binary::Endian::{Little, Big};
use std::iter::FromIterator;
use std::string::Drain;
use atoi::atoi;

pub struct PacketStruct {
	binary_stream : BinaryStreamStruct,
	send_time : f32
}

impl PacketStruct {
	pub fn new(buffer : Vec<u8>, offset : usize) -> PacketStruct {
		return PacketStruct {
			binary_stream : BinaryStreamStruct::new(buffer, offset),
			send_time : -1 as f32
		}
	}
}

pub trait Packet : BinaryStream {
	const PACKET_ID : u8;
	fn get_packet_ref(&self) -> &PacketStruct;
	fn get_packet_mut(&mut self) -> &mut PacketStruct;
	fn get_binary_stream_ref(&self) -> &BinaryStreamStruct {
		return &self.get_packet_ref().binary_stream;
	}
	fn get_binary_stream_mut(&mut self) -> &mut BinaryStreamStruct {
		return &mut self.get_packet_mut().binary_stream;
	}
	fn get_string(&mut self) -> String {
		return String::from_utf8(self.get(self.get_short(Big) as usize)).unwrap();
	}
	fn get_address(&mut self) -> InternetAddress {
		let version : u8 = self.get_byte();
		let mut addr : String;
		if version == 4 {
			addr = String::new();
			for i in 0..3 {
				fmt(&mut addr, self.get_byte());
				addr.push('.');
			}
			fmt(&mut addr, self.get_byte());
			let port: u16 = self.get_unsigned_short(Big); // DIFF
			return InternetAddress::new(addr.as_str(), port, version);
		} /*
		TODO ipv6 : need inet_ntop
		else if version == 6 {
			self.get_short(Little); //Family, AF_INET6
			let port = self.get_short(Big);
			self.get_int(Big); //flow info
		}
		*/
		else {
			panic!("BinaryDataException : Unknown IPAddress version {}", version);
		}
	}
	fn put_string(&mut self, v : &str) {
		self.put_short(v.len() as i16, Big);
		self.put(Vec::from(v));
	}
	fn put_address(&mut self, address : &InternetAddress) {
		self.put_byte(address.get_version());
		if address.get_version() == 4 {
			let mut parts : Vec<u8> = Vec::new();
			for i in address.get_ip().split('.') {
				parts.push(atoi::<u8>((i as &str).as_bytes()).unwrap());
			}
			assert!(parts.len() == 4, "Wrong number of parts in IPv4, expected 4, got {}", parts.len());
		}
		/* TODO ipv6 */
		else {
			panic!("InvalidArgumentException : IP version {} is not supported", address.get_version());
		}
	}
	fn encode(&mut self) {
		self.reset();
		self.encode_header();
		self.encode_payload();
	}
	fn encode_header(&mut self) {
		self.put_byte(Self::PACKET_ID);
	}
	fn encode_payload(&mut self);
	fn decode(&mut self) {
		self.set_offset(0);
		self.decode_header();
		self.decode_payload();
	}
	fn decode_header(&mut self) {
		self.get_byte();
	}
	fn decode_payload(&mut self);
	fn clean(&mut self) -> &mut Self {
		self.reset();
		self.get_packet_mut().send_time = 0 as f32;
		return self;
	}
}