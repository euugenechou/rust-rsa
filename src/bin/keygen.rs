use clap::Clap;
use rust_rsa::{RSAPrivKey, RSAPubKey};
use std::error::Error;
use std::ffi::OsString;
use std::fs::OpenOptions;
use std::io::BufWriter;

#[derive(Clap)]
#[clap(name = "keygen")]
struct Opt {
    #[clap(long, default_value = "512")]
    bits: u64,

    #[clap(long, default_value = "rsa.pub")]
    pbfile: OsString,

    #[clap(long, default_value = "rsa.priv")]
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
    RSAPubKey::write(&mut pubwriter, &pubkey)?;
    RSAPrivKey::write(&mut privwriter, &privkey)?;

    Ok(())
}