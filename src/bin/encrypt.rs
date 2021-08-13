use clap::Clap;
use rust_rsa::PubKey;
use std::error::Error;
use std::ffi::OsString;
use std::fs::{File, OpenOptions};
use std::io::{prelude::*, stdin, stdout, BufReader, BufWriter};

#[derive(Clap)]
#[clap(name = "encrypt")]
struct Opt {
    infile: Option<OsString>,
    outfile: Option<OsString>,

    #[clap(long, default_value = "/tmp/rsa.pub")]
    pbfile: OsString,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::parse();

    let mut reader = BufReader::new(match opt.infile {
        None => Box::new(stdin()) as Box<dyn Read>,
        Some(infile) => Box::new(File::open(infile)?) as Box<dyn Read>,
    });

    let mut writer = BufWriter::new(match opt.outfile {
        None => Box::new(stdout()) as Box<dyn Write>,
        Some(outfile) => Box::new(
            OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(outfile)?,
        ) as Box<dyn Write>,
    });

    let pbfile = File::open(opt.pbfile)?;
    let mut pubreader = BufReader::new(pbfile);

    let pubkey = PubKey::read(&mut pubreader)?;
    pubkey.stream_encrypt(&mut reader, &mut writer)?;

    Ok(())
}
