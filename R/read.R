#' Read GTFS-realtime vehicle positions into a data frame
#' 
#' @param filename filename to read. can be uncompressed or compressed with
#'      gzip or bzip2.
#' @param as_sf return an sf (spatial) object rather than a data frame.
#' @export
read_gtfsrt_positions = function (filename, as_sf=FALSE) {
    result = read_gtfsrt_positions_internal(filename)

    result$schedule_relationship = factor(
        result$schedule_relationship,
        levels = c(0, 1, 2, 3),
        labels = c("SCHEDULED", "SKIPPED", "NO_DATA", "UNSCHEDULED")
    )

    result$current_status = factor(
        result$current_status,
        levels = c(0, 1, 2),
        labels = c(
            "INCOMING_AT",
            "STOPPED_AT",
            "IN_TRANSIT_TO"
        )
    )

    result$congestion_level = factor(
        result$congestion_level,
        levels = c(0, 1, 2, 3, 4),
        labels = c(
            "UNKNOWN_CONGESTION_LEVEL",
            "RUNNING_SMOOTHLY",
            "STOP_AND_GO",
            "CONGESTION",
            "SEVERE_CONGESTION"
        )
    )

    result$occupancy_status = factor(
        result$occupancy_status,
        levels = c(0, 1, 2, 3, 4, 5, 6, 7, 8),
        labels = c(
            "EMPTY",
            "MANY_SEATS_AVAILABLE",
            "FEW_SEATS_AVAILABLE",
            "STANDING_ROOM_ONLY",
            "CRUSHED_STANDING_ROOM_ONLY",
            "FULL",
            "NOT_ACCEPTING_PASSENGERS",
            "NO_DATA_AVAILABLE",
            "NOT_BOARDABLE"
        )
    )

    if (as_sf) {
        result = st_as_sf(result, coords=c("longitude", "latitude"), crs=4326)
    }

    return(result)
}