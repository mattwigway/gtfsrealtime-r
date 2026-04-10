use extendr_api::prelude::*;
use strum::VariantArray;

use crate::test_data::enum_roundtrip::get_or_none;
use crate::test_data::write_positions;
use crate::transit_realtime::{
    trip_descriptor,
    vehicle_descriptor::WheelchairAccessible,
    vehicle_position::{CongestionLevel, OccupancyStatus, VehicleStopStatus},
    Position, TripDescriptor, VehicleDescriptor, VehiclePosition,
};

#[extendr]
pub fn test_data_enum_roundtrip_positions(filename: &str) -> Result<List> {
    let n = [
        VehicleStopStatus::VARIANTS.len(),
        OccupancyStatus::VARIANTS.len(),
        CongestionLevel::VARIANTS.len(),
        trip_descriptor::ScheduleRelationship::VARIANTS.len(),
        WheelchairAccessible::VARIANTS.len(),
    ]
    .into_iter()
    .max()
    .unwrap()
        + 1;

    let values: Vec<VehiclePosition> = (0..n)
        .into_iter()
        .map(|i| {
            let mut p = VehiclePosition {
                trip: Some(TripDescriptor {
                    trip_id: Some("42".to_owned()),
                    route_id: Some("42".to_owned()),
                    direction_id: Some(0),
                    start_time: Some("06:00:00".to_owned()),
                    start_date: Some("20250101".to_owned()),
                    schedule_relationship: None,
                    modified_trip: None,
                }),
                vehicle: Some(VehicleDescriptor {
                    id: Some("42".to_owned()),
                    label: Some("42".to_owned()),
                    license_plate: Some("42".to_owned()),
                    wheelchair_accessible: None,
                }),
                position: Some(Position {
                    latitude: 37.363,
                    longitude: -122.123,
                    bearing: None,
                    odometer: None,
                    speed: None,
                }),
                current_stop_sequence: Some(3),
                stop_id: Some("42".to_owned()),
                current_status: None,
                timestamp: Some(1774967570),
                congestion_level: None,
                occupancy_status: None,
                occupancy_percentage: None,
                multi_carriage_details: vec![],
            };

            // do it this way rather than specifying above so types are enforced by the rust type system.
            match get_or_none::<trip_descriptor::ScheduleRelationship>(i) {
                Some(r) => p.trip.as_mut().unwrap().set_schedule_relationship(r),
                None => {}
            };

            match get_or_none::<WheelchairAccessible>(i) {
                Some(r) => p.vehicle.as_mut().unwrap().set_wheelchair_accessible(r),
                None => {}
            };

            match get_or_none::<VehicleStopStatus>(i) {
                Some(r) => p.set_current_status(r),
                None => {}
            };

            match get_or_none::<CongestionLevel>(i) {
                Some(r) => p.set_congestion_level(r),
                None => {}
            };

            match get_or_none::<OccupancyStatus>(i) {
                Some(r) => p.set_occupancy_status(r),
                None => {}
            };

            p
        })
        .collect();

    // get the correct values as strings to pass to R separately for validation
    let l = list!(
        schedule_relationship = values
            .iter()
            .map(|p| match p.trip.as_ref().unwrap().schedule_relationship {
                Some(_) => Some(
                    p.trip
                        .as_ref()
                        .unwrap()
                        .schedule_relationship()
                        .as_str_name()
                ),
                None => None,
            })
            .collect::<Vec<Option<&str>>>(),
        vehicle_wheelchair_accessible = values
            .iter()
            .map(
                |p| match p.vehicle.as_ref().unwrap().wheelchair_accessible {
                    Some(_) => Some(
                        p.vehicle
                            .as_ref()
                            .unwrap()
                            .wheelchair_accessible()
                            .as_str_name()
                    ),
                    None => None,
                }
            )
            .collect::<Vec<Option<&str>>>(),
        current_status = values
            .iter()
            .map(|p| match p.current_status {
                Some(_) => Some(p.current_status().as_str_name()),
                None => None,
            })
            .collect::<Vec<Option<&str>>>(),
        congestion_level = values
            .iter()
            .map(|p| match p.congestion_level {
                Some(_) => Some(p.congestion_level().as_str_name()),
                None => None,
            })
            .collect::<Vec<Option<&str>>>(),
        occupancy_status = values
            .iter()
            .map(|p| match p.occupancy_status {
                Some(_) => Some(p.occupancy_status().as_str_name()),
                None => None,
            })
            .collect::<Vec<Option<&str>>>()
    );

    write_positions(filename, values)?;

    Ok(l)
}

extendr_module! {
    mod enum_roundtrip_positions;
    fn test_data_enum_roundtrip_positions;
}
