#' Read GTFS-realtime trip updates
#' 
#' Note that each trip update becomes multiple rows, one per stop time update, with
#' the trip_id, etc., duplicated in each row. The column id can be used
#' to identify which rows came from the same trip update.
#' 
#' @param filename filename to read. Can be uncompressed or compressed with
#'      gzip or bzip2. Can also be an http:// or https:// URL.
#' @param label_values should enum types in GTFS-realtime (i.e. categorical variables)
#'      be converted to factors with their English labels. If false, they
#'      will be left as numeric codes. Default true.
#' @export
read_gtfsrt_trip_updates = function (filename, label_values = TRUE) {
    result = read_gtfsrt_trip_updates_internal(filename)

    if (!is.null(result$err)) {
        cli_abort(result$err)
    } else {
        result = result$ok
    }

    if (label_values) {
        result$trip_schedule_relationship = enum_to_factor(
            result$trip_schedule_relationship,
            enum_TripDescriptor_ScheduleRelationship()
        )
        result$stop_schedule_relationship = enum_to_factor(
            result$stop_schedule_relationship,
            enum_TripUpdate_StopTimeUpdate_ScheduleRelationship()
        )

        result$departure_occupancy_status = enum_to_factor(
            result$departure_occupancy_status,
            enum_VehiclePosition_OccupancyStatus()
        )

        result$wheelchair_accessible = enum_to_factor(
            result$wheelchair_accessible,
            enum_VehicleDescriptor_WheelchairAccessible()
        )
    }

    return(result)
}