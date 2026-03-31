#' Read GTFS-realtime trip updates
#' 
#' Note that each trip update becomes multiple rows, one per stop time update, with
#' the trip_id, etc., duplicated in each row. The column id can be used
#' to identify which rows came from the same trip update.
#' 
#' @param filename filename to read. Can be uncompressed or compressed with
#'      gzip or bzip2. Can also be an http:// or https:// URL.
#' @export
read_gtfsrt_trip_updates = function (filename) {
    result = read_gtfsrt_trip_updates_internal(filename)

    if (!is.null(result$err)) {
        cli_abort(result$err)
    } else {
        result = result$ok
    }

    result$trip_schedule_relationship = enum_to_factor(
        result$trip_schedule_relationship,
        enum_TripUpdate_StopTimeUpdate_ScheduleRelationship()
    )
    result$stop_schedule_relationship = enum_to_factor(
        result$stop_schedule_relationship,
        enum_TripUpdate_StopTimeUpdate_ScheduleRelationship()
    )
    result$wheelchair_accessible = enum_to_factor(
        result$wheelchair_accessible,
        enum_VehicleDescriptor_WheelchairAccessible()
    )

    return(result)
}