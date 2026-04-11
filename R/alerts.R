#' Read GTFS-realtime alerts
#'
#' This function reads GTFS realtime alerts. Alerts are hierarchical:
#' a single alert can have multiple applicability periods, multiple
#' affected entities, and translations to multiple languages. This
#' function flattens all of that to a tabular format, with one row
#' for every combination of applicability, entity, and language. All
#' rows from a single alert can be identified through a common `id`. This
#' is true even if there are duplicate IDs in the feed; they will be
#' deduplicated by adding _duplicated_1, _duplicated_2, etc.
#'
#' Alerts are intended to capture widespread or long-term disruptions or changes.
#' Trip updates (see [read_gtfsrt_trip_updates()]) are better suited to providing
#' information about day-to-day delays and cancellations to specific trips.
#'
#' Typically, GTFS-realtime feeds will contain only a single type of entity, but if there
#' are multiple types of entities in a single feed, this function will read only the
#' alerts.
#'
#' @returns A data frame with one row for every combination of alert,
#' applicability period, affected entity, and language. One alert potentially
#' becomes many rows. The data frame has the following columns. Many of these descriptions
#' come verbatim or nearly so from
#' [the GTFS-realtime specification](https://gtfs.org/documentation/realtime/reference/#message-alert).
#' Path refers to where in the original GTFS-realtime Alert data structure each column comes from.
#'
#' - `id`: GTFS-realtime entity ID. Since one alert can become multiple rows (for example, for different
#'      languages, or different informed entities), the ID can be used to identify rows that came from
#'      the same Alert.
#'
#'      IDs are required by the specification to be unique within a
#'      GTFS-realtime file, but sometimes are not. If there are non-unique IDs in the feed, they will
#'      be made unique when data are loaded by appending `_duplicated_1`, `_duplicated_2`, and so on
#'      and a warning will be issued, which guarantees that all rows from a single file have unique IDs.
#'      When working with archived data, there will quite likely be duplicated IDs between files archived
#'      at different times (path: `id` property of `FeedEntity` containing this `Alert`).
#' - `start`: Time when the alert should first be shown. If missing, the alert should be shown as long as it appears in
#'      the feed. Converted to local time based on the `timezone` argument. One alert may have multiple
#'      start and end times, in which case it will be presented in multiple rows. (path: `active_period.start`)
#' - `end`: Time when the alert should last be shown. If missing, the alert should be shown as long as it appears in
#'      the feed. (path: `active_period.end`).
#' - The next few colums represent the informed entity of the alert (i.e. the agency, route, etc. for which the alert
#'   should be shown). If there are multiple informed entities, the alert will be duplicated for each one.
#'   - `agency_id`: The `agency_id` from the GTFS static feed that this alert refers to. (path: `informed_entity.agency_id`)
#'   - `route_id`: The `route_id`from the GTFS that this alert refers to. If `direction_id`is provided, `route_id`
#'        must also be provided. (path: `informed_entity.route_id`)
#'   - `route_type`: The [`route_type`](https://gtfs.org/documentation/schedule/reference/#routestxt) from the static GTFS
#'        that this alert refers to. (path: `informed_entity.route_type`)
#'   - `direction_id`: The `direction_id`from the static GTFS feed `trips.txt` file, used to select all trips in one
#'        direction for a route, specified by route_id. If `direction_id` is provided, route_id must also be provided.
#'        Caution: this field is still experimental, and subject to change. It may be formally adopted in the future.
#'        (path: `informed_entity.direction_id`).
#'   - `trip_trip_id`: The `trip_id` from the GTFS feed that this selector refers to. For non frequency-based trips
#'        (trips not defined in GTFS `frequencies.txt``), this field is enough to uniquely identify the trip. For
#'        frequency-based trips defined in GTFS `frequencies.txt`, `trip_id`, `start_time`, and `start_date` are all
#'        required. For scheduled-based trips (trips not defined in GTFS frequencies.txt), trip_id can only be omitted
#'        if the trip can be uniquely identified by a combination of `route_id`, `direction_id`, `start_time`, and
#'        `start_date`, and all those fields are provided. (path: `informed_entity.trip.trip_id`)
#'   - `trip_route_id`: The route_id from the GTFS that this trip refers to. If `trip_id` is omitted, `route_id`, `direction_id`,
#'       `start_time`, and `schedule_relationship`=`SCHEDULED` must all be set to identify a trip instance. `trip_route_id` should not be used
#'        to specify a route-wide alert that affects all trips for a route, and generally will not be - `route_id` should be used instead.
#'        (path: `informed_entity.trip.route_id`)
#'   - `trip_direction_id`: The `direction_id`from the GTFS feed trips.txt file, indicating the direction of travel for trips this
#'        selector refers to. If trip_id is omitted, direction_id must be provided. Caution: this field is still experimental, and subject
#'        to change. It may be formally adopted in the future. (path: `informed_entity.trip.route_id`)
#'   - `trip_start_time`: The initially scheduled start time of this trip instance. When the `trip_id` corresponds to a non-frequency-based
#'        trip, this field should either be omitted or be equal to the value in the GTFS feed. When the trip_id correponds to a frequency-based
#'        trip defined in GTFS `frequencies.txt`, start_time is required and must be specified for trip updates and vehicle positions.
#'        If the trip corresponds to exact_times=1 GTFS record, then start_time must be some multiple (including zero) of headway_secs later than
#'        `frequencies.txt` `start_time` for the corresponding time period. If the trip corresponds to exact_times=0, then its start_time may be
#'        arbitrary, and is initially expected to be the first departure of the trip. Once established, the `start_time` of this frequency-based
#'        exact_times=0 trip should be considered immutable, even if the first departure time changes -- that time change may instead be reflected
#'        in a StopTimeUpdate in a TripUpdate. Format and semantics of the field is same as that of GTFS/frequencies.txt/start_time, e.g.,
#'        `11:15:35` for 11:15:35 AM or `25:15:35` for 1:15:35 AM the day after the service day. (path: `informed_entity.trip.start_time`)
#'   - `trip_start_date`: The start date of this trip instance in YYYYMMDD format. For scheduled trips (trips not defined in GTFS frequencies.txt),
#'         this field must be provided to disambiguate trips that are so late as to collide with a scheduled trip on a next day. For example, for a
#'         train that departs 8:00 and 20:00 every day, and is 12 hours late, there would be two distinct trips on the same time. This field can be
#'         provided but is not mandatory for schedules in which such collisions are impossible - for example, a service running on hourly schedule where
#'         a vehicle that is one hour late is not considered to be related to schedule anymore. This field is required for frequency-based trips defined
#'         in GTFS `frequencies.txt`. (path: `informed_entity.trip.start_date`)
#'   - `trip_schedule_relationship`: The relation between this trip and the static schedule. This is not supposed to be used
#'         with alerts, but is provided for completeness. (path: `informed_entity.trip.schedule_relationship`). Possible values:
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
#'   - `trip_modification_id`: Linkage to any modifications done to this trip (shape changes, removal or addition of stops).
#'         Reading trip modifications themselves is not yet supported (see [#21](https://github.com/mattwigway/gtfsrealtime-r/issues/21)).
#'         If you have a feed with trip modifications, please comment on that issue so we are aware they exist in the wild.
#'         (path: `informed_entity.trip.modification_id`)
#'   - `stop_id`: The `stop_id` from the GTFS feed that this alert refers to.
#' - `cause`: The cause of the disruption. (path: `cause`) Possible values:
#'     - `UNKNOWN_CAUSE`
#'     - `OTHER_CAUSE`
#'     - `TECHNICAL_PROBLEM`
#'     - `STRIKE`
#'     - `DEMONSTRATION`
#'     - `ACCIDENT`
#'     - `HOLIDAY`
#'     - `WEATHER`
#'     - `MAINTENANCE`
#'     - `CONSTRUCTION`
#'     - `POLICE_ACTIVITY`
#'     - `MEDICAL_EMERGENCY``
#' - `effect`: The effect of the disruption (path: `effect`). Possible values:
#'     - `NO_SERVICE`
#'     - `REDUCED_SERVICE`
#'     - `SIGNIFICANT_DELAYS`
#'     - `DETOUR`
#'     - `ADDITIONAL_SERVICE`
#'     - `MODIFIED_SERVICE`
#'     - `OTHER_EFFECT`
#'     - `UNKNOWN_EFFECT`
#'     - `STOP_MOVED`
#'     - `NO_EFFECT`
#'     - `ACCESSIBILITY_ISSUE`
#' - `language`: The free-text fields in alerts can be presented in multiple languages. If they are,
#'      rows associated with the alert will be duplicated for each language; this column will contain
#'      the language identifier (e.g. "EN", "ES"), and the remaining fields will contain the translated
#'      alerts in that language. Not every feed will have multiple languages (or lange flags at all),
#'      and it is also possible that some fields are translated into a particular language and others
#'      are left NA.
#' - `cause_detail`: Description of the cause of the alert that allows for agency-specific language;
#'      more specific than the `cause`.  Caution: this field is still experimental, and subject to change.
#'      It may be formally adopted in the future.(path: `cause_detail`)
#' - `effect_detail`: Description of the effect of the alert that allows for agency-specific language;
#'      more specific than the Effect. Caution: this field is still experimental, and subject to change.
#'      It may be formally adopted in the future. (path: `effect_detail`)
#' - `url`: The URL which provides additional information about the alert. May differ for different languages.
#'      (path: `url`).
#' - `header_text`: Header (i.e. title) for the alert (path: `header`)
#' - `description_text`: Description for the alert. The information in the description should add to the
#'    information of the header. (path: `description`)
#' - `tts_header_text`: Text containing the alert's header to be used for text-to-speech implementations.
#'    This field is the text-to-speech version of header_text. It should contain the same information as
#'    `header_text` but formatted such that it can read as text-to-speech (for example, abbreviations removed,
#'    numbers spelled out, etc.) (path: `tts_header_text`)
#' - `tts_description_text`: Text containing a description for the alert to be used for text-to-speech implementations.
#'    This field is the text-to-speech version of `description_text``. It should contain the same information as
#'    `description_text`but formatted such that it can be read as text-to-speech (for example, abbreviations removed,
#'    numbers spelled out, etc.) (path: `tts_description_text`)
#' - `severity_level`: Severity of the alert. (path: `severity_level`). Possible values:
#'     - `UNKNOWN_SEVERITY`
#'     - `INFO`
#'     - `WARNING`
#'     - `SEVERE`
#'
#'
#' @param filename filename to read. Can be uncompressed or compressed with
#'      gzip or bzip2. Can also be an http:// or https:// URL.
#' @param timezone timezone of feed, in Olson format. Times in GTFS-realtime are
#'  stored as Unix time in UTC; this option will convert to local times. If you
#'  want to read times in UTC, specify "Etc/UTC".
#' @param label_values should enum types in GTFS-realtime (i.e. categorical variables)
#'      be converted to factors with their English labels. If false, they
#'      will be left as numeric codes. Default true.
#'
#' @examples
#' # This will read an alerts feed included with gtfsrealtime. Replace with
#' # the path to your own file if desired.
#' file = system.file("nyc-service-alerts.pb.bz2", package = "gtfsrealtime")
#'
#' # Need to specify timezone so timestamps will be in local time.
#' read_gtfsrt_alerts(file, "America/New_York")
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
