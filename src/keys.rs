use super::numtheory::*;
use rug::{integer::Order, Integer};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::io::prelude::*;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct PubKey {
    pub n: Integer,
    pub e: Integer,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct PrivKey {
    n: Integer,
    d: Integer,
}

pub fn new_keypair(bits: u32) -> (PubKey, PrivKey) {
    let p = makeprime(bits / 2);
    let q = makeprime(bits / 2 + 1);
    let n = Integer::from(&p * &q);
    let totient = Integer::from(&p - 1) * Integer::from(&q - 1);

    let mut e = makeprime(bits / 2);
    while gcd(&e, &totient) != 1 {
        e = makeprime(bits / 2);
    }
    let d = inverse(&e, &totient).unwrap();

    (PubKey { n: n.clone(), e }, PrivKey { n, d })
}

impl PubKey {
    pub fn new(bits: u32) -> Self {
        let p = makeprime(bits / 2);
        let q = makeprime(bits / 2 + 1);
        let n = Integer::from(&p * &q);
        let totient = Integer::from(&p - 1) * Integer::from(&q - 1);

        let mut e = makeprime(bits / 2);
        while gcd(&e, &totient) != 1 {
            e = makeprime(bits / 2);
        }

        PubKey { n, e }
    }

    pub fn read<R: Read>(reader: &mut R) -> Result<Self, serde_json::Error> {
        serde_json::from_reader(reader)
    }

    pub fn write<W: Write>(writer: &mut W, pubkey: &Self) -> Result<(), serde_json::Error> {
        serde_json::to_writer(writer, pubkey)
    }

    pub fn encrypt(&self, m: &Integer) -> Integer {
        powermod(m, &self.e, &self.n)
    }

    pub fn stream_encrypt<R: Read, W: Write>(
        &self,
        mut reader: R,
        mut writer: W,
    ) -> Result<(), Box<dyn Error>> {
        let blocksize = (((self.n.significant_bits() - 1) / 8) - 1) as usize;
        let mut buffer = vec![0xFF; blocksize];

        while let Ok(bytes) = reader.read(&mut buffer[1..]) {
            if bytes == 0 {
                break;
            }

            let m = Integer::from_digits::<u8>(&buffer[..bytes + 1], Order::MsfBe);
            let c = self.encrypt(&m);
            serde_json::to_writer(&mut writer, &c)?;
        }

        Ok(())
    }
}

impl PrivKey {
    pub fn read<R: Read>(reader: &mut R) -> Result<Self, serde_json::Error> {
        serde_json::from_reader(reader)
    }

    pub fn write<W: Write>(writer: &mut W, privkey: &Self) -> Result<(), serde_json::Error> {
        serde_json::to_writer(writer, privkey)
    }

    pub fn decrypt(&self, c: &Integer) -> Integer {
        powermod(c, &self.d, &self.n)
    }

    pub fn stream_decrypt<R: Read, W: Write>(
        &self,
        mut reader: R,
        mut writer: W,
    ) -> Result<(), Box<dyn Error>> {
        while let Ok(c) = serde_json::from_reader(&mut reader) {
            let m = self.decrypt(&c);
            let bytes = m.to_digits::<u8>(Order::MsfBe);
            writer.write_all(&bytes[1..])?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{BufReader, BufWriter, Cursor};

    #[test]
    fn test_key_io() -> Result<(), Box<dyn Error>> {
        let (pubkey, privkey) = new_keypair(4096);

        let mut buffer = Vec::new();
        let mut writer = BufWriter::new(Cursor::new(&mut buffer));
        PubKey::write(&mut writer, &pubkey)?;
        drop(writer);

        let mut reader = BufReader::new(Cursor::new(&mut buffer));
        let keypub = PubKey::read(&mut reader)?;
        assert_eq!(pubkey, keypub);

        let mut buffer = Vec::new();
        let mut writer = BufWriter::new(Cursor::new(&mut buffer));
        PrivKey::write(&mut writer, &privkey)?;
        drop(writer);

        let mut reader = BufReader::new(Cursor::new(&mut buffer));
        let keypriv = PrivKey::read(&mut reader)?;
        assert_eq!(privkey, keypriv);

        Ok(())
    }

    #[test]
    fn test_encrypt_decrypt() -> Result<(), Box<dyn Error>> {
        let (pubkey, privkey) = new_keypair(4096);

        let mut data = b"hello world";
        let mut encrypted = Vec::new();
        let mut reader = BufReader::new(Cursor::new(&mut data));
        let mut writer = BufWriter::new(Cursor::new(&mut encrypted));
        pubkey.stream_encrypt(&mut reader, &mut writer)?;
        drop(writer);

        let mut data = Vec::new();
        let mut reader = BufReader::new(Cursor::new(&mut encrypted));
        let mut writer = BufWriter::new(Cursor::new(&mut data));
        privkey.stream_decrypt(&mut reader, &mut writer)?;
        drop(writer);

        assert_eq!(data, b"hello world");
        Ok(())
    }
}
