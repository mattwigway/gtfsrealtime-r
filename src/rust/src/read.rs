use std::{
    fs::File,
    io::{Read, Seek},
};

use bytes::BytesMut;
use bzip2::read::BzDecoder;
use extendr_api::prelude::*;
use flate2::read::GzDecoder;
use prost::Message;
use zip::ZipArchive;

use crate::transit_realtime::FeedMessage;

const BZIP2_MAGIC_BYTES: [u8; 3] = [b'B', b'Z', b'h'];
const GZIP_MAGIC_BYTES: [u8; 3] = [0x1f, 0x8b, 0x08];
const ZIP_MAGIC_BYTES: [u8; 4] = [b'P', b'K', 0x03, 0x04];

pub fn read_feed(
    path: String,
) -> std::result::Result<Vec<FeedMessage>, Box<dyn std::error::Error>> {
    if path.starts_with("https://") || path.starts_with("http://") {
        let mut buf: Vec<u8> = Vec::new();

        ureq::get(path)
            .header("User-Agent", "R gtfsrealtime")
            .call()?
            .body_mut()
            .as_reader()
            .read_to_end(&mut buf)?;

        // we don't support reading zip files of multiple days over the internet
        Ok(vec![read_one_feed(buf)?])
    } else {
        // check if it's a zip file
        let mut header = [0u8; 4];
        let mut file = File::open(path)?;
        file.read_exact(&mut header)?;
        file.seek(std::io::SeekFrom::Start(0))?;

        if header == ZIP_MAGIC_BYTES {
            // try to read every element of the zip file
            let mut archive = ZipArchive::new(file)?;

            let mut messages: Vec<FeedMessage> = Vec::new();

            for idx in 0..archive.len() {
                let mut entry = archive.by_index(idx)?;
                if entry.is_file() {
                    let mut buf: Vec<u8> = Vec::new();
                    entry.read_to_end(&mut buf)?;

                    match read_one_feed(buf) {
                        Ok(msg) => messages.push(msg),
                        Err(e) => {
                            // don't warn about weird apple files
                            if !entry.name().ends_with(".DS_Store") {
                                R!(r#"
                                    cli::cli_warn(c(
                                        "!" = paste("Failed to read", {{ entry.name() }}),
                                        "x" = {{ e.to_string() }},
                                        "i" = "This file will be skipped"
                                    ))
                                "#)?;
                            }
                        }
                    };
                }
            }

            return Ok(messages);
        } else {
            let mut buf: Vec<u8> = Vec::new();
            file.read_to_end(&mut buf)?;

            // single file
            Ok(vec![read_one_feed(buf)?])
        }
    }
}

fn read_one_feed(mut buf: Vec<u8>) -> std::result::Result<FeedMessage, Box<dyn std::error::Error>> {
    if buf.len() < 3 {
        return Err(Box::new(extendr_api::Error::Other(
            "File is not a GTFS realtime feed".to_string(),
        )));
    }

    let first_three = &buf[..3];

    // transparently handle Bzipped or Gzipped data
    if first_three == BZIP2_MAGIC_BYTES {
        let mut buf2 = Vec::new();
        let mut decompressor = BzDecoder::new(&buf[..]);
        decompressor.read_to_end(&mut buf2)?;
        buf = buf2;
    } else if first_three == GZIP_MAGIC_BYTES {
        let mut buf2 = Vec::new();
        let mut decompressor = GzDecoder::new(&buf[..]);
        decompressor.read_to_end(&mut buf2)?;
        buf = buf2;
    }

    let byt = BytesMut::from(buf.as_slice());
    let msg = FeedMessage::decode(byt)?;
    return Ok(msg);
}
