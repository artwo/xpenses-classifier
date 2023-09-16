use std::error::Error;
use std::fs::File;
use std::io::BufReader;

use encoding_rs::WINDOWS_1252;
use encoding_rs_io::{DecodeReaderBytes, DecodeReaderBytesBuilder};

pub fn open_encoded_file(
    file_path: &str,
) -> Result<BufReader<DecodeReaderBytes<File, Vec<u8>>>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let rdr = BufReader::new(
        DecodeReaderBytesBuilder::new()
            .encoding(Some(WINDOWS_1252))
            .build(file),
    );
    Ok(rdr)
}
