// This doesn't follow the prost-build recommended process of having the
// .proto file compiled to Rust directly into the output directory, because
// it is unlikely that protoc is available on CRAN build machines. So instead
// we have the generated code in the source tree, but not checked into version
// control. When the source package is built by GH actions, the code is generated
// and gets included in the .tar.gz without needing to be in the repository.
pub mod transit_realtime {
    include!("generated/transit_realtime.rs");
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
