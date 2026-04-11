#' Read GTFS-realtime vehicle positions into a data frame.
#'
#' Reads vehicle position data from a GTFS-realtime feed into a data.frame with
#' one row per position update. GTFS-realtime timestamps are represented in Unix
#' time; a timezone must be specified to convert these to time-zone-aware R date-time
#' objects (i.e. POSIXct).
#'
#' Typically, GTFS-realtime feeds will contain only a single type of entity, but if there
#' are multiple types of entities in a single feed, this function will read only the
#' vehicle positions. Each vehicle position will become a single row in the data frame.
#'
#' The data frame will have the following columns. Note that most of these columns can contain NAs
#' (and in most feeds, many will be entirely NA). GTFS-realtime is a hierarchical format that is
#' converted to a flat format for use in R; the paths refer to where each column comes from
#' within the GTFS-realtime VehiclePosition data structure. Each column is reported below with
#' its definition, many of which come verbatim from the [GTFS-realtime specification](https://gtfs.org/documentation/realtime/reference/)
#'
#' - `id`: GTFS-realtime entity ID. These are required by the specification to be unique within a
#' GTFS-realtime file, but sometimes are not. If there are non-unique IDs in the feed, they will
#' be made unique when data are loaded by appending `_duplicate_1`, `_duplicate_2`, and so on
#' and a warning will be issued, which guarantees that all rows from a single file have unique IDs.
#' When working with archived data, there will quite likely be duplicated IDs between files archived
#' at different times (path: `id` property of `FeedEntity` containing this `VehiclePosition`).
#' - `latitude`: reported latitude of vehicle (path: `position.latitude`).
#' - `longitude`: reported latitude of vehicle (path: `position.latitude`).
#' - `bearing`: current bearing (compass heading) of vehicle, in degrees
#'  (path: `position.bearing`).
#' - `odometer`: the distance the vehicle is traveled, according to the specification
#'  should be in meters (path: `position.odometer`).
#' - `speed`: current speed of the vehicle, in meters per second
#'  (path: `position.speed`).
#' - `trip_id`: Trip ID the vehicle is currently serving (path: `trip.trip_id`)
#' - `route_id`: Route ID the vehicle is currently serving (path `trip.route_id`)
#' - `direction_id`: GTFS direction ID (0 or 1) of the trip the vehicle is currently
#'    serving (path: `trip.direction_id`)
#' - `start_time`: the time this trip started, needed to differentiate trips in frequency
#'    based GTFS. (path: `trip.start_time`)
#' - `start_date`: the time this trip started, needed to differentiate trips in frequency
#'    based GTFS or when the service day is otherwise not clear. (path: `trips.start_date`)
#' - `schedule_relationship`: How is this trip related to the schedule in the static GTFS?
#'      (path: `trip.schedule_relationship`). Possible values:

#'     - `SCHEDULED`:
#'       Trip that is running in accordance with its GTFS schedule, or is close
#'       enough to the scheduled trip to be associated with it.
#'     - `ADDED`:
#'       This value has been deprecated as the behavior was unspecified.
#'       Use `DUPLICATED` for an extra trip that is the same as a scheduled trip except the start date or time,
#'       or `NEW` for an extra trip that is unrelated to an existing trip.
#'     - `UNSCHEDULED`:
#'       A trip that is running with no schedule associated to it (GTFS `frequencies.txt` `exact_times`=0).
#'       Trips with `trip_schedule_relationship`=`UNSCHEDULED` must also set all `stop_schedule_relationship`=`UNSCHEDULED.`
#'     - `CANCELED`:
#'       A trip that existed in the schedule but was removed.
#'     - `REPLACEMENT`:
#'       A trip that replaces an existing trip in the schedule.
#'       NOTE: This field is still experimental, and subject to change. It may be formally adopted in the future.
#'     - `DUPLICATED`:
#'       An extra trip that was added in addition to a running schedule, for example, to replace a broken vehicle or to
#'       respond to sudden passenger load. Used with `trip_id`, `start_date`,
#'       and `start_time` to copy an existing trip from static GTFS but start at a different service
#'       date and/or time. Duplicating a trip is allowed if the service related to the original trip in (CSV) GTFS
#'       (in calendar.txt or calendar_dates.txt) is operating within the next 30 days. The trip to be duplicated is
#'       identified via `trip_id.` This enumeration does not modify the existing trip referenced by
#'       `trip_id` - if a producer wants to cancel the original trip, it must publish a separate
#'       TripUpdate with the value of `CANCELED` or `DELETED`. If a producer wants to replace the original trip, a value of
#'       `REPLACEMENT` should be used instead.
#'
#'       Trips defined in GTFS `frequencies.txt` with `exact_times` that is
#'       empty or equal to 0 cannot be duplicated.
#'
#'       Existing producers and consumers that were using the ADDED enumeration to represent duplicated trips must follow
#'       [the migration guide](https://github.com/google/transit/blob/master/gtfs-realtime/spec/en/examples/migration-duplicated.md)
#'       to transition to the `DUPLICATED` enumeration.
#'       NOTE: This field is still experimental, and subject to change. It may be formally adopted in the future.
#'     - `DELETED`:
#'       A trip that existed in the schedule but was removed and must not be shown to users.
#'       `DELETED` should be used instead of `CANCELED` to indicate that a transit provider would like to entirely remove
#'       information about the corresponding trip from consuming applications, so the trip is not shown as cancelled to
#'       riders, e.g. a trip that is entirely being replaced by another trip.
#'       This designation becomes particularly important if several trips are cancelled and replaced with substitute service.
#'       If consumers were to show explicit information about the cancellations it would distract from the more important
#'       real-time predictions.
#'       NOTE: This field is still experimental, and subject to change. It may be formally adopted in the future.
#'     - `NEW`:
#'       An extra trip unrelated to any existing trips, for example, to respond to sudden passenger load.
#'       NOTE: This field is still experimental, and subject to change. It may be formally adopted in the future.
#' - `stop_id`: Stop ID in static GTFS the vehicle is at/approaching. See `current_status`. (path: `stop_id`)
#' - `current_stop_sequence`: Stop sequence in static GTFS the vehicle is at/approaching (used to disambiguate
#'      trips that serve the same stop twice). See `current_status`.
#'      (path: `current_stop_sequence`)
#' - `current_status`: Current status of the vehicle in relation to the stop identified by
#'      `stop_id`/`current_stop_sequence`. (path: `current_status`). Potential values:
#'    - "INCOMING_AT": The vehicle is approaching this stop.
#'    - "STOPPED_AT": The vehicle is stopped at this stop.
#'    - "IN_TRANSIT_TO": The GTFS-realtime specification says this means "The vehicle has
#'      departed and is in transit to the next stop." which would suggest that it has _departed_
#'      the stop identified by `stop_id`. However, the value itself suggests the vehicle has
#'      departed the _previous_ stop. Auditing this across a number of GTFS feeds is planned.
#' - `timestamp`: Moment at which the vehicle's position was measured. In the GTFS-realtime data
#'      this is recorded as second since January 1, 1970, UTC (i.e. Unix time), but when read by
#'      this function it will be converted to local time in the timezone specified by the function
#'      call. (path: `timestamp`).
#' - `congestion_level`: Congestion level the vehicle is experiencing. Possible values are
#'      UNKNOWN_CONGESTION_LEVEL, RUNNING_SMOOTHLY, STOP_AND_GO, CONGESTION, SEVERE_CONGESTION,
#'      with SEVERE_CONGESTION flagged as meaning "People leaving their cars."
#' - `occupancy_status`: How full the vehicle is. (path: `occupancy_status`). Possible values (with official
#'      descriptions from the GTFS-realtime spec).:
#'   - "EMPTY": The vehicle or carriage is considered empty by most measures, and has few or no
#'       passengers onboard, but is still accepting passengers.
#'   - "MANY_SEATS_AVAILABLE": The vehicle or carriage has a large number of seats available.
#'       The amount of free seats out of the total seats available to be
#'       considered large enough to fall into this category is determined at the
#'       discretion of the producer.
#'   - "FEW_SEATS_AVAILABLE": The vehicle or carriage has a relatively small number of seats available.
#'       The amount of free seats out of the total seats available to be
#'       considered small enough to fall into this category is determined at the
#'       discretion of the feed producer.
#'   - "STANDING_ROOM_ONLY": The vehicle or carriage can currently accommodate only standing passengers.
#'   - "CRUSHED_STANDING_ROOM_ONLY": The vehicle or carriage can currently accommodate only standing passengers
#'       and has limited space for them.
#'   - "FULL": The vehicle or carriage is considered full by most measures, but may still be
#'       allowing passengers to board.
#'   - "NOT_ACCEPTING_PASSENGERS": The vehicle or carriage is not accepting passengers, but usually accepts passengers for boarding.
#'   - "NO_DATA_AVAILABLE": The vehicle or carriage doesn't have any occupancy data available at that time.
#'   - "NOT_BOARDABLE": The vehicle or carriage is not boardable and never accepts passengers.
#'       Useful for special vehicles or carriages (engine, maintenance carriage, etc…).
#' - `occupancy_percentage`: A percentage value indicating the degree of passenger occupancy in the vehicle.
#'     The values are represented as an integer without decimals. 0 means 0% and 100 means 100%.
#'     The value 100 should represent the total maximum occupancy the vehicle was designed for,
#'     including both seated and standing capacity, and current operating regulations allow.
#'     The value may exceed 100 if there are more passengers than the maximum designed capacity.
#'     The precision of occupancy_percentage should be low enough that individual passengers cannot be tracked boarding or alighting the vehicle.
#'     If multi_carriage_status is populated with per-carriage occupancy_percentage,
#'     then this field should describe the entire vehicle with all carriages accepting passengers considered.
#'     This field is still experimental, and subject to change. It may be formally adopted in the future. (path: `occupancy_percentage`)
#' - `vehicle_id`: Internal system identification of the vehicle. Should be unique per vehicle, and is used for
#'     tracking the vehicle as it proceeds through the system. This id should not be made visible to the end-user;
#'     for that purpose use the label field. (path: `vehicle.id`)
#' - `vehicle_label`: User visible label, i.e., something that must be shown to the passenger to help
#'     identify the correct vehicle. (path: `vehicle.label`)
#' - `vehicle_license_plate`: The license plate of the vehicle. (path: `vehicle.license_plate`)
#' - `vehicle_wheelchair_accessible`: Whether the vehicle is wheelchair accessible. If provided,
#'    can overwrite the wheelchair_accessible value from the static GTFS. (path: `vehicle.wheelchair_accessible`). Possible values:
#' ```{r child="man/rmd/wheelchair_accessible.md"}
#' ```
#'
#' @param filename filename to read. Can be uncompressed or compressed with
#'      gzip or bzip2. Can also be an http: or https: URL.
#' @param timezone timezone of feed, in Olson format. Times in GTFS-realtime are
#'  stored as Unix time in UTC; this option will convert to local times. If you
#'  want to read times in UTC, specify "Etc/UTC"
#' @param as_sf return an sf (spatial) object rather than a data frame.
#' @param label_values should enum types in GTFS-realtime (i.e. categorical variables)
#'      be converted to factors with their English labels. If false, they
#'      will be left as numeric codes? Default TRUE
#' @returns data frame containing vehicle position data
#'
#' @examples
#' # This will read a positions feed included with gtfsrealtime. Replace with
#' # the path to your own file if desired.
#' file = system.file("nyc-vehicle-positions.pb.bz2", package = "gtfsrealtime")
#'
#' # Need to specify timezone so timestamps will be in local time.
#' read_gtfsrt_positions(file, "America/New_York")
#' @export
read_gtfsrt_positions = function(filename, timezone, as_sf = FALSE, label_values = TRUE) {
  check_timezone(timezone)

  result = read_gtfsrt_positions_internal(filename)

  if (!is.null(result$err)) {
    cli_abort(result$err)
  } else {
    result = result$ok
  }

  result$timestamp = as.POSIXct(result$timestamp, tz = timezone)

  if (label_values) {
    result$schedule_relationship = enum_to_factor(
      result$schedule_relationship,
      enum_TripDescriptor_ScheduleRelationship()
    )

    result$vehicle_wheelchair_accessible = enum_to_factor(
      result$vehicle_wheelchair_accessible,
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
  }

  if (as_sf) {
    result = st_as_sf(result, coords = c("longitude", "latitude"), crs = 4326)
  }

  return(result)
}
