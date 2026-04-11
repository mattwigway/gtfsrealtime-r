use std::env;
use std::io::Result;
use std::path::Path;
use std::process::Command;

use prost_build::Config;

fn print_path_exists(pathstr: &str) -> () {
    let path = Path::new(pathstr);
    if path.exists() {
        eprintln!("{pathstr}: exists");
    } else {
        eprintln!("{pathstr}: does not exist");
    }
}

fn main() -> Result<()> {
    eprintln!("Running build script. Current situation:");
    print_path_exists("src");
    print_path_exists("src/generated");
    print_path_exists("src/gtfs-realtime.proto");

    if env::consts::OS == "linux" || env::consts::OS == "macos" {
        match Command::new("which").arg("protoc").output() {
            Ok(res) => {
                let path = String::from_utf8(res.stdout).unwrap();
                println!("protoc found at {path}");
            }
            Err(_) => println!("protoc not found"),
        }
    } else {
        println!("Not checking for protoc location on Windows");
    }

    // generate generated rust code from protobuf, if needed
    // The reason we don't do this always is because protoc is unlikely to be
    // available on CRAN build machines. So transit_realtime.rs is built on
    // Github Actions and then rolled into the built package.
    if !Path::new("src/generated/transit_realtime.rs").exists() {
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
