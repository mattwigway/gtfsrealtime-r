use std::fs;

use bytes::BytesMut;
use extendr_api::prelude::*;
use prost::Message as _;

use crate::transit_realtime::{
    feed_header::Incrementality, FeedEntity, FeedHeader, FeedMessage, VehiclePosition,
};

#[extendr]
pub fn test_data_duplicate_ids_positions(filename: &str) -> Result<()> {
    let msg = FeedMessage {
        header: FeedHeader {
            gtfs_realtime_version: "2.0".to_string(),
            incrementality: None,
            timestamp: None,
            feed_version: None,
        },
        entity: vec![
            FeedEntity {
                // extendr does not interpolate rust code into R by naively doing string manipulation,
                // but this just makes sure that IDs with R code in them aren't problematic.
                // https://xkcd.com/327/
                id: "\")); stop(\"identifier with r code executed!\")#".to_string(),
                is_deleted: None,
                trip_update: None,
                vehicle: Some(VehiclePosition {
                    trip: None,
                    vehicle: None,
                    position: None,
                    current_stop_sequence: None,
                    stop_id: None,
                    current_status: None,
                    timestamp: None,
                    congestion_level: None,
                    occupancy_status: None,
                    occupancy_percentage: None,
                    multi_carriage_details: vec![],
                }),
                alert: None,
                shape: None,
                stop: None,
                trip_modifications: None,
            },
            FeedEntity {
                id: "\")); stop(\"identifier with r code executed!\")#".to_string(),
                is_deleted: None,
                trip_update: None,
                vehicle: Some(VehiclePosition {
                    trip: None,
                    vehicle: None,
                    position: None,
                    current_stop_sequence: None,
                    stop_id: None,
                    current_status: None,
                    timestamp: None,
                    congestion_level: None,
                    occupancy_status: None,
                    occupancy_percentage: None,
                    multi_carriage_details: vec![],
                }),
                alert: None,
                shape: None,
                stop: None,
                trip_modifications: None,
            },
            // a third one to make sure
            FeedEntity {
                id: "\")); stop(\"identifier with r code executed!\")#".to_string(),
                is_deleted: None,
                trip_update: None,
                vehicle: Some(VehiclePosition {
                    trip: None,
                    vehicle: None,
                    position: None,
                    current_stop_sequence: None,
                    stop_id: None,
                    current_status: None,
                    timestamp: None,
                    congestion_level: None,
                    occupancy_status: None,
                    occupancy_percentage: None,
                    multi_carriage_details: vec![],
                }),
                alert: None,
                shape: None,
                stop: None,
                trip_modifications: None,
            },
        ],
    };

    let mut bytes = BytesMut::new();
    msg.encode(&mut bytes)
        .or(Err(Error::Other("encoding error".to_string())))?;
    fs::write(filename, &bytes).or(Err(Error::Other("fs error".to_string())))?;

    Ok(())
}

extendr_module! {
    mod duplicate_ids;
    fn test_data_duplicate_ids_positions;
}
