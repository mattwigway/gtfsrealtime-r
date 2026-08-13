use std::fs;

use bytes::BytesMut;
use extendr_api::error::Result;
use extendr_api::prelude::*;
use prost::Message;

use crate::transit_realtime::{feed_header::Incrementality, FeedHeader, FeedMessage};

#[extendr]
pub fn test_data_differential_feed(filename: &str) -> Result<()> {
    let msg = FeedMessage {
        header: FeedHeader {
            gtfs_realtime_version: "2.0".to_owned(),
            incrementality: Some(Incrementality::Differential as i32),
            timestamp: Some(1774967578),
            feed_version: None,
        },
        entity: vec![],
    };

    let mut bytes = BytesMut::new();
    msg.encode(&mut bytes)
        .or(Err(Error::Other("encoding error".to_string())))?;
    fs::write(filename, &bytes).or(Err(Error::Other("fs error".to_string())))?;

    Ok(())
}

extendr_module! {
    mod differential_feed;
    fn test_data_differential_feed;
}
