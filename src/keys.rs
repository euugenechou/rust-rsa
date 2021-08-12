#![allow(dead_code)]

use super::numtheory::*;
use num_bigint::{BigUint, RandBigInt};
use num_traits::One;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::io::prelude::*;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct RSAPubKey {
    n: BigUint,
    e: BigUint,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct RSAPrivKey {
    n: BigUint,
    d: BigUint,
}

pub fn new_keypair(bits: u64) -> (RSAPubKey, RSAPrivKey) {
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

    (RSAPubKey { n: n.clone(), e }, RSAPrivKey { n, d })
}

impl RSAPubKey {
    pub fn read<R: Read>(reader: &mut R) -> Result<Self, serde_json::Error> {
        serde_json::from_reader(reader)
    }

    pub fn write<W: Write>(writer: &mut W, pubkey: &Self) -> Result<(), serde_json::Error> {
        serde_json::to_writer(writer, pubkey)
    }

    pub fn encrypt(&self, m: &BigUint) -> BigUint {
        powermod(m, &self.e, &self.n)
    }
}

impl RSAPrivKey {
    pub fn read<R: Read>(reader: &mut R) -> Result<Self, serde_json::Error> {
        serde_json::from_reader(reader)
    }

    pub fn write<W: Write>(writer: &mut W, privkey: &Self) -> Result<(), serde_json::Error> {
        serde_json::to_writer(writer, privkey)
    }

    pub fn decrypt(&self, c: &BigUint) -> BigUint {
        powermod(c, &self.d, &self.n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;
    use std::io::{BufReader, BufWriter, Cursor};

    #[test]
    fn test_key_io() -> Result<(), Box<dyn Error>> {
        let (pubkey, privkey) = new_keypair(512);

        let mut buffer = Vec::new();
        let mut writer = BufWriter::new(Cursor::new(&mut buffer));
        RSAPubKey::write(&mut writer, &pubkey)?;
        drop(writer);

        let mut reader = BufReader::new(Cursor::new(&mut buffer));
        let keypub = RSAPubKey::read(&mut reader)?;
        assert_eq!(pubkey, keypub);

        let mut buffer = Vec::new();
        let mut writer = BufWriter::new(Cursor::new(&mut buffer));
        RSAPrivKey::write(&mut writer, &privkey)?;
        drop(writer);

        let mut reader = BufReader::new(Cursor::new(&mut buffer));
        let keypriv = RSAPrivKey::read(&mut reader)?;
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
}
