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

    result$schedule_relationship = no_coerce_factor(
        result$schedule_relationship,
        levels = c(0, 1, 2, 3),
        labels = c("SCHEDULED", "SKIPPED", "NO_DATA", "UNSCHEDULED")
    )

    result$current_status = no_coerce_factor(
        result$current_status,
        levels = c(0, 1, 2),
        labels = c(
            "INCOMING_AT",
            "STOPPED_AT",
            "IN_TRANSIT_TO"
        )
    )

    result$congestion_level = no_coerce_factor(
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

    result$occupancy_status = no_coerce_factor(
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
        # sfheaders sf_point much faster than sf::st_as_sf
        result = sf_point(
            result,
            x = "longitude",
            y = "latitude",
            keep = TRUE
        )

        st_crs(result) = 4326
    }

    return(result)
}