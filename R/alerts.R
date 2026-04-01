#' Read GTFS-realtime alerts
#' 
#' This function reads GTFS realtime alerts. Alerts are hierarchical:
#' a single alert can have multiple applicability periods, multiple
#' affected entities, and translations to multiple languages. This
#' function flattens all of that to a tabular format, with one row
#' for every combination of applicability, entity, and language. All
#' rows from a single alert can be identified through a common id.
#' 
#' @param filename filename to read. Can be uncompressed or compressed with
#'      gzip or bzip2. Can also be an http:// or https:// URL.
#' @export
read_gtfsrt_alerts = function (filename) {
    result = read_gtfsrt_alerts_internal(filename)

    if (!is.null(result$err)) {
        cli_abort(result$err)
    } else {
        result = result$ok
    }

    result$trip_schedule_relationship = enum_to_factor(
        result$trip_schedule_relationship,
        enum_TripDescriptor_ScheduleRelationship()
    )

    result$cause = enum_to_factor(
        result$cause,
        enum_Alert_Cause()
    )

    result$effect = enum_to_factor(
        result$effect,
        enum_Alert_Effect()
    )

    result$severity_level = enum_to_factor(
        result$severity_level,
        enum_Alert_SeverityLevel()
    )

    return(result)
}