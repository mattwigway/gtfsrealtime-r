// code to create various GTFS-realtime datasets for use in tests
mod enum_roundtrip;
mod enum_roundtrip_alerts;
mod enum_roundtrip_positions;
mod enum_roundtrip_updates;
mod invalid_enum_positions;
mod trip_update_unwrapping;
mod positions_all_values;

use bytes::BytesMut;
use extendr_api::extendr_module;
use extendr_api::prelude::*;
use prost::Message;
use std::{fs, string::ToString};

use crate::transit_realtime::{
    self, feed_header::Incrementality, Alert, FeedEntity, FeedHeader, TripUpdate, VehiclePosition,
};

fn write_msg(
    filename: &str,
    positions: Vec<VehiclePosition>,
    alerts: Vec<Alert>,
    trip_updates: Vec<TripUpdate>,
) -> Result<()> {
    let mut pos_id = 0;
    let mut alert_id = 0;
    let mut upd_id = 0;

    let msg = transit_realtime::FeedMessage {
        header: FeedHeader {
            gtfs_realtime_version: "2.0".to_owned(),
            incrementality: Some(Incrementality::FullDataset as i32),
            timestamp: Some(1774967578),
            feed_version: None,
        },
        entity: positions
            .into_iter()
            .map(|x| {
                pos_id += 1;
                FeedEntity {
                    id: pos_id.to_string(),
                    is_deleted: None,
                    vehicle: Some(x),
                    trip_update: None,
                    alert: None,
                    shape: None,
                    stop: None,
                    trip_modifications: None,
                }
            })
            .chain(alerts.into_iter().map(|x| {
                alert_id += 1;
                FeedEntity {
                    id: alert_id.to_string(),
                    is_deleted: None,
                    vehicle: None,
                    trip_update: None,
                    alert: Some(x),
                    shape: None,
                    stop: None,
                    trip_modifications: None,
                }
            }))
            .chain(trip_updates.into_iter().map(|x| {
                upd_id += 1;
                FeedEntity {
                    id: upd_id.to_string(),
                    is_deleted: None,
                    vehicle: None,
                    trip_update: Some(x),
                    alert: None,
                    shape: None,
                    stop: None,
                    trip_modifications: None,
                }
            }))
            .collect(),
    };

    let mut bytes = BytesMut::new();
    msg.encode(&mut bytes)
        .or(Err(Error::Other("encoding error".to_string())))?;
    fs::write(filename, &bytes).or(Err(Error::Other("fs error".to_string())))?;
    Ok(())
}

fn write_positions(filename: &str, positions: Vec<VehiclePosition>) -> Result<()> {
    write_msg(filename, positions, vec![], vec![])
}

fn write_updates(filename: &str, updates: Vec<TripUpdate>) -> Result<()> {
    write_msg(filename, vec![], vec![], updates)
}

fn write_alerts(filename: &str, alerts: Vec<Alert>) -> Result<()> {
    write_msg(filename, vec![], alerts, vec![])
}

extendr_module! {
    mod test_data;
    use enum_roundtrip_positions;
    use enum_roundtrip_alerts;
    use enum_roundtrip_updates;
    use invalid_enum_positions;
    use trip_update_unwrapping;
    use positions_all_values;
}
