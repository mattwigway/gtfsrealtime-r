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
#' @param label_values should enum types in GTFS-realtime (i.e. categorical variables)
#'      be converted to factors with their English labels. If false, they
#'      will be left as numeric codes. Default true.
#' @export
read_gtfsrt_alerts = function(filename, timezone, label_values = TRUE) {
  check_timezone(timezone)

  result = read_gtfsrt_alerts_internal(filename)

  if (!is.null(result$err)) {
    cli_abort(result$err)
  } else {
    result = result$ok
  }

  result$start = as.POSIXct(result$start, tz = timezone)
  result$end = as.POSIXct(result$end, tz = timezone)

  if (label_values) {
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
  }

  return(result)
}
