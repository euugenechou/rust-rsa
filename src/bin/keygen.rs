use clap::Clap;
use rust_rsa::{PrivKey, PubKey};
use std::error::Error;
use std::ffi::OsString;
use std::fs::OpenOptions;
use std::io::BufWriter;

#[derive(Clap)]
#[clap(name = "keygen")]
struct Opt {
    #[clap(long, default_value = "512")]
    bits: u64,

    #[clap(long, default_value = "/tmp/rsa.pub")]
    pbfile: OsString,

    #[clap(long, default_value = "/tmp/rsa.priv")]
    pvfile: OsString,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::parse();

    let pbfile = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(opt.pbfile)?;
    let mut pubwriter = BufWriter::new(pbfile);

    let pvfile = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(opt.pvfile)?;
    let mut privwriter = BufWriter::new(pvfile);

    let (pubkey, privkey) = rust_rsa::new_keypair(opt.bits);
    PubKey::write(&mut pubwriter, &pubkey)?;
    PrivKey::write(&mut privwriter, &privkey)?;

    Ok(())
}
