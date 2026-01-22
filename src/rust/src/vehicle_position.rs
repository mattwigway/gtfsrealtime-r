use extendr_api::prelude::*;
use crate::{read::read_feed, transit_realtime::vehicle_position::{CongestionLevel, OccupancyStatus, VehicleStopStatus}};

#[derive(IntoDataFrameRow, Debug, PartialEq)]
pub struct RVehiclePosition {
    // position
    latitude: Option<f32>,
    longitude: Option<f32>,
    bearing: Option<f32>,
    odometer: Option<f64>,
    speed: Option<f32>,

    // trip
    trip_id: Option<String>,
    route_id: Option<String>,
    direction_id: Option<u32>,
    start_time: Option<String>,
    start_date: Option<String>,
    schedule_relationship: Option<i32>,

    // stop
    stop_id: Option<String>,
    current_stop_sequence: Option<u32>,
    current_status: Option<i32>,

    // misc
    timestamp: Option<u64>,
    congestion_level: Option<i32>,
    occupancy_status: Option<i32>,
    occupancy_percentage: Option<u32>,

    // TODO multi carriage details - does not map well to tabular format

    // vehicle
    vehicle_id: Option<String>,
    vehicle_label: Option<String>,
    vehicle_license_plate: Option<String>
}

/// Read GTFS-RT vehicle positions
#[extendr]
pub fn read_gtfsrt_positions_internal(file: String) -> Result<Dataframe<RVehiclePosition>> {
    let msg = read_feed(file)?;
    let content: Vec<RVehiclePosition> = msg.entity.iter()
        .filter(|entity| entity.vehicle.is_some())
        .map(|entity| {
            let veh = entity.vehicle.as_ref().unwrap();
            let trip = veh.trip.as_ref();

            RVehiclePosition {
                latitude: veh.position.as_ref().map_or(None, |pos| Some(pos.latitude)),
                longitude: veh.position.as_ref().map_or(None, |pos| Some(pos.longitude)),
                bearing: veh.position.as_ref().map_or(None, |pos| pos.bearing),
                odometer: veh.position.as_ref().map_or(None, |pos| pos.odometer),
                speed: veh.position.as_ref().map_or(None, |pos| pos.speed),

                trip_id: trip.map_or(None, |t| t.trip_id.clone()),
                route_id: trip.map_or(None, |t| t.route_id.clone()),
                direction_id: trip.map_or(None, |t| t.direction_id),
                start_time: trip.map_or(None, |t| t.start_time.clone()),
                start_date: trip.map_or(None, |t| t.start_date.clone()),
                schedule_relationship: trip.map_or(None, |t| t.schedule_relationship),

                // stop
                stop_id: veh.stop_id.clone(),
                current_stop_sequence: veh.current_stop_sequence,
                current_status: veh.current_status,

                // misc
                timestamp: veh.timestamp,
                congestion_level: veh.congestion_level,
                occupancy_status: veh.occupancy_status,
                occupancy_percentage: veh.occupancy_percentage,

                vehicle_id: veh.vehicle.as_ref().map_or(None, |veh| veh.id.clone()),
                vehicle_label: veh.vehicle.as_ref().map_or(None, |veh| veh.label.clone()),
                vehicle_license_plate: veh.vehicle.as_ref().map_or(None, |veh| veh.license_plate.clone())
            }
        })
        .collect();

    return content.into_dataframe();
}

extendr_module! {
    mod gtfsrealtime;
    fn read_gtfsrt_positions_internal;
}