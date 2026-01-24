use std::{fs::File, io::Read};

use bytes::BytesMut;
use bzip2::read::BzDecoder;
use flate2::read::GzDecoder;
use prost::Message;

use crate::transit_realtime::FeedMessage;

const BZIP2_MAGIC_BYTES: [u8; 3] = [b'B', b'Z', b'h'];
const GZIP_MAGIC_BYTES: [u8; 3] = [0x1f, 0x8b, 0x08];

pub fn read_feed(path: String) -> Result<FeedMessage, Box<dyn std::error::Error>> {
    let mut buf = Vec::new();

    if path.starts_with("https://") || path.starts_with("http://") {
        ureq::get(path)
            .header("User-Agent", "R gtfsrealtime")
            .call()?
            .body_mut()
            .as_reader()
            .read_to_end(&mut buf)?;
    } else {
        // treat as local file
        File::open(path)?
            .read_to_end(&mut buf)?;
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