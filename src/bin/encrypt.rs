use clap::{App, Arg};
use rust_rsa::PubKey;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{prelude::*, stdin, stdout, BufReader, BufWriter};

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("encrypt")
        .arg(Arg::with_name("infile").long("infile").takes_value(true))
        .arg(Arg::with_name("outfile").long("outfile").takes_value(true))
        .arg(Arg::with_name("pbfile").long("pbfile").takes_value(true))
        .get_matches();

    let mut reader = BufReader::new(match matches.value_of("infile") {
        None => Box::new(stdin()) as Box<dyn Read>,
        Some(infile) => Box::new(File::open(infile)?) as Box<dyn Read>,
    });

    let mut writer = BufWriter::new(match matches.value_of("outfile") {
        None => Box::new(stdout()) as Box<dyn Write>,
        Some(outfile) => Box::new(
            OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(outfile)?,
        ) as Box<dyn Write>,
    });

    let pbfile = File::open(matches.value_of("pbfile").unwrap_or("/tmp/rsa.pub"))?;
    let mut pubreader = BufReader::new(pbfile);

    let pubkey = PubKey::read(&mut pubreader)?;
    pubkey.stream_encrypt(&mut reader, &mut writer)?;

    Ok(())
}
