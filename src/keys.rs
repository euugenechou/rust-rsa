use super::numtheory::*;
use num_bigint::{BigUint, RandBigInt};
use num_traits::One;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::io::prelude::*;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct PubKey {
    n: BigUint,
    e: BigUint,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct PrivKey {
    n: BigUint,
    d: BigUint,
}

pub fn new_keypair(bits: u64) -> (PubKey, PrivKey) {
    let mut rng = rand::thread_rng();
    let pbits = rng.gen_range((bits / 4)..(3 * bits / 4));
    let qbits = bits - pbits;

    let p = makeprime(pbits);
    let q = makeprime(qbits);
    let n = &p * &q;
    let totient = (&p - 1u8) * (&q - 1u8);

    let mut e = rng.gen_biguint(bits);
    while gcd(&e, &totient) != One::one() {
        e = rng.gen_biguint(bits);
    }
    let d = inverse(&e, &totient).unwrap();

    (PubKey { n: n.clone(), e }, PrivKey { n, d })
}

impl PubKey {
    pub fn read<R: Read>(reader: &mut R) -> Result<Self, serde_json::Error> {
        serde_json::from_reader(reader)
    }

    pub fn write<W: Write>(writer: &mut W, pubkey: &Self) -> Result<(), serde_json::Error> {
        serde_json::to_writer(writer, pubkey)
    }

    pub fn encrypt(&self, m: &BigUint) -> BigUint {
        powermod(m, &self.e, &self.n)
    }

    pub fn stream_encrypt<R: Read, W: Write>(
        &self,
        mut reader: R,
        mut writer: W,
    ) -> Result<(), Box<dyn Error>> {
        let blocksize = (((self.n.bits() - 1) / 8) - 1) as usize;
        let mut buffer = vec![0xFF; blocksize];

        while let Ok(bytes) = reader.read(&mut buffer[1..]) {
            if bytes == 0 {
                break;
            }

            let m = BigUint::from_bytes_le(&buffer[..bytes + 1]);
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

    pub fn decrypt(&self, c: &BigUint) -> BigUint {
        powermod(c, &self.d, &self.n)
    }

    pub fn stream_decrypt<R: Read, W: Write>(
        &self,
        mut reader: R,
        mut writer: W,
    ) -> Result<(), Box<dyn Error>> {
        while let Ok(c) = serde_json::from_reader(&mut reader) {
            let m = self.decrypt(&c);
            let bytes = m.to_bytes_le();
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
        let (pubkey, privkey) = new_keypair(512);

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
    fn test_encrypt_decrypt() {
        let mut rng = rand::thread_rng();
        let (pubkey, privkey) = new_keypair(512);

        for _ in 0..10 {
            let m = rng.gen_biguint(256);
            assert_eq!(privkey.decrypt(&pubkey.encrypt(&m)), m);
        }
    }

    #[test]
    fn test_stream_encrypt_decrypt() -> Result<(), Box<dyn Error>> {
        let (pubkey, privkey) = new_keypair(512);

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
