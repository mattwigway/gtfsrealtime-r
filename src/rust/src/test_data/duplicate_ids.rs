use std::fs;

use bytes::BytesMut;
use extendr_api::prelude::*;
use prost::Message as _;

use crate::transit_realtime::{
    trip_update::StopTimeUpdate, Alert, FeedEntity, FeedHeader,
    FeedMessage, TimeRange, TripDescriptor, TripUpdate, VehiclePosition,
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
                id: ")); stop(\"identifier with r code executed!\")#".to_string(),
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
                id: ")); stop(\"identifier with r code executed!\")#".to_string(),
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
                id: ")); stop(\"identifier with r code executed!\")#".to_string(),
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

/// Feed containing three trip updates.
/// "id" is a trip update with no stop time updates
/// "id" is another trip update with the same ID, with two stop time updates
/// "id2" is another trip update with no stop time updates
/// @param filename filename to save the feed
/// @keywords internal
#[extendr]
pub fn test_data_duplicate_ids_updates(filename: &str) -> Result<()> {
    let msg = FeedMessage {
        header: FeedHeader {
            gtfs_realtime_version: "2.0".to_string(),
            incrementality: None,
            timestamp: None,
            feed_version: None,
        },
        entity: vec![
            FeedEntity {
                id: "id".to_string(),
                is_deleted: None,
                trip_update: Some(TripUpdate {
                    trip: TripDescriptor {
                        trip_id: None,
                        route_id: None,
                        direction_id: None,
                        start_time: None,
                        start_date: None,
                        schedule_relationship: None,
                        modified_trip: None,
                    },
                    vehicle: None,
                    stop_time_update: vec![],
                    timestamp: None,
                    delay: None,
                    trip_properties: None,
                }),
                vehicle: None,
                alert: None,
                shape: None,
                stop: None,
                trip_modifications: None,
            },
            FeedEntity {
                id: "id".to_string(),
                is_deleted: None,
                trip_update: Some(TripUpdate {
                    trip: TripDescriptor {
                        trip_id: None,
                        route_id: None,
                        direction_id: None,
                        start_time: None,
                        start_date: None,
                        schedule_relationship: None,
                        modified_trip: None,
                    },
                    vehicle: None,
                    stop_time_update: vec![
                        StopTimeUpdate {
                            stop_sequence: None,
                            stop_id: None,
                            arrival: None,
                            departure: None,
                            departure_occupancy_status: None,
                            schedule_relationship: None,
                            stop_time_properties: None,
                        },
                        StopTimeUpdate {
                            stop_sequence: None,
                            stop_id: None,
                            arrival: None,
                            departure: None,
                            departure_occupancy_status: None,
                            schedule_relationship: None,
                            stop_time_properties: None,
                        },
                    ],
                    timestamp: None,
                    delay: None,
                    trip_properties: None,
                }),
                vehicle: None,
                alert: None,
                shape: None,
                stop: None,
                trip_modifications: None,
            },
            FeedEntity {
                id: "id2".to_string(),
                is_deleted: None,
                trip_update: Some(TripUpdate {
                    trip: TripDescriptor {
                        trip_id: None,
                        route_id: None,
                        direction_id: None,
                        start_time: None,
                        start_date: None,
                        schedule_relationship: None,
                        modified_trip: None,
                    },
                    vehicle: None,
                    stop_time_update: vec![],
                    timestamp: None,
                    delay: None,
                    trip_properties: None,
                }),
                vehicle: None,
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

/// Write an alerts feed with duplicate IDs
/// the first one is "id" and will not be expanded
/// the second one is also id and has two TimeRanges so will be expanded
/// @param filename filename to write feed to
/// @keywords internal
#[extendr]
pub fn test_data_duplicate_ids_alerts(filename: &str) -> Result<()> {
    let msg = FeedMessage {
        header: FeedHeader {
            gtfs_realtime_version: "2.0".to_string(),
            incrementality: None,
            timestamp: None,
            feed_version: None,
        },
        entity: vec![
            FeedEntity {
                id: "id".to_string(),
                is_deleted: None,
                trip_update: None,
                vehicle: None,
                alert: Some(Alert {
                    active_period: vec![],
                    informed_entity: vec![],
                    cause: None,
                    effect: None,
                    url: None,
                    header_text: None,
                    description_text: None,
                    tts_header_text: None,
                    tts_description_text: None,
                    severity_level: None,
                    image: None,
                    image_alternative_text: None,
                    cause_detail: None,
                    effect_detail: None,
                }),
                shape: None,
                stop: None,
                trip_modifications: None,
            },
            FeedEntity {
                id: "id".to_string(),
                is_deleted: None,
                trip_update: None,
                vehicle: None,
                alert: Some(Alert {
                    active_period: vec![
                        TimeRange {
                            start: Some(1775222385),
                            end: Some(1775223385),
                        },
                        TimeRange {
                            start: Some(1775252385),
                            end: Some(1775253385),
                        },
                    ],
                    informed_entity: vec![],
                    cause: None,
                    effect: None,
                    url: None,
                    header_text: None,
                    description_text: None,
                    tts_header_text: None,
                    tts_description_text: None,
                    severity_level: None,
                    image: None,
                    image_alternative_text: None,
                    cause_detail: None,
                    effect_detail: None,
                }),
                shape: None,
                stop: None,
                trip_modifications: None,
            },
            FeedEntity {
                id: "id2".to_string(),
                is_deleted: None,
                trip_update: None,
                vehicle: None,
                alert: Some(Alert {
                    active_period: vec![],
                    informed_entity: vec![],
                    cause: None,
                    effect: None,
                    url: None,
                    header_text: None,
                    description_text: None,
                    tts_header_text: None,
                    tts_description_text: None,
                    severity_level: None,
                    image: None,
                    image_alternative_text: None,
                    cause_detail: None,
                    effect_detail: None,
                }),
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
    fn test_data_duplicate_ids_updates;
    fn test_data_duplicate_ids_alerts;
}
