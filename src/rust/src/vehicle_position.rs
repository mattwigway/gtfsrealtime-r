use extendr_api::prelude::*;
use crate::read::read_feed;

#[derive(IntoDataFrameRow, Debug, PartialEq)]
pub struct RVehiclePosition {
    latitude: Option<f32>,
    longitude: Option<f32>,
    bearing: Option<f32>,
    vehicle_id: Option<String>,
    vehicle_label: Option<String>,
    vehicle_license_plate: Option<String>
}

/// Read GTFS-RT vehicle positions
/// @export
#[extendr]
pub fn read_gtfsrt_positions(file: String) -> Result<Dataframe<RVehiclePosition>> {
    let msg = read_feed(file)?;
    let content: Vec<RVehiclePosition> = msg.entity.iter()
        .filter(|entity| entity.vehicle.is_some())
        .map(|entity| {
            let veh = entity.vehicle.as_ref().unwrap();
            RVehiclePosition {
                latitude: veh.position.as_ref().map_or(None, |pos| Some(pos.latitude)),
                longitude: veh.position.as_ref().map_or(None, |pos| Some(pos.longitude)),
                bearing: veh.position.as_ref().map_or(None, |pos| pos.bearing),
                vehicle_id: veh.vehicle.as_ref().map_or(None, |veh| veh.id.clone()),
                vehicle_label: veh.vehicle.as_ref().map_or(None, |veh| veh.label.clone()),
                vehicle_license_plate: veh.vehicle.as_ref().map_or(None, |veh| veh.license_plate.clone())
            }
        })
        .collect();

    return content.into_dataframe();
}

extendr_module! {
    mod gtfsrt;
    fn read_gtfsrt_positions;
}