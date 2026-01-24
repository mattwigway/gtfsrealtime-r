pub mod transit_realtime {
    include!(concat!(env!("OUT_DIR"), "/transit_realtime.rs"));
}

mod vehicle_position;
mod trip_update;
mod read;

use extendr_api::prelude::*;

pub use vehicle_position::read_gtfsrt_positions_internal;
pub use trip_update::read_gtfsrt_trip_updates_internal;

extendr_module! {
    mod gtfsrealtime;
    use vehicle_position;
    use trip_update;
}