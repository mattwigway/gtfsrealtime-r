use std::io::Result;

use prost_build::Config;
fn main() -> Result<()> {
    let deriv = "#[derive(strum_macros::VariantArray, strum_macros::IntoStaticStr)]";
    Config::new()
        .type_attribute(".transit_realtime.TripUpdate.StopTimeUpdate.ScheduleRelationship", &deriv)
        .type_attribute(".transit_realtime.TripDescriptor.ScheduleRelationship", &deriv)
        .type_attribute(".transit_realtime.VehiclePosition.VehicleStopStatus", &deriv)
        .type_attribute(".transit_realtime.VehiclePosition.CongestionLevel", &deriv)
        .type_attribute(".transit_realtime.VehiclePosition.OccupancyStatus", &deriv)
        .compile_protos(&["src/gtfs-realtime.proto"], &["src/"])?;
    Ok(())
}