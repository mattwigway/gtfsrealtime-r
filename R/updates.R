#' Read GTFS-realtime trip updates
#' 
#' Note that each trip update becomes multiple rows, one per stop time update, with
#' the trip_id, etc., duplicated in each row. The column feed_index can be used
#' to identify which rows came from the same trip update.
#' @export
read_gtfsrt_trip_updates = function (filename) {
    result = read_gtfsrt_trip_updates_internal(filename)

    result$trip_schedule_relationship = no_coerce_factor(
        result$trip_schedule_relationship,
        # note intentionally no 4
        levels = c(0, 1, 2, 3, 5, 6, 7, 8),
        labels = c(
            "SCHEDULED", # 0;
            "ADDED", # 1 [deprecated", # true];
            "UNSCHEDULED", # 2;
            "CANCELED", # 3;
            "REPLACEMENT", # 5;
            "DUPLICATED", # 6;
            "DELETED", # 7;
            "NEW" # 8;
        )
    )

    result$stop_schedule_relationship = no_coerce_factor(
        result$stop_schedule_relationship,
        # note intentionally no 4
        levels = c(0, 1, 2, 3, 5, 6, 7, 8),
        labels = c(
            "SCHEDULED", # 0;
            "ADDED", # 1 [deprecated", # true];
            "UNSCHEDULED", # 2;
            "CANCELED", # 3;
            "REPLACEMENT", # 5;
            "DUPLICATED", # 6;
            "DELETED", # 7;
            "NEW" # 8;
        )
    )

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