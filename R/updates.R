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

    result$trip_schedule_relationship = enum_schedule_relationship(result$trip_schedule_relationship)
    result$stop_schedule_relationship = enum_schedule_relationship(result$stop_schedule_relationship)

    result$wheelchair_accessible = no_coerce_factor(
        result$wheelchair_accessible,
        levels = c(0, 1, 2, 3),
        labels = c(
            "NO_VALUE", # 0;
            "UNKNOWN", # 1;
            "WHEELCHAIR_ACCESSIBLE", # 2;
            "WHEELCHAIR_INACCESSIBLE" # 3;
        )
    )

    return(result)
}