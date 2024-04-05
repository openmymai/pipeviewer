use clap::{arg, command, ArgAction};
use std::env;
use std::fs::File;
use std::io::{self, BufReader, BufWriter, ErrorKind, Read, Result, Write};

const CHUNK_SIZE: usize = 16 * 1024;

fn main() -> Result<()> {
    let matches = command!()
        .arg(arg!([infile]))
        .arg(arg!(-o --outfile <FILE> "Write output to a file instead of stdout"))
        .arg(arg!(-s - -silent).action(ArgAction::SetTrue))
        .get_matches();

    // let infile = matches.get_one::<String>("infile");
    let outfile = matches.get_one::<String>("outfile");
    let silent = if matches.get_flag("silent") {
        true
    } else {
        !env::var("PV_SILENT").unwrap_or_default().is_empty()
    };

    dbg!(outfile, silent);

    let mut reader: Box<dyn Read> = if matches.get_one::<String>("infile").is_some() {
        Box::new(BufReader::new(File::open(
            matches.get_one::<String>("infile").unwrap(),
        )?))
    } else {
        Box::new(io::stdin())
    };

    let mut writer: Box<dyn Write> = if matches.get_one::<String>("outfile").is_some() {
        Box::new(BufWriter::new(File::create(
            matches.get_one::<String>("outfile").unwrap(),
        )?))
    } else {
        Box::new(io::stdout())
    };

    let mut total_bytes = 0;
    let mut buffer = [0; CHUNK_SIZE];
    loop {
        let num_read = match reader.read(&mut buffer) {
            Ok(0) => break,
            Ok(x) => x,
            Err(_) => break,
        };
        total_bytes += num_read;
        if !silent {
            eprint!("\r{}", total_bytes);
        }
        if let Err(e) = writer.write_all(&buffer[..num_read]) {
            // [..x] -> [0..x] -> [..] all elements
            if e.kind() == ErrorKind::BrokenPipe {
                break;
            }
            return Err(e);
        }
        // io::stdout().write_all(&buffer[..num_read])?; // error handling short format
    }
    if !silent {
        eprintln!("\r{}", total_bytes);
    }

    Ok(())
}
