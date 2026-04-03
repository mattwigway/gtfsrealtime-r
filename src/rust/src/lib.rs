pub mod transit_realtime {
    include!(concat!(env!("OUT_DIR"), "/transit_realtime.rs"));
}

mod alert;
mod check_types;
mod enums;
mod id_deduplicator;
mod read;
mod test_data;
mod trip_update;
mod vehicle_position;

use extendr_api::prelude::*;

pub use trip_update::read_gtfsrt_trip_updates_internal;
pub use vehicle_position::read_gtfsrt_positions_internal;

extendr_module! {
    mod gtfsrealtime;
    use vehicle_position;
    use trip_update;
    use alert;
    use test_data;
}
