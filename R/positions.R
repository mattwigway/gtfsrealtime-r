#' Read GTFS-realtime vehicle positions into a data frame
#' 
#' @param filename filename to read. Can be uncompressed or compressed with
#'      gzip or bzip2. Can also be an http:// or https:// URL.
#' @param timezone timezone of feed, in Olson format. Times in GTFS-realtime are
#'  stored as Unix time in UTC; this option will convert to local times. If you
#'  want to read times in UTC, specify "Etc/UTC"
#' @param as_sf return an sf (spatial) object rather than a data frame.
#' @export
read_gtfsrt_positions = function (filename, timezone, as_sf=FALSE) {
    if (!(timezone %in% OlsonNames())) {
        cli_abort(c(
            "Invalid time zone",
            "i" = "Specify a timezone in Olson format, e.g. \"America/New_York\" or \"Etc/UTC\"",
            "x" = glue("You specified \"{timezone}\""
            )
        ))
    }

    result = read_gtfsrt_positions_internal(filename)

    if (!is.null(result$err)) {
        cli_abort(result$err)
    } else {
        result = result$ok
    }

    result$timestamp = as.POSIXct(result$timestamp, tz = timezone)

    result$schedule_relationship = enum_to_factor(
        result$schedule_relationship,
        enum_TripDescriptor_ScheduleRelationship()
    )

    result$wheelchair_accessible = enum_to_factor(
        result$wheelchair_accessible,
        enum_VehicleDescriptor_WheelchairAccessible()
    )

    result$current_status = enum_to_factor(
        result$current_status,
        enum_VehiclePosition_VehicleStopStatus()
    )

    result$congestion_level = enum_to_factor(
        result$congestion_level,
        enum_VehiclePosition_CongestionLevel()
    )

    result$occupancy_status = enum_to_factor(
        result$occupancy_status,
        enum_VehiclePosition_OccupancyStatus()
    )

    if (as_sf) {
        result = st_as_sf(result, coords=c("longitude", "latitude"), crs=4326)
    }

    return(result)
}