use std::io::Result;

use prost_build::Config;
fn main() -> Result<()> {
    let deriv = "#[derive(strum_macros::VariantArray, strum_macros::IntoStaticStr, gtfsrt_macros::AsStrName)]";
    Config::new()
        // I can't figure out a way to tell prost to just apply this to all enums without also applying to structs
        .type_attribute(
            ".transit_realtime.TripUpdate.StopTimeUpdate.ScheduleRelationship",
            &deriv,
        )
        .type_attribute(
            ".transit_realtime.TripDescriptor.ScheduleRelationship",
            &deriv,
        )
        .type_attribute(
            ".transit_realtime.VehiclePosition.VehicleStopStatus",
            &deriv,
        )
        .type_attribute(".transit_realtime.VehiclePosition.CongestionLevel", &deriv)
        .type_attribute(".transit_realtime.VehiclePosition.OccupancyStatus", &deriv)
        .type_attribute(
            ".transit_realtime.VehicleDescriptor.WheelchairAccessible",
            &deriv,
        )
        .type_attribute(".transit_realtime.Alert.Cause", &deriv)
        .type_attribute(".transit_realtime.Alert.Effect", &deriv)
        .type_attribute(".transit_realtime.Alert.SeverityLevel", &deriv)
        .compile_protos(&["src/gtfs-realtime.proto"], &["src/"])?;
    Ok(())
}
