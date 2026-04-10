use std::io::Result;
use std::process::Command;
use std::path::Path;

use prost_build::Config;
fn main() -> Result<()> {
    // generate generated rust code from protobuf, if needed
    // The reason we don't do this always is because protoc is unlikely to be
    // available on CRAN build machines. So transit_realtime.rs is built on
    // Github Actions and then rolled into the built package.
    if !Path::exists(Path::new("src/generated/transit_realtime.rs")) {
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
            .out_dir("src/generated")
            .compile_protos(&["src/gtfs-realtime.proto"], &["src/"])?;
    }
    Ok(())
}
