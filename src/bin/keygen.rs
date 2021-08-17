use clap::{value_t, App, Arg};
use rust_rsa::{PrivKey, PubKey};
use std::error::Error;
use std::fs::OpenOptions;
use std::io::BufWriter;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("keygen")
        .arg(Arg::with_name("bits").long("bits").takes_value(true))
        .arg(Arg::with_name("pbfile").long("pbfile").takes_value(true))
        .arg(Arg::with_name("pvfile").long("pvfile").takes_value(true))
        .get_matches();

    let pbfile = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(matches.value_of("pbfile").unwrap_or("/tmp/rsa.pub"))?;
    let mut pubwriter = BufWriter::new(pbfile);

    let pvfile = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(matches.value_of("pvfile").unwrap_or("tmp/rsa.priv"))?;
    let mut privwriter = BufWriter::new(pvfile);

    let bits = value_t!(matches, "bits", u64).unwrap_or(512);
    let (pubkey, privkey) = rust_rsa::new_keypair(bits);
    PubKey::write(&mut pubwriter, &pubkey)?;
    PrivKey::write(&mut privwriter, &privkey)?;

    Ok(())
}
