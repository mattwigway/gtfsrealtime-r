use std::{fs::File, io::Read};

use bytes::BytesMut;
use prost::Message;

use crate::transit_realtime::FeedMessage;

pub fn read_feed(path: String) -> Result<FeedMessage, Box<dyn std::error::Error>> {
    let mut file = File::open(path)?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;
    let byt = BytesMut::from(buf.as_slice());
    let msg = FeedMessage::decode(byt)?;
    return Ok(msg);
}