use derive::Key;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Serialize, Deserialize, Key)]
pub struct Nt<'a> {
	__: u8,
	_a: u8,
	pub ns: &'a str,
	_b: u8,
	_c: u8,
	_d: u8,
	pub tk: &'a str,
}

pub fn new<'a>(ns: &'a str, tk: &'a str) -> Nt<'a> {
	Nt::new(ns, tk)
}

pub fn prefix(ns: &str) -> Vec<u8> {
	let mut k = super::namespace::new(ns).encode().unwrap();
	k.extend_from_slice(&[b'!', b'n', b't', 0x00]);
	k
}

pub fn suffix(ns: &str) -> Vec<u8> {
	let mut k = super::namespace::new(ns).encode().unwrap();
	k.extend_from_slice(&[b'!', b'n', b't', 0xff]);
	k
}

impl<'a> Nt<'a> {
	pub fn new(ns: &'a str, tk: &'a str) -> Self {
		Self {
			__: b'/',
			_a: b'*',
			ns,
			_b: b'!',
			_c: b'n',
			_d: b't',
			tk,
		}
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn key() {
		use super::*;
		#[rustfmt::skip]
		let val = Nt::new(
			"testns",
			"testtk",
		);
		let enc = Nt::encode(&val).unwrap();
		assert_eq!(enc, b"/*testns\0!nttesttk\0");
		let dec = Nt::decode(&enc).unwrap();
		assert_eq!(val, dec);
	}
}
