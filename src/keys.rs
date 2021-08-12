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

pub fn new_keypair(bits: u64, iters: u64) -> (RSAPubKey, RSAPrivKey) {
    let mut rng = rand::thread_rng();
    let pbits = rng.gen_range((bits / 4)..(3 * bits / 4));
    let qbits = bits - pbits;

    let p = makeprime(pbits, iters);
    let q = makeprime(qbits, iters);
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
}

impl RSAPrivKey {
    pub fn read<R: Read>(reader: &mut R) -> Result<Self, serde_json::Error> {
        serde_json::from_reader(reader)
    }

    pub fn write<W: Write>(writer: &mut W, privkey: &Self) -> Result<(), serde_json::Error> {
        serde_json::to_writer(writer, privkey)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;
    use std::io::{BufReader, BufWriter, Cursor};

    #[test]
    fn test_pubkey_io() -> Result<(), Box<dyn Error>> {
        let mut buffer = Vec::new();
        let (pubkey, _) = new_keypair(256, 50);

        let mut writer = BufWriter::new(Cursor::new(&mut buffer));
        RSAPubKey::write(&mut writer, &pubkey)?;
        drop(writer);

        let mut reader = BufReader::new(Cursor::new(&mut buffer));
        let keypub = RSAPubKey::read(&mut reader)?;

        assert_eq!(pubkey, keypub);
        Ok(())
    }

    #[test]
    fn test_privkey_io() -> Result<(), Box<dyn Error>> {
        let mut buffer = Vec::new();
        let (_, privkey) = new_keypair(256, 50);

        let mut writer = BufWriter::new(Cursor::new(&mut buffer));
        RSAPrivKey::write(&mut writer, &privkey)?;
        drop(writer);

        let mut reader = BufReader::new(Cursor::new(&mut buffer));
        let keypriv = RSAPrivKey::read(&mut reader)?;

        assert_eq!(privkey, keypriv);
        Ok(())
    }
}
