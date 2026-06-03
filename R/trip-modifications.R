#' Read GTFS-realtime trip modifications
#' @export
read_gtfsrt_trip_modifications = function(file, timezone) {
  file = path.expand(file)

  check_timezone(timezone)

  result = read_gtfsrt_trip_modifications_internal(file)

  if (!is.null(result$err)) {
    cli_abort(result$err)
  } else {
    result = result$ok
  }

  result$last_modified = as.POSIXct(result$last_modified, tz = timezone)
  result$file_timestamp = as.POSIXct(result$file_timestamp, tz = timezone)

  # TODO stop, shape messages
  return(list(trip_modifications = result))
}
