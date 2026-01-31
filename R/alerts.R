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

    result$trip_schedule_relationship =
        enum_schedule_relationship(result$trip_schedule_relationship)
    result$cause = enum_cause(result$cause)
    result$severity_level = enum_severity_level(result$severity_level)

    return(result)
}