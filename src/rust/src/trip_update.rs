use extendr_api::prelude::*;

use crate::read::read_feed;

// GTFS-realtime uses a hierarchical trip update -> stop time update 
#[derive(IntoDataFrameRow, PartialEq, Debug)]
pub struct RStopTimeUpdate {
    feed_index: i32,

    trip_id: Option<String>,
    route_id: Option<String>,
    direction_id: Option<u32>,
    start_time: Option<String>,
    start_date: Option<String>,
    trip_schedule_relationship: Option<i32>,
    modifications_id: Option<String>,

    // vehicle info
    vehicle_id: Option<String>,
    vehicle_label: Option<String>,
    license_plate: Option<String>,
    wheelchair_accessible: Option<i32>,

    // stop time updates, exploded
    stop_sequence: Option<u32>,
    stop_id: Option<String>,
    arrival_delay: Option<i32>,
    arrival_time: Option<i64>,
    arrival_scheduled_time: Option<i64>,
    arrival_uncertainty: Option<i32>,

    departure_delay: Option<i32>,
    departure_time: Option<i64>,
    departure_scheduled_time: Option<i64>,
    departure_uncertainty: Option<i32>,

    departure_occupancy_status: Option<i32>,
    stop_schedule_relationship: Option<i32>
}


/// Read GTFS-RT trip updates (result is expanded to one row per stop update, group by
#[extendr]
pub fn read_gtfsrt_trip_updates_internal(file: String) -> Result<Dataframe<RStopTimeUpdate>> {
    let msg = read_feed(file)?;

    let content: Vec<RStopTimeUpdate> = msg.entity.iter()
        .filter(|entity| entity.trip_update.is_some())
        .enumerate()
        .map(|(index, entity)| {
            // todo handle empty stop time updates
            let upd = entity.trip_update.as_ref().unwrap();

            // rust complains about moving modified_trip etc if we put these in the RStopTimeUpdate directly
            let modifications_id = upd.trip.modified_trip.as_ref()
                .map_or(None, |m| m.modifications_id.clone());

            let veh = upd.vehicle.as_ref();
            let vehicle_id = veh.map_or(None, |v| v.id.clone());
            let vehicle_label = veh.map_or(None, |v| v.label.clone());
            let license_plate = veh.map_or(None, |v| v.license_plate.clone());
            let wheelchair_accessible = veh.map_or(None, |v| v.wheelchair_accessible);

            if !upd.stop_time_update.is_empty() {
                    upd.stop_time_update.iter().map(|stupd| {
                    let arr = stupd.arrival.as_ref();
                    let dep = stupd.departure.as_ref();

                    RStopTimeUpdate {
                        feed_index: index as i32, 
                        trip_id: upd.trip.trip_id.clone(),
                        route_id: upd.trip.route_id.clone(),
                        direction_id: upd.trip.direction_id,
                        start_time: upd.trip.start_time.clone(),
                        start_date: upd.trip.start_date.clone(),
                        trip_schedule_relationship: upd.trip.schedule_relationship,
                        modifications_id: modifications_id.clone(),
                        vehicle_id: vehicle_id.clone(),
                        vehicle_label: vehicle_label.clone(),
                        license_plate: license_plate.clone(),
                        wheelchair_accessible: wheelchair_accessible,
                        stop_sequence: stupd.stop_sequence,
                        stop_id: stupd.stop_id.clone(),
                        arrival_delay: arr.map_or(None, |d| d.delay),
                        arrival_time: arr.map_or(None, |a| a.time),
                        arrival_scheduled_time: arr.map_or(None, |a| a.scheduled_time),
                        arrival_uncertainty: arr.map_or(None, |d| d.uncertainty),
                        departure_delay: dep.map_or(None, |d| d.delay),
                        departure_time: dep.map_or(None, |d| d.time),
                        departure_scheduled_time: dep.map_or(None, |d| d.scheduled_time),
                        departure_uncertainty: dep.map_or(None, |d| d.uncertainty),
                        departure_occupancy_status: stupd.departure_occupancy_status,
                        stop_schedule_relationship: stupd.schedule_relationship
                    }
                })
                .collect::<Vec<RStopTimeUpdate>>()
            } else {
                // no stop time updates (maybe e.g. canceled trip): add with stop time update fields NA
                vec![
                    RStopTimeUpdate {
                        feed_index: index as i32, 
                        trip_id: upd.trip.trip_id.clone(),
                        route_id: upd.trip.route_id.clone(),
                        direction_id: upd.trip.direction_id,
                        start_time: upd.trip.start_time.clone(),
                        start_date: upd.trip.start_date.clone(),
                        trip_schedule_relationship: upd.trip.schedule_relationship,
                        modifications_id: modifications_id.clone(),
                        vehicle_id: vehicle_id.clone(),
                        vehicle_label: vehicle_label.clone(),
                        license_plate: license_plate.clone(),
                        wheelchair_accessible: wheelchair_accessible,
                         stop_sequence: None,
                         stop_id: None,
                         arrival_delay: None,
                         arrival_time: None,
                         arrival_scheduled_time: None,
                         arrival_uncertainty: None,
                         departure_delay: None,
                         departure_time: None,
                         departure_scheduled_time: None,
                         departure_uncertainty: None,
                         departure_occupancy_status: None,
                         stop_schedule_relationship: None }
                ]
            }
        })
        .flatten()
        .collect();

    return content.into_dataframe();
}

extendr_module! {
    mod trip_update;
    fn read_gtfsrt_trip_updates_internal;
}