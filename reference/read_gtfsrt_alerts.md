# Read GTFS-realtime alerts

This function reads GTFS realtime alerts. Alerts are hierarchical: a
single alert can have multiple applicability periods, multiple affected
entities, and translations to multiple languages. This function flattens
all of that to a tabular format, with one row for every combination of
applicability, entity, and language. All rows from a single alert can be
identified through a common `id`. This is true even if there are
duplicate IDs in the feed; they will be deduplicated by adding
\_duplicated_1, \_duplicated_2, etc.

## Usage

``` r
read_gtfsrt_alerts(filename, timezone, label_values = TRUE)
```

## Arguments

- filename:

  filename to read. Can be uncompressed or compressed with gzip or
  bzip2. Can also be an http:// or https:// URL.

- timezone:

  timezone of feed, in Olson format. Times in GTFS-realtime are stored
  as Unix time in UTC; this option will convert to local times. If you
  want to read times in UTC, specify "Etc/UTC".

- label_values:

  should enum types in GTFS-realtime (i.e. categorical variables) be
  converted to factors with their English labels. If false, they will be
  left as numeric codes. Default true.

## Value

A data frame with one row for every combination of alert, applicability
period, affected entity, and language. One alert potentially becomes
many rows. The data frame has the following columns. Many of these
descriptions come verbatim or nearly so from [the GTFS-realtime
specification](https://gtfs.org/documentation/realtime/reference/#message-alert).
Path refers to where in the original GTFS-realtime Alert data structure
each column comes from.

- `id`: GTFS-realtime entity ID. Since one alert can become multiple
  rows (for example, for different languages, or different informed
  entities), the ID can be used to identify rows that came from the same
  Alert.

  IDs are required by the specification to be unique within a
  GTFS-realtime file, but sometimes are not. If there are non-unique IDs
  in the feed, they will be made unique when data are loaded by
  appending `_duplicated_1`, `_duplicated_2`, and so on and a warning
  will be issued, which guarantees that all rows from a single file have
  unique IDs. When working with archived data, there will quite likely
  be duplicated IDs between files archived at different times (path:
  `id` property of `FeedEntity` containing this `Alert`).

- `start`: Time when the alert should first be shown. If missing, the
  alert should be shown as long as it appears in the feed. Converted to
  local time based on the `timezone` argument. One alert may have
  multiple start and end times, in which case it will be presented in
  multiple rows. (path: `active_period.start`)

- `end`: Time when the alert should last be shown. If missing, the alert
  should be shown as long as it appears in the feed. (path:
  `active_period.end`).

- The next few colums represent the informed entity of the alert (i.e.
  the agency, route, etc. for which the alert should be shown). If there
  are multiple informed entities, the alert will be duplicated for each
  one.

  - `agency_id`: The `agency_id` from the GTFS static feed that this
    alert refers to. (path: `informed_entity.agency_id`)

  - `route_id`: The
    ``` route_id`` from the GTFS that this alert refers to. If  ```direction_id“
    is provided, `route_id` must also be provided. (path:
    \`informed_entity.route_id\`)

  - `route_type`: The
    [`route_type`](https://gtfs.org/documentation/schedule/reference/#routestxt)
    from the static GTFS that this alert refers to. (path:
    `informed_entity.route_type`)

  - `direction_id`: The
    ``` direction_id`` from the static GTFS feed  ```trips.txt`file, used to select all trips in one direction for a route, specified by route_id. If`direction_id`is provided, route_id must also be provided. Caution: this field is still experimental, and subject to change. It may be formally adopted in the future. (path:`informed_entity.direction_id\`).

  - `trip_trip_id`: The `trip_id` from the GTFS feed that this selector
    refers to. For non frequency-based trips (trips not defined in GTFS
    ``` frequencies.txt``), this field is enough to uniquely identify the trip. For frequency-based trips defined in GTFS  ```frequencies.txt`, `trip_id`, `start_time`, and `start_date`are all required. For scheduled-based trips (trips not defined in GTFS frequencies.txt), trip_id can only be omitted if the trip can be uniquely identified by a combination of`route_id`, `direction_id`, `start_time`, and `start_date`, and all those fields are provided. (path: `informed_entity.trip.trip_id\`)

  - `trip_route_id`: The route_id from the GTFS that this trip refers
    to. If `trip_id` is omitted, `route_id`, `direction_id`,
    `start_time`, and `schedule_relationship`=`SCHEDULED` must all be
    set to identify a trip instance. `trip_route_id` should not be used
    to specify a route-wide alert that affects all trips for a route,
    and generally will not be - `route_id` should be used instead.
    (path: `informed_entity.trip.route_id`)

  - `trip_direction_id`: The
    ``` direction_id`` from the GTFS feed trips.txt file, indicating the direction of travel for trips this selector refers to. If trip_id is omitted, direction_id must be provided. Caution: this field is still experimental, and subject to change. It may be formally adopted in the future. (path:  ```informed_entity.trip.route_id\`)

  - `trip_start_time`: The initially scheduled start time of this trip
    instance. When the `trip_id` corresponds to a non-frequency-based
    trip, this field should either be omitted or be equal to the value
    in the GTFS feed. When the trip_id correponds to a frequency-based
    trip defined in GTFS `frequencies.txt`, start_time is required and
    must be specified for trip updates and vehicle positions. If the
    trip corresponds to exact_times=1 GTFS record, then start_time must
    be some multiple (including zero) of headway_secs later than
    `frequencies.txt` `start_time` for the corresponding time period. If
    the trip corresponds to exact_times=0, then its start_time may be
    arbitrary, and is initially expected to be the first departure of
    the trip. Once established, the `start_time` of this frequency-based
    exact_times=0 trip should be considered immutable, even if the first
    departure time changes – that time change may instead be reflected
    in a StopTimeUpdate in a TripUpdate. Format and semantics of the
    field is same as that of GTFS/frequencies.txt/start_time, e.g.,
    `11:15:35` for 11:15:35 AM or `25:15:35` for 1:15:35 AM the day
    after the service day. (path: `informed_entity.trip.start_time`)

  - `trip_start_date`: The start date of this trip instance in YYYYMMDD
    format. For scheduled trips (trips not defined in GTFS
    frequencies.txt), this field must be provided to disambiguate trips
    that are so late as to collide with a scheduled trip on a next day.
    For example, for a train that departs 8:00 and 20:00 every day, and
    is 12 hours late, there would be two distinct trips on the same
    time. This field can be provided but is not mandatory for schedules
    in which such collisions are impossible - for example, a service
    running on hourly schedule where a vehicle that is one hour late is
    not considered to be related to schedule anymore. This field is
    required for frequency-based trips defined in GTFS
    `frequencies.txt`. (path: `informed_entity.trip.start_date`)

  - `trip_schedule_relationship`: The relation between this trip and the
    static schedule. This is not supposed to be used with alerts, but is
    provided for completeness. (path:
    `informed_entity.trip.schedule_relationship`). Possible values:

    - `SCHEDULED`: Trip that is running in accordance with its GTFS
      schedule, or is close enough to the scheduled trip to be
      associated with it.

    - `ADDED`: This value has been deprecated as the behavior was
      unspecified. Use `DUPLICATED` for an extra trip that is the same
      as a scheduled trip except the start date or time, or `NEW` for an
      extra trip that is unrelated to an existing trip.

    - `UNSCHEDULED`: A trip that is running with no schedule associated
      to it (GTFS `frequencies.txt` `exact_times`=0). Trips with
      `trip_schedule_relationship`=`UNSCHEDULED` must also set all
      `stop_schedule_relationship`=`UNSCHEDULED.`

    - `CANCELED`: A trip that existed in the schedule but was removed.

    - `REPLACEMENT`: A trip that replaces an existing trip in the
      schedule. NOTE: This field is still experimental, and subject to
      change. It may be formally adopted in the future.

    - `DUPLICATED`: An extra trip that was added in addition to a
      running schedule, for example, to replace a broken vehicle or to
      respond to sudden passenger load. Used with `trip_id`,
      `start_date`, and `start_time` to copy an existing trip from
      static GTFS but start at a different service date and/or time.
      Duplicating a trip is allowed if the service related to the
      original trip in (CSV) GTFS (in calendar.txt or
      calendar_dates.txt) is operating within the next 30 days. The trip
      to be duplicated is identified via `trip_id.` This enumeration
      does not modify the existing trip referenced by `trip_id` - if a
      producer wants to cancel the original trip, it must publish a
      separate TripUpdate with the value of `CANCELED` or `DELETED`. If
      a producer wants to replace the original trip, a value of
      `REPLACEMENT` should be used instead.

      Trips defined in GTFS `frequencies.txt` with `exact_times` that is
      empty or equal to 0 cannot be duplicated.

      Existing producers and consumers that were using the ADDED
      enumeration to represent duplicated trips must follow [the
      migration
      guide](https://github.com/google/transit/tree/master/gtfs-realtime/spec/en/examples/migration-duplicated.md)
      to transition to the `DUPLICATED` enumeration. NOTE: This field is
      still experimental, and subject to change. It may be formally
      adopted in the future.

    - `DELETED`: A trip that existed in the schedule but was removed and
      must not be shown to users. `DELETED` should be used instead of
      `CANCELED` to indicate that a transit provider would like to
      entirely remove information about the corresponding trip from
      consuming applications, so the trip is not shown as cancelled to
      riders, e.g. a trip that is entirely being replaced by another
      trip. This designation becomes particularly important if several
      trips are cancelled and replaced with substitute service. If
      consumers were to show explicit information about the
      cancellations it would distract from the more important real-time
      predictions. NOTE: This field is still experimental, and subject
      to change. It may be formally adopted in the future.

    - `NEW`: An extra trip unrelated to any existing trips, for example,
      to respond to sudden passenger load. NOTE: This field is still
      experimental, and subject to change. It may be formally adopted in
      the future.

  - `trip_modification_id`: Linkage to any modifications done to this
    trip (shape changes, removal or addition of stops). Reading trip
    modifications themselves is not yet supported (see
    [\#21](https://github.com/mattwigway/gtfsrealtime-r/issues/21)). If
    you have a feed with trip modifications, please comment on that
    issue so we are aware they exist in the wild. (path:
    `informed_entity.trip.modification_id`)

  - `stop_id`: The `stop_id` from the GTFS feed that this alert refers
    to.

- `cause`: The cause of the disruption. (path: `cause`) Possible values:

  - `UNKNOWN_CAUSE`

  - `OTHER_CAUSE`

  - `TECHNICAL_PROBLEM`

  - `STRIKE`

  - `DEMONSTRATION`

  - `ACCIDENT`

  - `HOLIDAY`

  - `WEATHER`

  - `MAINTENANCE`

  - `CONSTRUCTION`

  - `POLICE_ACTIVITY`

  - \`MEDICAL_EMERGENCY“

- `effect`: The effect of the disruption (path: `effect`). Possible
  values:

  - `NO_SERVICE`

  - `REDUCED_SERVICE`

  - `SIGNIFICANT_DELAYS`

  - `DETOUR`

  - `ADDITIONAL_SERVICE`

  - `MODIFIED_SERVICE`

  - `OTHER_EFFECT`

  - `UNKNOWN_EFFECT`

  - `STOP_MOVED`

  - `NO_EFFECT`

  - `ACCESSIBILITY_ISSUE`

- `language`: The free-text fields in alerts can be presented in
  multiple languages. If they are, rows associated with the alert will
  be duplicated for each language; this column will contain the language
  identifier (e.g. "EN", "ES"), and the remaining fields will contain
  the translated alerts in that language. Not every feed will have
  multiple languages (or lange flags at all), and it is also possible
  that some fields are translated into a particular language and others
  are left NA.

- `cause_detail`: Description of the cause of the alert that allows for
  agency-specific language; more specific than the `cause`. Caution:
  this field is still experimental, and subject to change. It may be
  formally adopted in the future.(path: `cause_detail`)

- `effect_detail`: Description of the effect of the alert that allows
  for agency-specific language; more specific than the Effect. Caution:
  this field is still experimental, and subject to change. It may be
  formally adopted in the future. (path: `effect_detail`)

- `url`: The URL which provides additional information about the alert.
  May differ for different languages. (path: `url`).

- `header_text`: Header (i.e. title) for the alert (path: `header`)

- `description_text`: Description for the alert. The information in the
  description should add to the information of the header. (path:
  `description`)

- `tts_header_text`: Text containing the alert's header to be used for
  text-to-speech implementations. This field is the text-to-speech
  version of header_text. It should contain the same information as
  `header_text` but formatted such that it can read as text-to-speech
  (for example, abbreviations removed, numbers spelled out, etc.) (path:
  `tts_header_text`)

- `tts_description_text`: Text containing a description for the alert to
  be used for text-to-speech implementations. This field is the
  text-to-speech version of
  ``` description_text``. It should contain the same information as  ```description_text“
  but formatted such that it can be read as text-to-speech (for example,
  abbreviations removed, numbers spelled out, etc.) (path:
  `tts_description_text`)

- `severity_level`: Severity of the alert. (path: `severity_level`).
  Possible values:

  - `UNKNOWN_SEVERITY`

  - `INFO`

  - `WARNING`

  - `SEVERE`

## Details

Alerts are intended to capture widespread or long-term disruptions or
changes. Trip updates (see
[`read_gtfsrt_trip_updates()`](https://projects.indicatrix.org/gtfsrealtime-r/reference/read_gtfsrt_trip_updates.md))
are better suited to providing information about day-to-day delays and
cancellations to specific trips.

Typically, GTFS-realtime feeds will contain only a single type of
entity, but if there are multiple types of entities in a single feed,
this function will read only the alerts.

## Examples

``` r
# This will read an alerts feed included with gtfsrealtime. Replace with
# the path to your own file if desired.
file = system.file("nyc-service-alerts.pb.bz2", package = "gtfsrealtime")

# Need to specify timezone so timestamps will be in local time.
read_gtfsrt_alerts(file, "America/New_York")
#>                                  id               start                 end
#> 1   MTA NYCT_lmm:planned_work:22245 2025-01-03 00:00:35                <NA>
#> 2   MTA NYCT_lmm:planned_work:22245 2025-01-03 00:00:35                <NA>
#> 3   MTA NYCT_lmm:planned_work:29838 2026-01-21 00:00:00 2026-06-30 00:00:00
#> 4   MTA NYCT_lmm:planned_work:29838 2026-01-21 00:00:00 2026-06-30 00:00:00
#> 5         MTA NYCT_lmm:alert:503501 2026-01-28 00:03:57                <NA>
#> 6         MTA NYCT_lmm:alert:503501 2026-01-28 00:03:57                <NA>
#> 7         MTA NYCT_lmm:alert:503667 2026-01-28 06:33:04                <NA>
#> 8         MTA NYCT_lmm:alert:503667 2026-01-28 06:33:04                <NA>
#> 9         MTA NYCT_lmm:alert:503667 2026-01-28 06:33:04                <NA>
#> 10  MTA NYCT_lmm:planned_work:22403 2025-01-15 00:00:00 2026-06-14 20:00:00
#> 11  MTA NYCT_lmm:planned_work:22403 2025-01-15 00:00:00 2026-06-14 20:00:00
#> 12  MTA NYCT_lmm:planned_work:29559 2025-12-31 17:00:00 2026-06-30 00:00:00
#> 13  MTA NYCT_lmm:planned_work:29559 2025-12-31 17:00:00 2026-06-30 00:00:00
#> 14  MTA NYCT_lmm:planned_work:27810 2026-01-22 00:00:00                <NA>
#> 15  MTA NYCT_lmm:planned_work:27810 2026-01-22 00:00:00                <NA>
#> 16        MTA NYCT_lmm:alert:503341 2026-01-28 00:04:36                <NA>
#> 17        MTA NYCT_lmm:alert:503341 2026-01-28 00:04:36                <NA>
#> 18        MTA NYCT_lmm:alert:503341 2026-01-28 00:04:36                <NA>
#> 19        MTA NYCT_lmm:alert:503341 2026-01-28 00:04:36                <NA>
#> 20        MTA NYCT_lmm:alert:503341 2026-01-28 00:04:36                <NA>
#> 21        MTA NYCT_lmm:alert:503341 2026-01-28 00:04:36                <NA>
#> 22        MTA NYCT_lmm:alert:503341 2026-01-28 00:04:36                <NA>
#> 23        MTA NYCT_lmm:alert:503341 2026-01-28 00:04:36                <NA>
#> 24        MTA NYCT_lmm:alert:503341 2026-01-28 00:04:36                <NA>
#> 25        MTA NYCT_lmm:alert:503341 2026-01-28 00:04:36                <NA>
#> 26        MTA NYCT_lmm:alert:503341 2026-01-28 00:04:36                <NA>
#> 27        MTA NYCT_lmm:alert:503341 2026-01-28 00:04:36                <NA>
#> 28        MTA NYCT_lmm:alert:503341 2026-01-28 00:04:36                <NA>
#> 29        MTA NYCT_lmm:alert:503341 2026-01-28 00:04:36                <NA>
#> 30        MTA NYCT_lmm:alert:503341 2026-01-28 00:04:36                <NA>
#> 31        MTA NYCT_lmm:alert:503341 2026-01-28 00:04:36                <NA>
#> 32        MTA NYCT_lmm:alert:503341 2026-01-28 00:04:36                <NA>
#> 33        MTA NYCT_lmm:alert:503341 2026-01-28 00:04:36                <NA>
#> 34        MTA NYCT_lmm:alert:503341 2026-01-28 00:04:36                <NA>
#> 35        MTA NYCT_lmm:alert:503341 2026-01-28 00:04:36                <NA>
#> 36        MTA NYCT_lmm:alert:503341 2026-01-28 00:04:36                <NA>
#> 37        MTA NYCT_lmm:alert:503341 2026-01-28 00:04:36                <NA>
#> 38        MTA NYCT_lmm:alert:503341 2026-01-28 00:04:36                <NA>
#> 39        MTA NYCT_lmm:alert:503341 2026-01-28 00:04:36                <NA>
#> 40        MTA NYCT_lmm:alert:503341 2026-01-28 00:04:36                <NA>
#> 41        MTA NYCT_lmm:alert:503341 2026-01-28 00:04:36                <NA>
#> 42        MTA NYCT_lmm:alert:503341 2026-01-28 00:04:36                <NA>
#> 43        MTA NYCT_lmm:alert:503341 2026-01-28 00:04:36                <NA>
#> 44        MTA NYCT_lmm:alert:503341 2026-01-28 00:04:36                <NA>
#> 45        MTA NYCT_lmm:alert:503341 2026-01-28 00:04:36                <NA>
#> 46        MTA NYCT_lmm:alert:503341 2026-01-28 00:04:36                <NA>
#> 47        MTA NYCT_lmm:alert:503341 2026-01-28 00:04:36                <NA>
#> 48        MTA NYCT_lmm:alert:503341 2026-01-28 00:04:36                <NA>
#> 49        MTA NYCT_lmm:alert:503341 2026-01-28 00:04:36                <NA>
#> 50        MTA NYCT_lmm:alert:503341 2026-01-28 00:04:36                <NA>
#> 51        MTA NYCT_lmm:alert:503341 2026-01-28 00:04:36                <NA>
#> 52        MTA NYCT_lmm:alert:503341 2026-01-28 00:04:36                <NA>
#> 53        MTA NYCT_lmm:alert:503341 2026-01-28 00:04:36                <NA>
#> 54        MTA NYCT_lmm:alert:503341 2026-01-28 00:04:36                <NA>
#> 55        MTA NYCT_lmm:alert:503341 2026-01-28 00:04:36                <NA>
#> 56        MTA NYCT_lmm:alert:503341 2026-01-28 00:04:36                <NA>
#> 57        MTA NYCT_lmm:alert:503341 2026-01-28 00:04:36                <NA>
#> 58        MTA NYCT_lmm:alert:503341 2026-01-28 00:04:36                <NA>
#> 59        MTA NYCT_lmm:alert:503341 2026-01-28 00:04:36                <NA>
#> 60        MTA NYCT_lmm:alert:503341 2026-01-28 00:04:36                <NA>
#> 61        MTA NYCT_lmm:alert:503341 2026-01-28 00:04:36                <NA>
#> 62        MTA NYCT_lmm:alert:503341 2026-01-28 00:04:36                <NA>
#> 63        MTA NYCT_lmm:alert:503341 2026-01-28 00:04:36                <NA>
#> 64        MTA NYCT_lmm:alert:503341 2026-01-28 00:04:36                <NA>
#> 65        MTA NYCT_lmm:alert:503341 2026-01-28 00:04:36                <NA>
#> 66        MTA NYCT_lmm:alert:503341 2026-01-28 00:04:36                <NA>
#> 67        MTA NYCT_lmm:alert:503341 2026-01-28 00:04:36                <NA>
#> 68        MTA NYCT_lmm:alert:503341 2026-01-28 00:04:36                <NA>
#> 69        MTA NYCT_lmm:alert:503341 2026-01-28 00:04:36                <NA>
#> 70        MTA NYCT_lmm:alert:503341 2026-01-28 00:04:36                <NA>
#> 71        MTA NYCT_lmm:alert:503341 2026-01-28 00:04:36                <NA>
#> 72        MTA NYCT_lmm:alert:503341 2026-01-28 00:04:36                <NA>
#> 73        MTA NYCT_lmm:alert:503781 2026-01-28 08:47:42                <NA>
#> 74        MTA NYCT_lmm:alert:503781 2026-01-28 08:47:42                <NA>
#> 75        MTA NYCT_lmm:alert:503781 2026-01-28 08:47:42                <NA>
#> 76        MTA NYCT_lmm:alert:503781 2026-01-28 08:47:42                <NA>
#> 77        MTA NYCT_lmm:alert:503781 2026-01-28 08:47:42                <NA>
#> 78        MTA NYCT_lmm:alert:503781 2026-01-28 08:47:42                <NA>
#> 79        MTA NYCT_lmm:alert:503781 2026-01-28 08:47:42                <NA>
#> 80        MTA NYCT_lmm:alert:503781 2026-01-28 08:47:42                <NA>
#> 81        MTA NYCT_lmm:alert:503781 2026-01-28 08:47:42                <NA>
#> 82        MTA NYCT_lmm:alert:503781 2026-01-28 08:47:42                <NA>
#> 83        MTA NYCT_lmm:alert:503781 2026-01-28 08:47:42                <NA>
#> 84        MTA NYCT_lmm:alert:503781 2026-01-28 08:47:42                <NA>
#> 85        MTA NYCT_lmm:alert:503781 2026-01-28 08:47:42                <NA>
#> 86        MTA NYCT_lmm:alert:503781 2026-01-28 08:47:42                <NA>
#> 87        MTA NYCT_lmm:alert:503781 2026-01-28 08:47:42                <NA>
#> 88        MTA NYCT_lmm:alert:503781 2026-01-28 08:47:42                <NA>
#> 89        MTA NYCT_lmm:alert:503781 2026-01-28 08:47:42                <NA>
#> 90        MTA NYCT_lmm:alert:503781 2026-01-28 08:47:42                <NA>
#> 91        MTA NYCT_lmm:alert:503781 2026-01-28 08:47:42                <NA>
#> 92        MTA NYCT_lmm:alert:503781 2026-01-28 08:47:42                <NA>
#> 93        MTA NYCT_lmm:alert:503781 2026-01-28 08:47:42                <NA>
#> 94        MTA NYCT_lmm:alert:503781 2026-01-28 08:47:42                <NA>
#> 95        MTA NYCT_lmm:alert:503781 2026-01-28 08:47:42                <NA>
#> 96        MTA NYCT_lmm:alert:503781 2026-01-28 08:47:42                <NA>
#> 97        MTA NYCT_lmm:alert:503781 2026-01-28 08:47:42                <NA>
#> 98        MTA NYCT_lmm:alert:503781 2026-01-28 08:47:42                <NA>
#> 99        MTA NYCT_lmm:alert:503781 2026-01-28 08:47:42                <NA>
#> 100       MTA NYCT_lmm:alert:503781 2026-01-28 08:47:42                <NA>
#> 101       MTA NYCT_lmm:alert:503781 2026-01-28 08:47:42                <NA>
#> 102       MTA NYCT_lmm:alert:503781 2026-01-28 08:47:42                <NA>
#> 103       MTA NYCT_lmm:alert:503781 2026-01-28 08:47:42                <NA>
#> 104       MTA NYCT_lmm:alert:503781 2026-01-28 08:47:42                <NA>
#> 105       MTA NYCT_lmm:alert:503781 2026-01-28 08:47:42                <NA>
#> 106       MTA NYCT_lmm:alert:503781 2026-01-28 08:47:42                <NA>
#> 107       MTA NYCT_lmm:alert:503781 2026-01-28 08:47:42                <NA>
#> 108       MTA NYCT_lmm:alert:503781 2026-01-28 08:47:42                <NA>
#> 109       MTA NYCT_lmm:alert:503781 2026-01-28 08:47:42                <NA>
#> 110       MTA NYCT_lmm:alert:503781 2026-01-28 08:47:42                <NA>
#> 111       MTA NYCT_lmm:alert:503781 2026-01-28 08:47:42                <NA>
#> 112       MTA NYCT_lmm:alert:503781 2026-01-28 08:47:42                <NA>
#> 113       MTA NYCT_lmm:alert:503781 2026-01-28 08:47:42                <NA>
#> 114       MTA NYCT_lmm:alert:503781 2026-01-28 08:47:42                <NA>
#> 115       MTA NYCT_lmm:alert:503781 2026-01-28 08:47:42                <NA>
#> 116       MTA NYCT_lmm:alert:503781 2026-01-28 08:47:42                <NA>
#> 117       MTA NYCT_lmm:alert:503781 2026-01-28 08:47:42                <NA>
#> 118       MTA NYCT_lmm:alert:503781 2026-01-28 08:47:42                <NA>
#> 119       MTA NYCT_lmm:alert:503781 2026-01-28 08:47:42                <NA>
#> 120       MTA NYCT_lmm:alert:503781 2026-01-28 08:47:42                <NA>
#> 121       MTA NYCT_lmm:alert:503781 2026-01-28 08:47:42                <NA>
#> 122       MTA NYCT_lmm:alert:503781 2026-01-28 08:47:42                <NA>
#> 123       MTA NYCT_lmm:alert:503781 2026-01-28 08:47:42                <NA>
#> 124       MTA NYCT_lmm:alert:503781 2026-01-28 08:47:42                <NA>
#> 125       MTA NYCT_lmm:alert:503781 2026-01-28 08:47:42                <NA>
#> 126       MTA NYCT_lmm:alert:503781 2026-01-28 08:47:42                <NA>
#> 127       MTA NYCT_lmm:alert:503781 2026-01-28 08:47:42                <NA>
#> 128       MTA NYCT_lmm:alert:503781 2026-01-28 08:47:42                <NA>
#> 129       MTA NYCT_lmm:alert:503781 2026-01-28 08:47:42                <NA>
#> 130       MTA NYCT_lmm:alert:503781 2026-01-28 08:47:42                <NA>
#> 131       MTA NYCT_lmm:alert:503781 2026-01-28 08:47:42                <NA>
#> 132       MTA NYCT_lmm:alert:503781 2026-01-28 08:47:42                <NA>
#> 133       MTA NYCT_lmm:alert:503781 2026-01-28 08:47:42                <NA>
#> 134       MTA NYCT_lmm:alert:503781 2026-01-28 08:47:42                <NA>
#> 135       MTA NYCT_lmm:alert:503781 2026-01-28 08:47:42                <NA>
#> 136 MTA NYCT_lmm:planned_work:22527 2025-01-27 00:00:00 2026-06-14 20:00:00
#> 137 MTA NYCT_lmm:planned_work:22527 2025-01-27 00:00:00 2026-06-14 20:00:00
#> 138 MTA NYCT_lmm:planned_work:28708 2025-11-07 00:00:00                <NA>
#> 139 MTA NYCT_lmm:planned_work:28708 2025-11-07 00:00:00                <NA>
#> 140 MTA NYCT_lmm:planned_work:21837 2024-12-04 00:00:00 2026-06-14 20:00:00
#> 141 MTA NYCT_lmm:planned_work:21837 2024-12-04 00:00:00 2026-06-14 20:00:00
#> 142 MTA NYCT_lmm:planned_work:21837 2024-12-04 00:00:00 2026-06-14 20:00:00
#> 143 MTA NYCT_lmm:planned_work:21838 2024-12-04 00:00:00 2026-06-14 20:00:00
#> 144 MTA NYCT_lmm:planned_work:21838 2024-12-04 00:00:00 2026-06-14 20:00:00
#> 145 MTA NYCT_lmm:planned_work:17935 2024-06-13 00:00:00                <NA>
#> 146 MTA NYCT_lmm:planned_work:17935 2024-06-13 00:00:00                <NA>
#> 147 MTA NYCT_lmm:planned_work:16723 2024-04-24 00:00:00 2026-04-24 00:00:00
#> 148 MTA NYCT_lmm:planned_work:16723 2024-04-24 00:00:00 2026-04-24 00:00:00
#> 149 MTA NYCT_lmm:planned_work:15159 2024-01-23 00:00:00 2026-06-14 20:00:00
#> 150 MTA NYCT_lmm:planned_work:15159 2024-01-23 00:00:00 2026-06-14 20:00:00
#> 151 MTA NYCT_lmm:planned_work:29312 2025-12-12 00:00:00 2026-06-14 20:00:00
#> 152 MTA NYCT_lmm:planned_work:29312 2025-12-12 00:00:00 2026-06-14 20:00:00
#> 153 MTA NYCT_lmm:planned_work:23011 2025-02-28 00:00:00                <NA>
#> 154 MTA NYCT_lmm:planned_work:23011 2025-02-28 00:00:00                <NA>
#> 155 MTA NYCT_lmm:planned_work:23010 2025-02-28 00:00:00                <NA>
#> 156 MTA NYCT_lmm:planned_work:23010 2025-02-28 00:00:00                <NA>
#> 157 MTA NYCT_lmm:planned_work:29849 2026-01-22 00:00:00                <NA>
#> 158 MTA NYCT_lmm:planned_work:29849 2026-01-22 00:00:00                <NA>
#> 159 MTA NYCT_lmm:planned_work:29647 2026-01-09 00:00:00                <NA>
#> 160 MTA NYCT_lmm:planned_work:29647 2026-01-09 00:00:00                <NA>
#> 161 MTA NYCT_lmm:planned_work:29922 2026-01-27 00:00:00                <NA>
#> 162 MTA NYCT_lmm:planned_work:29922 2026-01-27 00:00:00                <NA>
#> 163 MTA NYCT_lmm:planned_work:21840 2024-12-04 00:00:00 2026-06-14 20:00:00
#> 164 MTA NYCT_lmm:planned_work:21840 2024-12-04 00:00:00 2026-06-14 20:00:00
#> 165 MTA NYCT_lmm:planned_work:15485 2025-07-23 00:00:00                <NA>
#> 166 MTA NYCT_lmm:planned_work:15485 2025-07-23 00:00:00                <NA>
#> 167       MTA NYCT_lmm:alert:502386 2026-01-28 00:04:48                <NA>
#> 168       MTA NYCT_lmm:alert:502386 2026-01-28 00:04:48                <NA>
#> 169       MTA NYCT_lmm:alert:502386 2026-01-28 00:04:48                <NA>
#> 170       MTA NYCT_lmm:alert:502386 2026-01-28 00:04:48                <NA>
#> 171       MTA NYCT_lmm:alert:502386 2026-01-28 00:04:48                <NA>
#> 172       MTA NYCT_lmm:alert:502386 2026-01-28 00:04:48                <NA>
#> 173       MTA NYCT_lmm:alert:502386 2026-01-28 00:04:48                <NA>
#> 174       MTA NYCT_lmm:alert:502386 2026-01-28 00:04:48                <NA>
#> 175       MTA NYCT_lmm:alert:502386 2026-01-28 00:04:48                <NA>
#> 176       MTA NYCT_lmm:alert:502386 2026-01-28 00:04:48                <NA>
#> 177       MTA NYCT_lmm:alert:502386 2026-01-28 00:04:48                <NA>
#> 178       MTA NYCT_lmm:alert:502386 2026-01-28 00:04:48                <NA>
#> 179       MTA NYCT_lmm:alert:502386 2026-01-28 00:04:48                <NA>
#> 180       MTA NYCT_lmm:alert:502386 2026-01-28 00:04:48                <NA>
#> 181       MTA NYCT_lmm:alert:502386 2026-01-28 00:04:48                <NA>
#> 182       MTA NYCT_lmm:alert:502386 2026-01-28 00:04:48                <NA>
#> 183       MTA NYCT_lmm:alert:502386 2026-01-28 00:04:48                <NA>
#> 184       MTA NYCT_lmm:alert:502386 2026-01-28 00:04:48                <NA>
#> 185       MTA NYCT_lmm:alert:502386 2026-01-28 00:04:48                <NA>
#> 186       MTA NYCT_lmm:alert:502386 2026-01-28 00:04:48                <NA>
#> 187       MTA NYCT_lmm:alert:502386 2026-01-28 00:04:48                <NA>
#> 188       MTA NYCT_lmm:alert:502386 2026-01-28 00:04:48                <NA>
#> 189       MTA NYCT_lmm:alert:502386 2026-01-28 00:04:48                <NA>
#> 190       MTA NYCT_lmm:alert:502386 2026-01-28 00:04:48                <NA>
#> 191       MTA NYCT_lmm:alert:502386 2026-01-28 00:04:48                <NA>
#> 192       MTA NYCT_lmm:alert:502386 2026-01-28 00:04:48                <NA>
#> 193       MTA NYCT_lmm:alert:502386 2026-01-28 00:04:48                <NA>
#> 194       MTA NYCT_lmm:alert:502386 2026-01-28 00:04:48                <NA>
#> 195       MTA NYCT_lmm:alert:502386 2026-01-28 00:04:48                <NA>
#> 196       MTA NYCT_lmm:alert:502386 2026-01-28 00:04:48                <NA>
#> 197       MTA NYCT_lmm:alert:502386 2026-01-28 00:04:48                <NA>
#> 198       MTA NYCT_lmm:alert:502386 2026-01-28 00:04:48                <NA>
#> 199       MTA NYCT_lmm:alert:502386 2026-01-28 00:04:48                <NA>
#> 200       MTA NYCT_lmm:alert:502386 2026-01-28 00:04:48                <NA>
#> 201       MTA NYCT_lmm:alert:502386 2026-01-28 00:04:48                <NA>
#> 202       MTA NYCT_lmm:alert:502386 2026-01-28 00:04:48                <NA>
#> 203       MTA NYCT_lmm:alert:502386 2026-01-28 00:04:48                <NA>
#> 204       MTA NYCT_lmm:alert:502386 2026-01-28 00:04:48                <NA>
#> 205       MTA NYCT_lmm:alert:502386 2026-01-28 00:04:48                <NA>
#> 206       MTA NYCT_lmm:alert:502386 2026-01-28 00:04:48                <NA>
#> 207       MTA NYCT_lmm:alert:502386 2026-01-28 00:04:48                <NA>
#> 208       MTA NYCT_lmm:alert:502386 2026-01-28 00:04:48                <NA>
#> 209       MTA NYCT_lmm:alert:502421 2026-01-28 00:04:40                <NA>
#> 210       MTA NYCT_lmm:alert:502421 2026-01-28 00:04:40                <NA>
#> 211       MTA NYCT_lmm:alert:502421 2026-01-28 00:04:40                <NA>
#> 212       MTA NYCT_lmm:alert:502421 2026-01-28 00:04:40                <NA>
#> 213       MTA NYCT_lmm:alert:502421 2026-01-28 00:04:40                <NA>
#> 214       MTA NYCT_lmm:alert:502421 2026-01-28 00:04:40                <NA>
#> 215       MTA NYCT_lmm:alert:502421 2026-01-28 00:04:40                <NA>
#> 216       MTA NYCT_lmm:alert:502421 2026-01-28 00:04:40                <NA>
#> 217       MTA NYCT_lmm:alert:502421 2026-01-28 00:04:40                <NA>
#> 218       MTA NYCT_lmm:alert:502421 2026-01-28 00:04:40                <NA>
#> 219       MTA NYCT_lmm:alert:502421 2026-01-28 00:04:40                <NA>
#> 220       MTA NYCT_lmm:alert:502421 2026-01-28 00:04:40                <NA>
#> 221       MTA NYCT_lmm:alert:502421 2026-01-28 00:04:40                <NA>
#> 222       MTA NYCT_lmm:alert:502421 2026-01-28 00:04:40                <NA>
#> 223       MTA NYCT_lmm:alert:502421 2026-01-28 00:04:40                <NA>
#> 224       MTA NYCT_lmm:alert:502421 2026-01-28 00:04:40                <NA>
#> 225       MTA NYCT_lmm:alert:502421 2026-01-28 00:04:40                <NA>
#> 226       MTA NYCT_lmm:alert:502421 2026-01-28 00:04:40                <NA>
#> 227       MTA NYCT_lmm:alert:502421 2026-01-28 00:04:40                <NA>
#> 228       MTA NYCT_lmm:alert:502421 2026-01-28 00:04:40                <NA>
#> 229       MTA NYCT_lmm:alert:502421 2026-01-28 00:04:40                <NA>
#> 230       MTA NYCT_lmm:alert:502421 2026-01-28 00:04:40                <NA>
#> 231       MTA NYCT_lmm:alert:502421 2026-01-28 00:04:40                <NA>
#> 232       MTA NYCT_lmm:alert:502421 2026-01-28 00:04:40                <NA>
#> 233       MTA NYCT_lmm:alert:502421 2026-01-28 00:04:40                <NA>
#> 234       MTA NYCT_lmm:alert:502421 2026-01-28 00:04:40                <NA>
#> 235       MTA NYCT_lmm:alert:502421 2026-01-28 00:04:40                <NA>
#> 236       MTA NYCT_lmm:alert:502421 2026-01-28 00:04:40                <NA>
#> 237       MTA NYCT_lmm:alert:502421 2026-01-28 00:04:40                <NA>
#> 238       MTA NYCT_lmm:alert:502421 2026-01-28 00:04:40                <NA>
#> 239       MTA NYCT_lmm:alert:502421 2026-01-28 00:04:40                <NA>
#> 240       MTA NYCT_lmm:alert:502421 2026-01-28 00:04:40                <NA>
#> 241       MTA NYCT_lmm:alert:502421 2026-01-28 00:04:40                <NA>
#> 242       MTA NYCT_lmm:alert:502421 2026-01-28 00:04:40                <NA>
#> 243       MTA NYCT_lmm:alert:502421 2026-01-28 00:04:40                <NA>
#> 244       MTA NYCT_lmm:alert:502421 2026-01-28 00:04:40                <NA>
#> 245       MTA NYCT_lmm:alert:502421 2026-01-28 00:04:40                <NA>
#> 246       MTA NYCT_lmm:alert:502421 2026-01-28 00:04:40                <NA>
#> 247       MTA NYCT_lmm:alert:502421 2026-01-28 00:04:40                <NA>
#> 248       MTA NYCT_lmm:alert:502421 2026-01-28 00:04:40                <NA>
#> 249       MTA NYCT_lmm:alert:502421 2026-01-28 00:04:40                <NA>
#> 250       MTA NYCT_lmm:alert:502421 2026-01-28 00:04:40                <NA>
#> 251       MTA NYCT_lmm:alert:502421 2026-01-28 00:04:40                <NA>
#> 252       MTA NYCT_lmm:alert:502421 2026-01-28 00:04:40                <NA>
#> 253       MTA NYCT_lmm:alert:502421 2026-01-28 00:04:40                <NA>
#> 254       MTA NYCT_lmm:alert:502421 2026-01-28 00:04:40                <NA>
#> 255       MTA NYCT_lmm:alert:502421 2026-01-28 00:04:40                <NA>
#> 256       MTA NYCT_lmm:alert:502421 2026-01-28 00:04:40                <NA>
#> 257       MTA NYCT_lmm:alert:502421 2026-01-28 00:04:40                <NA>
#> 258       MTA NYCT_lmm:alert:502421 2026-01-28 00:04:40                <NA>
#> 259       MTA NYCT_lmm:alert:502421 2026-01-28 00:04:40                <NA>
#> 260       MTA NYCT_lmm:alert:502421 2026-01-28 00:04:40                <NA>
#> 261       MTA NYCT_lmm:alert:502421 2026-01-28 00:04:40                <NA>
#> 262       MTA NYCT_lmm:alert:502421 2026-01-28 00:04:40                <NA>
#> 263       MTA NYCT_lmm:alert:502421 2026-01-28 00:04:40                <NA>
#> 264       MTA NYCT_lmm:alert:502421 2026-01-28 00:04:40                <NA>
#> 265       MTA NYCT_lmm:alert:502421 2026-01-28 00:04:40                <NA>
#> 266       MTA NYCT_lmm:alert:502421 2026-01-28 00:04:40                <NA>
#> 267       MTA NYCT_lmm:alert:502421 2026-01-28 00:04:40                <NA>
#> 268       MTA NYCT_lmm:alert:502421 2026-01-28 00:04:40                <NA>
#> 269       MTA NYCT_lmm:alert:502421 2026-01-28 00:04:40                <NA>
#> 270       MTA NYCT_lmm:alert:502421 2026-01-28 00:04:40                <NA>
#> 271       MTA NYCT_lmm:alert:502421 2026-01-28 00:04:40                <NA>
#> 272       MTA NYCT_lmm:alert:502421 2026-01-28 00:04:40                <NA>
#> 273       MTA NYCT_lmm:alert:502421 2026-01-28 00:04:40                <NA>
#> 274       MTA NYCT_lmm:alert:502421 2026-01-28 00:04:40                <NA>
#> 275       MTA NYCT_lmm:alert:502421 2026-01-28 00:04:40                <NA>
#> 276       MTA NYCT_lmm:alert:502420 2026-01-28 00:04:29                <NA>
#> 277       MTA NYCT_lmm:alert:502420 2026-01-28 00:04:29                <NA>
#> 278       MTA NYCT_lmm:alert:502420 2026-01-28 00:04:29                <NA>
#> 279       MTA NYCT_lmm:alert:502420 2026-01-28 00:04:29                <NA>
#> 280       MTA NYCT_lmm:alert:502420 2026-01-28 00:04:29                <NA>
#> 281       MTA NYCT_lmm:alert:502420 2026-01-28 00:04:29                <NA>
#> 282       MTA NYCT_lmm:alert:502420 2026-01-28 00:04:29                <NA>
#> 283       MTA NYCT_lmm:alert:502420 2026-01-28 00:04:29                <NA>
#> 284       MTA NYCT_lmm:alert:502420 2026-01-28 00:04:29                <NA>
#> 285       MTA NYCT_lmm:alert:502420 2026-01-28 00:04:29                <NA>
#> 286       MTA NYCT_lmm:alert:502420 2026-01-28 00:04:29                <NA>
#> 287       MTA NYCT_lmm:alert:502420 2026-01-28 00:04:29                <NA>
#> 288       MTA NYCT_lmm:alert:502420 2026-01-28 00:04:29                <NA>
#> 289       MTA NYCT_lmm:alert:502420 2026-01-28 00:04:29                <NA>
#> 290       MTA NYCT_lmm:alert:502420 2026-01-28 00:04:29                <NA>
#> 291       MTA NYCT_lmm:alert:502420 2026-01-28 00:04:29                <NA>
#> 292       MTA NYCT_lmm:alert:502420 2026-01-28 00:04:29                <NA>
#> 293       MTA NYCT_lmm:alert:502420 2026-01-28 00:04:29                <NA>
#> 294       MTA NYCT_lmm:alert:502420 2026-01-28 00:04:29                <NA>
#> 295       MTA NYCT_lmm:alert:502420 2026-01-28 00:04:29                <NA>
#> 296       MTA NYCT_lmm:alert:502420 2026-01-28 00:04:29                <NA>
#> 297       MTA NYCT_lmm:alert:502420 2026-01-28 00:04:29                <NA>
#> 298       MTA NYCT_lmm:alert:502420 2026-01-28 00:04:29                <NA>
#> 299       MTA NYCT_lmm:alert:502420 2026-01-28 00:04:29                <NA>
#> 300       MTA NYCT_lmm:alert:502420 2026-01-28 00:04:29                <NA>
#> 301       MTA NYCT_lmm:alert:502420 2026-01-28 00:04:29                <NA>
#> 302       MTA NYCT_lmm:alert:502420 2026-01-28 00:04:29                <NA>
#> 303       MTA NYCT_lmm:alert:502420 2026-01-28 00:04:29                <NA>
#> 304       MTA NYCT_lmm:alert:502420 2026-01-28 00:04:29                <NA>
#> 305       MTA NYCT_lmm:alert:502420 2026-01-28 00:04:29                <NA>
#> 306       MTA NYCT_lmm:alert:502420 2026-01-28 00:04:29                <NA>
#> 307       MTA NYCT_lmm:alert:502420 2026-01-28 00:04:29                <NA>
#> 308       MTA NYCT_lmm:alert:502420 2026-01-28 00:04:29                <NA>
#> 309       MTA NYCT_lmm:alert:502420 2026-01-28 00:04:29                <NA>
#> 310       MTA NYCT_lmm:alert:502420 2026-01-28 00:04:29                <NA>
#> 311       MTA NYCT_lmm:alert:502420 2026-01-28 00:04:29                <NA>
#> 312       MTA NYCT_lmm:alert:502420 2026-01-28 00:04:29                <NA>
#> 313       MTA NYCT_lmm:alert:502420 2026-01-28 00:04:29                <NA>
#> 314       MTA NYCT_lmm:alert:502420 2026-01-28 00:04:29                <NA>
#> 315       MTA NYCT_lmm:alert:502420 2026-01-28 00:04:29                <NA>
#> 316       MTA NYCT_lmm:alert:502420 2026-01-28 00:04:29                <NA>
#> 317       MTA NYCT_lmm:alert:502420 2026-01-28 00:04:29                <NA>
#> 318       MTA NYCT_lmm:alert:502420 2026-01-28 00:04:29                <NA>
#> 319       MTA NYCT_lmm:alert:502420 2026-01-28 00:04:29                <NA>
#> 320       MTA NYCT_lmm:alert:502420 2026-01-28 00:04:29                <NA>
#> 321       MTA NYCT_lmm:alert:502420 2026-01-28 00:04:29                <NA>
#> 322       MTA NYCT_lmm:alert:502420 2026-01-28 00:04:29                <NA>
#> 323       MTA NYCT_lmm:alert:502420 2026-01-28 00:04:29                <NA>
#> 324       MTA NYCT_lmm:alert:502420 2026-01-28 00:04:29                <NA>
#> 325       MTA NYCT_lmm:alert:502420 2026-01-28 00:04:29                <NA>
#> 326       MTA NYCT_lmm:alert:502420 2026-01-28 00:04:29                <NA>
#> 327       MTA NYCT_lmm:alert:502420 2026-01-28 00:04:29                <NA>
#> 328       MTA NYCT_lmm:alert:502420 2026-01-28 00:04:29                <NA>
#> 329       MTA NYCT_lmm:alert:502420 2026-01-28 00:04:29                <NA>
#> 330       MTA NYCT_lmm:alert:502420 2026-01-28 00:04:29                <NA>
#> 331       MTA NYCT_lmm:alert:502420 2026-01-28 00:04:29                <NA>
#> 332       MTA NYCT_lmm:alert:502420 2026-01-28 00:04:29                <NA>
#> 333       MTA NYCT_lmm:alert:502420 2026-01-28 00:04:29                <NA>
#> 334  MTA NYCT_lmm:planned_work:9421 2023-04-01 00:00:00                <NA>
#> 335  MTA NYCT_lmm:planned_work:9421 2023-04-01 00:00:00                <NA>
#> 336 MTA NYCT_lmm:planned_work:22294 2025-01-07 00:00:00 2026-06-14 20:00:00
#> 337 MTA NYCT_lmm:planned_work:22294 2025-01-07 00:00:00 2026-06-14 20:00:00
#> 338 MTA NYCT_lmm:planned_work:18087 2024-07-25 00:00:00 2026-06-14 20:00:00
#> 339 MTA NYCT_lmm:planned_work:18087 2024-07-25 00:00:00 2026-06-14 20:00:00
#> 340 MTA NYCT_lmm:planned_work:29739 2026-01-26 00:00:00 2026-01-30 17:00:00
#> 341 MTA NYCT_lmm:planned_work:29739 2026-01-26 00:00:00 2026-01-30 17:00:00
#> 342 MTA NYCT_lmm:planned_work:26507 2025-09-01 00:00:00 2027-11-27 23:59:00
#> 343 MTA NYCT_lmm:planned_work:26507 2025-09-01 00:00:00 2027-11-27 23:59:00
#> 344       MTA NYCT_lmm:alert:503766 2026-01-28 08:22:41                <NA>
#> 345       MTA NYCT_lmm:alert:503766 2026-01-28 08:22:41                <NA>
#> 346 MTA NYCT_lmm:planned_work:29537 2026-01-06 00:00:00                <NA>
#> 347 MTA NYCT_lmm:planned_work:29537 2026-01-06 00:00:00                <NA>
#> 348 MTA NYCT_lmm:planned_work:26106 2025-07-21 00:00:00 2026-06-14 20:00:00
#> 349 MTA NYCT_lmm:planned_work:26106 2025-07-21 00:00:00 2026-06-14 20:00:00
#> 350       MTA NYCT_lmm:alert:503680 2026-01-28 06:47:00                <NA>
#> 351       MTA NYCT_lmm:alert:503680 2026-01-28 06:47:00                <NA>
#> 352 MTA NYCT_lmm:planned_work:15892 2026-01-25 00:00:00 2026-01-31 23:59:00
#> 353 MTA NYCT_lmm:planned_work:15892 2026-01-25 00:00:00 2026-01-31 23:59:00
#> 354 MTA NYCT_lmm:planned_work:25937 2025-07-11 00:00:00                <NA>
#> 355 MTA NYCT_lmm:planned_work:25937 2025-07-11 00:00:00                <NA>
#> 356       MTA NYCT_lmm:alert:503685 2026-01-28 06:48:55                <NA>
#> 357       MTA NYCT_lmm:alert:503685 2026-01-28 06:48:55                <NA>
#> 358       MTA NYCT_lmm:alert:503685 2026-01-28 06:48:55                <NA>
#> 359 MTA NYCT_lmm:planned_work:10128 2026-01-28 00:00:00 2026-01-28 23:59:00
#> 360 MTA NYCT_lmm:planned_work:10128 2026-01-28 00:00:00 2026-01-28 23:59:00
#> 361 MTA NYCT_lmm:planned_work:29895 2026-01-25 10:00:00 2026-01-31 01:00:00
#> 362 MTA NYCT_lmm:planned_work:29895 2026-01-25 10:00:00 2026-01-31 01:00:00
#> 363 MTA NYCT_lmm:planned_work:29733 2026-01-13 18:45:00                <NA>
#> 364 MTA NYCT_lmm:planned_work:29733 2026-01-13 18:45:00                <NA>
#> 365 MTA NYCT_lmm:planned_work:29733 2026-01-13 18:45:00                <NA>
#> 366 MTA NYCT_lmm:planned_work:29733 2026-01-13 18:45:00                <NA>
#> 367 MTA NYCT_lmm:planned_work:29733 2026-01-13 18:45:00                <NA>
#> 368 MTA NYCT_lmm:planned_work:29733 2026-01-13 18:45:00                <NA>
#> 369 MTA NYCT_lmm:planned_work:29733 2026-01-13 18:45:00                <NA>
#> 370 MTA NYCT_lmm:planned_work:29897 2026-01-25 00:00:00 2026-04-12 00:00:00
#> 371 MTA NYCT_lmm:planned_work:29897 2026-01-25 00:00:00 2026-04-12 00:00:00
#> 372  MTA NYCT_lmm:planned_work:9438 2026-01-28 04:35:00 2026-01-28 09:00:00
#> 373  MTA NYCT_lmm:planned_work:9438 2026-01-28 04:35:00 2026-01-28 09:00:00
#> 374 MTA NYCT_lmm:planned_work:23566 2025-04-02 00:00:00 2026-06-14 20:00:00
#> 375 MTA NYCT_lmm:planned_work:23566 2025-04-02 00:00:00 2026-06-14 20:00:00
#> 376 MTA NYCT_lmm:planned_work:25588 2026-01-28 05:20:00 2026-01-29 00:43:00
#> 377 MTA NYCT_lmm:planned_work:25588 2026-01-28 05:20:00 2026-01-29 00:43:00
#> 378 MTA NYCT_lmm:planned_work:23569 2025-04-02 00:00:00 2026-06-14 20:00:00
#> 379 MTA NYCT_lmm:planned_work:23569 2025-04-02 00:00:00 2026-06-14 20:00:00
#> 380 MTA NYCT_lmm:planned_work:23569 2025-04-02 00:00:00 2026-06-14 20:00:00
#> 381 MTA NYCT_lmm:planned_work:26875 2025-09-02 00:00:00                <NA>
#> 382 MTA NYCT_lmm:planned_work:26875 2025-09-02 00:00:00                <NA>
#> 383 MTA NYCT_lmm:planned_work:29747 2026-01-15 00:00:00 2026-01-31 00:00:00
#> 384 MTA NYCT_lmm:planned_work:29747 2026-01-15 00:00:00 2026-01-31 00:00:00
#> 385       MTA NYCT_lmm:alert:503651 2026-01-28 06:10:58                <NA>
#> 386       MTA NYCT_lmm:alert:503651 2026-01-28 06:10:58                <NA>
#> 387 MTA NYCT_lmm:planned_work:27928 2025-10-07 00:00:00 2026-06-14 20:00:00
#> 388 MTA NYCT_lmm:planned_work:27928 2025-10-07 00:00:00 2026-06-14 20:00:00
#> 389       MTA NYCT_lmm:alert:503454 2026-01-28 00:03:16                <NA>
#> 390       MTA NYCT_lmm:alert:503454 2026-01-28 00:03:16                <NA>
#> 391       MTA NYCT_lmm:alert:503652 2026-01-28 06:11:25                <NA>
#> 392       MTA NYCT_lmm:alert:503652 2026-01-28 06:11:25                <NA>
#> 393       MTA NYCT_lmm:alert:503652 2026-01-28 06:11:25                <NA>
#> 394       MTA NYCT_lmm:alert:503652 2026-01-28 06:11:25                <NA>
#> 395 MTA NYCT_lmm:planned_work:26871 2025-08-25 00:00:00                <NA>
#> 396 MTA NYCT_lmm:planned_work:26871 2025-08-25 00:00:00                <NA>
#> 397 MTA NYCT_lmm:planned_work:29303 2025-12-12 00:00:00 2026-06-14 20:00:00
#> 398 MTA NYCT_lmm:planned_work:29303 2025-12-12 00:00:00 2026-06-14 20:00:00
#> 399 MTA NYCT_lmm:planned_work:29188 2025-12-09 00:00:00 2026-06-01 00:00:00
#> 400 MTA NYCT_lmm:planned_work:29188 2025-12-09 00:00:00 2026-06-01 00:00:00
#> 401    MTABC_lmm:planned_work:29851 2026-01-22 00:00:00                <NA>
#> 402    MTABC_lmm:planned_work:29851 2026-01-22 00:00:00                <NA>
#> 403    MTABC_lmm:planned_work:29851 2026-01-22 00:00:00                <NA>
#> 404    MTABC_lmm:planned_work:29851 2026-01-22 00:00:00                <NA>
#> 405    MTABC_lmm:planned_work:29764 2026-01-16 00:00:00 2028-01-01 00:00:00
#> 406    MTABC_lmm:planned_work:29764 2026-01-16 00:00:00 2028-01-01 00:00:00
#> 407          MTABC_lmm:alert:503486 2026-01-28 00:02:24                <NA>
#> 408          MTABC_lmm:alert:503486 2026-01-28 00:02:24                <NA>
#> 409    MTABC_lmm:planned_work:25729 2025-06-28 00:00:00 2026-06-14 20:00:00
#> 410    MTABC_lmm:planned_work:25729 2025-06-28 00:00:00 2026-06-14 20:00:00
#> 411          MTABC_lmm:alert:503679 2026-01-28 06:46:36 2026-01-28 09:30:52
#> 412          MTABC_lmm:alert:503679 2026-01-28 06:46:36 2026-01-28 09:30:52
#> 413          MTABC_lmm:alert:503418 2026-01-28 00:02:42                <NA>
#> 414          MTABC_lmm:alert:503418 2026-01-28 00:02:42                <NA>
#> 415    MTABC_lmm:planned_work:26997 2025-08-31 00:00:00                <NA>
#> 416    MTABC_lmm:planned_work:26997 2025-08-31 00:00:00                <NA>
#> 417    MTABC_lmm:planned_work:16211 2024-04-01 00:00:00 2026-06-14 20:00:00
#> 418    MTABC_lmm:planned_work:16211 2024-04-01 00:00:00 2026-06-14 20:00:00
#> 419          MTABC_lmm:alert:503676 2026-01-28 06:44:52 2026-01-28 09:30:16
#> 420          MTABC_lmm:alert:503676 2026-01-28 06:44:52 2026-01-28 09:30:16
#> 421    MTABC_lmm:planned_work:16452 2024-04-12 00:00:05                <NA>
#> 422    MTABC_lmm:planned_work:16452 2024-04-12 00:00:05                <NA>
#> 423    MTABC_lmm:planned_work:26873 2025-08-25 00:00:00                <NA>
#> 424    MTABC_lmm:planned_work:26873 2025-08-25 00:00:00                <NA>
#> 425          MTABC_lmm:alert:503149 2026-01-28 00:27:16                <NA>
#> 426          MTABC_lmm:alert:503149 2026-01-28 00:27:16                <NA>
#> 427    MTABC_lmm:planned_work:26872 2025-08-25 00:00:00                <NA>
#> 428    MTABC_lmm:planned_work:26872 2025-08-25 00:00:00                <NA>
#> 429    MTABC_lmm:planned_work:26464 2025-08-05 00:00:00                <NA>
#> 430    MTABC_lmm:planned_work:26464 2025-08-05 00:00:00                <NA>
#>     agency_id route_id route_type direction_id trip_trip_id trip_route_id
#> 1    MTA NYCT      X38         NA           NA         <NA>          <NA>
#> 2    MTA NYCT      X28         NA           NA         <NA>          <NA>
#> 3    MTA NYCT     <NA>         NA           NA         <NA>          M15+
#> 4    MTA NYCT     <NA>         NA           NA         <NA>          M15+
#> 5    MTA NYCT     <NA>         NA           NA         <NA>           S66
#> 6    MTA NYCT     <NA>         NA           NA         <NA>           S66
#> 7    MTA NYCT       B9         NA           NA         <NA>          <NA>
#> 8    MTA NYCT      B63         NA           NA         <NA>          <NA>
#> 9    MTA NYCT      B35         NA           NA         <NA>          <NA>
#> 10   MTA NYCT     <NA>         NA           NA         <NA>          BX39
#> 11   MTA NYCT     <NA>         NA           NA         <NA>          BX39
#> 12   MTA NYCT     <NA>         NA           NA         <NA>            M5
#> 13   MTA NYCT     <NA>         NA           NA         <NA>            M5
#> 14   MTA NYCT     <NA>         NA           NA         <NA>           M66
#> 15   MTA NYCT     <NA>         NA           NA         <NA>           M66
#> 16   MTA NYCT     BX34         NA           NA         <NA>          <NA>
#> 17   MTA NYCT     BX26         NA           NA         <NA>          <NA>
#> 18   MTA NYCT     BX27         NA           NA         <NA>          <NA>
#> 19   MTA NYCT     BX38         NA           NA         <NA>          <NA>
#> 20   MTA NYCT      BX4         NA           NA         <NA>          <NA>
#> 21      MTABC     BXM2         NA           NA         <NA>          <NA>
#> 22   MTA NYCT     BX22         NA           NA         <NA>          <NA>
#> 23   MTA NYCT     BX33         NA           NA         <NA>          <NA>
#> 24   MTA NYCT     BX20         NA           NA         <NA>          <NA>
#> 25      MTABC     BXM3         NA           NA         <NA>          <NA>
#> 26   MTA NYCT     BX21         NA           NA         <NA>          <NA>
#> 27   MTA NYCT    BX18B         NA           NA         <NA>          <NA>
#> 28      MTABC     BXM7         NA           NA         <NA>          <NA>
#> 29   MTA NYCT     BX40         NA           NA         <NA>          <NA>
#> 30   MTA NYCT     BX24         NA           NA         <NA>          <NA>
#> 31   MTA NYCT     BX41         NA           NA         <NA>          <NA>
#> 32   MTA NYCT     BX32         NA           NA         <NA>          <NA>
#> 33   MTA NYCT     BX15         NA           NA         <NA>          <NA>
#> 34   MTA NYCT      BX1         NA           NA         <NA>          <NA>
#> 35   MTA NYCT     BX13         NA           NA         <NA>          <NA>
#> 36      MTABC     BXM1         NA           NA         <NA>          <NA>
#> 37   MTA NYCT     BX28         NA           NA         <NA>          <NA>
#> 38   MTA NYCT     BX12         NA           NA         <NA>          <NA>
#> 39   MTA NYCT     BX42         NA           NA         <NA>          <NA>
#> 40      MTABC     BXM4         NA           NA         <NA>          <NA>
#> 41   MTA NYCT    BX41+         NA           NA         <NA>          <NA>
#> 42   MTA NYCT     BX31         NA           NA         <NA>          <NA>
#> 43   MTA NYCT     BX11         NA           NA         <NA>          <NA>
#> 44   MTA NYCT     BX10         NA           NA         <NA>          <NA>
#> 45      MTABC    BXM11         NA           NA         <NA>          <NA>
#> 46      MTABC     BXM9         NA           NA         <NA>          <NA>
#> 47   MTA NYCT     BX6+         NA           NA         <NA>          <NA>
#> 48   MTA NYCT    BX12+         NA           NA         <NA>          <NA>
#> 49   MTA NYCT      BX8         NA           NA         <NA>          <NA>
#> 50   MTA NYCT     BX35         NA           NA         <NA>          <NA>
#> 51   MTA NYCT     BX4A         NA           NA         <NA>          <NA>
#> 52   MTA NYCT     BX30         NA           NA         <NA>          <NA>
#> 53   MTA NYCT     BX39         NA           NA         <NA>          <NA>
#> 54   MTA NYCT      BX9         NA           NA         <NA>          <NA>
#> 55      MTABC    BXM18         NA           NA         <NA>          <NA>
#> 56      MTABC     BXM8         NA           NA         <NA>          <NA>
#> 57      MTABC     BXM6         NA           NA         <NA>          <NA>
#> 58   MTA NYCT     BX17         NA           NA         <NA>          <NA>
#> 59   MTA NYCT      BX5         NA           NA         <NA>          <NA>
#> 60   MTA NYCT     BX19         NA           NA         <NA>          <NA>
#> 61      MTABC    BXM10         NA           NA         <NA>          <NA>
#> 62   MTA NYCT     BX36         NA           NA         <NA>          <NA>
#> 63   MTA NYCT     BX16         NA           NA         <NA>          <NA>
#> 64   MTA NYCT    BX18A         NA           NA         <NA>          <NA>
#> 65   MTA NYCT      BX6         NA           NA         <NA>          <NA>
#> 66   MTA NYCT      BX3         NA           NA         <NA>          <NA>
#> 67   MTA NYCT     BX46         NA           NA         <NA>          <NA>
#> 68   MTA NYCT      BX2         NA           NA         <NA>          <NA>
#> 69   MTA NYCT     BX29         NA           NA         <NA>          <NA>
#> 70      MTABC     BX23         NA           NA         <NA>          <NA>
#> 71   MTA NYCT      BX7         NA           NA         <NA>          <NA>
#> 72   MTA NYCT     BX25         NA           NA         <NA>          <NA>
#> 73      MTABC      Q18         NA           NA         <NA>          <NA>
#> 74   MTA NYCT      Q43         NA           NA         <NA>          <NA>
#> 75      MTABC      QM4         NA           NA         <NA>          <NA>
#> 76      MTABC      QM6         NA           NA         <NA>          <NA>
#> 77   MTA NYCT     Q44+         NA           NA         <NA>          <NA>
#> 78      MTABC     Q102         NA           NA         <NA>          <NA>
#> 79   MTA NYCT      Q13         NA           NA         <NA>          <NA>
#> 80   MTA NYCT      Q17         NA           NA         <NA>          <NA>
#> 81      MTABC      Q19         NA           NA         <NA>          <NA>
#> 82   MTA NYCT       Q5         NA           NA         <NA>          <NA>
#> 83      MTABC      QM8         NA           NA         <NA>          <NA>
#> 84   MTA NYCT      Q46         NA           NA         <NA>          <NA>
#> 85   MTA NYCT      Q77         NA           NA         <NA>          <NA>
#> 86   MTA NYCT       Q1         NA           NA         <NA>          <NA>
#> 87   MTA NYCT      Q88         NA           NA         <NA>          <NA>
#> 88      MTABC     QM24         NA           NA         <NA>          <NA>
#> 89      MTABC     QM44         NA           NA         <NA>          <NA>
#> 90   MTA NYCT      Q12         NA           NA         <NA>          <NA>
#> 91      MTABC     QM20         NA           NA         <NA>          <NA>
#> 92   MTA NYCT      Q30         NA           NA         <NA>          <NA>
#> 93   MTA NYCT      Q87         NA           NA         <NA>          <NA>
#> 94   MTA NYCT      Q29         NA           NA         <NA>          <NA>
#> 95      MTABC     QM11         NA           NA         <NA>          <NA>
#> 96   MTA NYCT      Q39         NA           NA         <NA>          <NA>
#> 97   MTA NYCT      Q16         NA           NA         <NA>          <NA>
#> 98      MTABC      Q66         NA           NA         <NA>          <NA>
#> 99   MTA NYCT      Q48         NA           NA         <NA>          <NA>
#> 100     MTABC     QM34         NA           NA         <NA>          <NA>
#> 101     MTABC      QM2         NA           NA         <NA>          <NA>
#> 102     MTABC      Q32         NA           NA         <NA>          <NA>
#> 103     MTABC     Q100         NA           NA         <NA>          <NA>
#> 104     MTABC      Q69         NA           NA         <NA>          <NA>
#> 105     MTABC      Q28         NA           NA         <NA>          <NA>
#> 106     MTABC     QM12         NA           NA         <NA>          <NA>
#> 107  MTA NYCT      Q67         NA           NA         <NA>          <NA>
#> 108  MTA NYCT      Q20         NA           NA         <NA>          <NA>
#> 109  MTA NYCT       Q3         NA           NA         <NA>          <NA>
#> 110     MTABC      Q25         NA           NA         <NA>          <NA>
#> 111     MTABC     Q53+         NA           NA         <NA>          <NA>
#> 112     MTABC      Q26         NA           NA         <NA>          <NA>
#> 113     MTABC     QM35         NA           NA         <NA>          <NA>
#> 114  MTA NYCT      Q15         NA           NA         <NA>          <NA>
#> 115     MTABC      Q33         NA           NA         <NA>          <NA>
#> 116  MTA NYCT      Q36         NA           NA         <NA>          <NA>
#> 117  MTA NYCT     QM68         NA           NA         <NA>          <NA>
#> 118     MTABC      QM1         NA           NA         <NA>          <NA>
#> 119     MTABC      QM5         NA           NA         <NA>          <NA>
#> 120  MTA NYCT       Q4         NA           NA         <NA>          <NA>
#> 121     MTABC      Q47         NA           NA         <NA>          <NA>
#> 122  MTA NYCT     QM63         NA           NA         <NA>          <NA>
#> 123  MTA NYCT      Q31         NA           NA         <NA>          <NA>
#> 124     MTABC     QM10         NA           NA         <NA>          <NA>
#> 125     MTABC      Q23         NA           NA         <NA>          <NA>
#> 126  MTA NYCT       Q2         NA           NA         <NA>          <NA>
#> 127     MTABC      QM7         NA           NA         <NA>          <NA>
#> 128  MTA NYCT      Q45         NA           NA         <NA>          <NA>
#> 129  MTA NYCT      Q27         NA           NA         <NA>          <NA>
#> 130  MTA NYCT     QM64         NA           NA         <NA>          <NA>
#> 131     MTABC     Q104         NA           NA         <NA>          <NA>
#> 132     MTABC      Q49         NA           NA         <NA>          <NA>
#> 133     MTABC     QM25         NA           NA         <NA>          <NA>
#> 134     MTABC     QM32         NA           NA         <NA>          <NA>
#> 135     MTABC     Q101         NA           NA         <NA>          <NA>
#> 136  MTA NYCT     <NA>         NA           NA         <NA>           B44
#> 137  MTA NYCT     <NA>         NA           NA         <NA>           B44
#> 138  MTA NYCT      B57         NA           NA         <NA>          <NA>
#> 139  MTA NYCT      B61         NA           NA         <NA>          <NA>
#> 140  MTA NYCT     M102         NA           NA         <NA>          <NA>
#> 141  MTA NYCT     M101         NA           NA         <NA>          <NA>
#> 142  MTA NYCT     M103         NA           NA         <NA>          <NA>
#> 143  MTA NYCT    SIM22         NA           NA         <NA>          <NA>
#> 144  MTA NYCT    SIM26         NA           NA         <NA>          <NA>
#> 145  MTA NYCT     <NA>         NA           NA         <NA>            B8
#> 146  MTA NYCT     <NA>         NA           NA         <NA>            B8
#> 147  MTA NYCT     <NA>         NA           NA         <NA>           M66
#> 148  MTA NYCT     <NA>         NA           NA         <NA>           M66
#> 149  MTA NYCT     <NA>         NA           NA         <NA>           B44
#> 150  MTA NYCT     <NA>         NA           NA         <NA>           B44
#> 151  MTA NYCT     M15+         NA           NA         <NA>          <NA>
#> 152  MTA NYCT      M15         NA           NA         <NA>          <NA>
#> 153  MTA NYCT     <NA>         NA           NA         <NA>           B65
#> 154  MTA NYCT     <NA>         NA           NA         <NA>           B65
#> 155  MTA NYCT     <NA>         NA           NA         <NA>           B61
#> 156  MTA NYCT     <NA>         NA           NA         <NA>           B61
#> 157  MTA NYCT      Q54         NA           NA         <NA>          <NA>
#> 158  MTA NYCT      Q59         NA           NA         <NA>          <NA>
#> 159  MTA NYCT     <NA>         NA           NA         <NA>           M66
#> 160  MTA NYCT     <NA>         NA           NA         <NA>           M66
#> 161  MTA NYCT     <NA>         NA           NA         <NA>           M15
#> 162  MTA NYCT     <NA>         NA           NA         <NA>           M15
#> 163  MTA NYCT     SIM6         NA           NA         <NA>          <NA>
#> 164  MTA NYCT    SIM11         NA           NA         <NA>          <NA>
#> 165  MTA NYCT     <NA>         NA           NA         <NA>            M8
#> 166  MTA NYCT     <NA>         NA           NA         <NA>            M8
#> 167  MTA NYCT     M100         NA           NA         <NA>          <NA>
#> 168  MTA NYCT      M42         NA           NA         <NA>          <NA>
#> 169  MTA NYCT      M21         NA           NA         <NA>          <NA>
#> 170  MTA NYCT      M57         NA           NA         <NA>          <NA>
#> 171  MTA NYCT    M14D+         NA           NA         <NA>          <NA>
#> 172  MTA NYCT     M101         NA           NA         <NA>          <NA>
#> 173  MTA NYCT     M104         NA           NA         <NA>          <NA>
#> 174  MTA NYCT     M79+         NA           NA         <NA>          <NA>
#> 175  MTA NYCT      M10         NA           NA         <NA>          <NA>
#> 176  MTA NYCT      M22         NA           NA         <NA>          <NA>
#> 177  MTA NYCT     M15+         NA           NA         <NA>          <NA>
#> 178  MTA NYCT      M50         NA           NA         <NA>          <NA>
#> 179  MTA NYCT       M9         NA           NA         <NA>          <NA>
#> 180  MTA NYCT       M5         NA           NA         <NA>          <NA>
#> 181  MTA NYCT      M35         NA           NA         <NA>          <NA>
#> 182  MTA NYCT     M103         NA           NA         <NA>          <NA>
#> 183  MTA NYCT    M14A+         NA           NA         <NA>          <NA>
#> 184  MTA NYCT       M4         NA           NA         <NA>          <NA>
#> 185  MTA NYCT      M66         NA           NA         <NA>          <NA>
#> 186  MTA NYCT      M72         NA           NA         <NA>          <NA>
#> 187  MTA NYCT     M102         NA           NA         <NA>          <NA>
#> 188  MTA NYCT     M60+         NA           NA         <NA>          <NA>
#> 189  MTA NYCT     M23+         NA           NA         <NA>          <NA>
#> 190  MTA NYCT    M34A+         NA           NA         <NA>          <NA>
#> 191  MTA NYCT      M20         NA           NA         <NA>          <NA>
#> 192  MTA NYCT      M55         NA           NA         <NA>          <NA>
#> 193  MTA NYCT     M125         NA           NA         <NA>          <NA>
#> 194  MTA NYCT      M98         NA           NA         <NA>          <NA>
#> 195  MTA NYCT       M2         NA           NA         <NA>          <NA>
#> 196  MTA NYCT      M12         NA           NA         <NA>          <NA>
#> 197  MTA NYCT     M116         NA           NA         <NA>          <NA>
#> 198  MTA NYCT     M34+         NA           NA         <NA>          <NA>
#> 199  MTA NYCT      M31         NA           NA         <NA>          <NA>
#> 200  MTA NYCT       M1         NA           NA         <NA>          <NA>
#> 201  MTA NYCT      M11         NA           NA         <NA>          <NA>
#> 202  MTA NYCT      M96         NA           NA         <NA>          <NA>
#> 203  MTA NYCT     M106         NA           NA         <NA>          <NA>
#> 204  MTA NYCT     M86+         NA           NA         <NA>          <NA>
#> 205  MTA NYCT      M15         NA           NA         <NA>          <NA>
#> 206  MTA NYCT       M3         NA           NA         <NA>          <NA>
#> 207  MTA NYCT       M7         NA           NA         <NA>          <NA>
#> 208  MTA NYCT       M8         NA           NA         <NA>          <NA>
#> 209  MTA NYCT      B47         NA           NA         <NA>          <NA>
#> 210  MTA NYCT      B38         NA           NA         <NA>          <NA>
#> 211  MTA NYCT      B24         NA           NA         <NA>          <NA>
#> 212  MTA NYCT      B84         NA           NA         <NA>          <NA>
#> 213  MTA NYCT      B52         NA           NA         <NA>          <NA>
#> 214  MTA NYCT      B41         NA           NA         <NA>          <NA>
#> 215  MTA NYCT      B63         NA           NA         <NA>          <NA>
#> 216     MTABC     B100         NA           NA         <NA>          <NA>
#> 217  MTA NYCT       B3         NA           NA         <NA>          <NA>
#> 218  MTA NYCT      B74         NA           NA         <NA>          <NA>
#> 219  MTA NYCT      B67         NA           NA         <NA>          <NA>
#> 220  MTA NYCT       B4         NA           NA         <NA>          <NA>
#> 221  MTA NYCT      X38         NA           NA         <NA>          <NA>
#> 222  MTA NYCT      B14         NA           NA         <NA>          <NA>
#> 223     MTABC      BM1         NA           NA         <NA>          <NA>
#> 224  MTA NYCT      B70         NA           NA         <NA>          <NA>
#> 225  MTA NYCT      B25         NA           NA         <NA>          <NA>
#> 226  MTA NYCT      B44         NA           NA         <NA>          <NA>
#> 227  MTA NYCT      B39         NA           NA         <NA>          <NA>
#> 228  MTA NYCT      B62         NA           NA         <NA>          <NA>
#> 229  MTA NYCT       B8         NA           NA         <NA>          <NA>
#> 230  MTA NYCT      B15         NA           NA         <NA>          <NA>
#> 231  MTA NYCT      B20         NA           NA         <NA>          <NA>
#> 232  MTA NYCT      B82         NA           NA         <NA>          <NA>
#> 233  MTA NYCT      B83         NA           NA         <NA>          <NA>
#> 234  MTA NYCT      B12         NA           NA         <NA>          <NA>
#> 235  MTA NYCT     B82+         NA           NA         <NA>          <NA>
#> 236  MTA NYCT      B61         NA           NA         <NA>          <NA>
#> 237  MTA NYCT      B17         NA           NA         <NA>          <NA>
#> 238  MTA NYCT      B65         NA           NA         <NA>          <NA>
#> 239     MTABC      BM3         NA           NA         <NA>          <NA>
#> 240  MTA NYCT      B68         NA           NA         <NA>          <NA>
#> 241  MTA NYCT      B60         NA           NA         <NA>          <NA>
#> 242  MTA NYCT      B36         NA           NA         <NA>          <NA>
#> 243  MTA NYCT      B54         NA           NA         <NA>          <NA>
#> 244  MTA NYCT      X28         NA           NA         <NA>          <NA>
#> 245  MTA NYCT      B16         NA           NA         <NA>          <NA>
#> 246  MTA NYCT      X37         NA           NA         <NA>          <NA>
#> 247  MTA NYCT      B45         NA           NA         <NA>          <NA>
#> 248  MTA NYCT      B31         NA           NA         <NA>          <NA>
#> 249  MTA NYCT      B26         NA           NA         <NA>          <NA>
#> 250     MTABC     B103         NA           NA         <NA>          <NA>
#> 251  MTA NYCT     B111         NA           NA         <NA>          <NA>
#> 252  MTA NYCT      B49         NA           NA         <NA>          <NA>
#> 253  MTA NYCT       B9         NA           NA         <NA>          <NA>
#> 254  MTA NYCT       B7         NA           NA         <NA>          <NA>
#> 255  MTA NYCT      B69         NA           NA         <NA>          <NA>
#> 256  MTA NYCT      B13         NA           NA         <NA>          <NA>
#> 257  MTA NYCT     B46+         NA           NA         <NA>          <NA>
#> 258     MTABC      BM2         NA           NA         <NA>          <NA>
#> 259  MTA NYCT      B48         NA           NA         <NA>          <NA>
#> 260  MTA NYCT       B6         NA           NA         <NA>          <NA>
#> 261  MTA NYCT      B57         NA           NA         <NA>          <NA>
#> 262  MTA NYCT      B32         NA           NA         <NA>          <NA>
#> 263  MTA NYCT     B44+         NA           NA         <NA>          <NA>
#> 264  MTA NYCT      B43         NA           NA         <NA>          <NA>
#> 265  MTA NYCT      B35         NA           NA         <NA>          <NA>
#> 266  MTA NYCT      X27         NA           NA         <NA>          <NA>
#> 267     MTABC      BM5         NA           NA         <NA>          <NA>
#> 268  MTA NYCT      B46         NA           NA         <NA>          <NA>
#> 269     MTABC      BM4         NA           NA         <NA>          <NA>
#> 270  MTA NYCT      B42         NA           NA         <NA>          <NA>
#> 271  MTA NYCT       B1         NA           NA         <NA>          <NA>
#> 272  MTA NYCT      B11         NA           NA         <NA>          <NA>
#> 273  MTA NYCT      B64         NA           NA         <NA>          <NA>
#> 274  MTA NYCT      B37         NA           NA         <NA>          <NA>
#> 275  MTA NYCT       B2         NA           NA         <NA>          <NA>
#> 276  MTA NYCT    SIM10         NA           NA         <NA>          <NA>
#> 277  MTA NYCT      S98         NA           NA         <NA>          <NA>
#> 278  MTA NYCT    SIM35         NA           NA         <NA>          <NA>
#> 279  MTA NYCT    SIM30         NA           NA         <NA>          <NA>
#> 280  MTA NYCT     SIM1         NA           NA         <NA>          <NA>
#> 281  MTA NYCT     SIM6         NA           NA         <NA>          <NA>
#> 282  MTA NYCT      S91         NA           NA         <NA>          <NA>
#> 283  MTA NYCT      S42         NA           NA         <NA>          <NA>
#> 284  MTA NYCT      S93         NA           NA         <NA>          <NA>
#> 285  MTA NYCT    SIM31         NA           NA         <NA>          <NA>
#> 286  MTA NYCT    SIM25         NA           NA         <NA>          <NA>
#> 287  MTA NYCT    SIM32         NA           NA         <NA>          <NA>
#> 288  MTA NYCT      S54         NA           NA         <NA>          <NA>
#> 289  MTA NYCT    SIM3C         NA           NA         <NA>          <NA>
#> 290  MTA NYCT    SIM1C         NA           NA         <NA>          <NA>
#> 291  MTA NYCT      S74         NA           NA         <NA>          <NA>
#> 292  MTA NYCT      S59         NA           NA         <NA>          <NA>
#> 293  MTA NYCT     SIM7         NA           NA         <NA>          <NA>
#> 294  MTA NYCT     S79+         NA           NA         <NA>          <NA>
#> 295  MTA NYCT      S44         NA           NA         <NA>          <NA>
#> 296  MTA NYCT    SIM34         NA           NA         <NA>          <NA>
#> 297  MTA NYCT      S52         NA           NA         <NA>          <NA>
#> 298  MTA NYCT    SIM24         NA           NA         <NA>          <NA>
#> 299  MTA NYCT      S84         NA           NA         <NA>          <NA>
#> 300  MTA NYCT      S51         NA           NA         <NA>          <NA>
#> 301  MTA NYCT      S53         NA           NA         <NA>          <NA>
#> 302  MTA NYCT      S96         NA           NA         <NA>          <NA>
#> 303  MTA NYCT      S76         NA           NA         <NA>          <NA>
#> 304  MTA NYCT      S56         NA           NA         <NA>          <NA>
#> 305  MTA NYCT   SIM33C         NA           NA         <NA>          <NA>
#> 306  MTA NYCT    SIM23         NA           NA         <NA>          <NA>
#> 307  MTA NYCT      S46         NA           NA         <NA>          <NA>
#> 308  MTA NYCT      S62         NA           NA         <NA>          <NA>
#> 309  MTA NYCT      S78         NA           NA         <NA>          <NA>
#> 310  MTA NYCT     SIM9         NA           NA         <NA>          <NA>
#> 311  MTA NYCT      S66         NA           NA         <NA>          <NA>
#> 312  MTA NYCT      S61         NA           NA         <NA>          <NA>
#> 313  MTA NYCT      S55         NA           NA         <NA>          <NA>
#> 314  MTA NYCT     SIM8         NA           NA         <NA>          <NA>
#> 315  MTA NYCT      S57         NA           NA         <NA>          <NA>
#> 316  MTA NYCT      S94         NA           NA         <NA>          <NA>
#> 317  MTA NYCT     SIM4         NA           NA         <NA>          <NA>
#> 318  MTA NYCT      S92         NA           NA         <NA>          <NA>
#> 319  MTA NYCT    SIM11         NA           NA         <NA>          <NA>
#> 320  MTA NYCT      S86         NA           NA         <NA>          <NA>
#> 321  MTA NYCT     SIM3         NA           NA         <NA>          <NA>
#> 322  MTA NYCT     SIM5         NA           NA         <NA>          <NA>
#> 323  MTA NYCT     SIM2         NA           NA         <NA>          <NA>
#> 324  MTA NYCT    SIM26         NA           NA         <NA>          <NA>
#> 325  MTA NYCT      S89         NA           NA         <NA>          <NA>
#> 326  MTA NYCT    SIM33         NA           NA         <NA>          <NA>
#> 327  MTA NYCT      S48         NA           NA         <NA>          <NA>
#> 328  MTA NYCT    SIM4C         NA           NA         <NA>          <NA>
#> 329  MTA NYCT      S81         NA           NA         <NA>          <NA>
#> 330  MTA NYCT    SIM22         NA           NA         <NA>          <NA>
#> 331  MTA NYCT    SIM15         NA           NA         <NA>          <NA>
#> 332  MTA NYCT      S40         NA           NA         <NA>          <NA>
#> 333  MTA NYCT      S90         NA           NA         <NA>          <NA>
#> 334  MTA NYCT     <NA>         NA           NA         <NA>           B67
#> 335  MTA NYCT     <NA>         NA           NA         <NA>           B67
#> 336  MTA NYCT     <NA>         NA           NA         <NA>           B16
#> 337  MTA NYCT     <NA>         NA           NA         <NA>           B16
#> 338  MTA NYCT     <NA>         NA           NA         <NA>          M79+
#> 339  MTA NYCT     <NA>         NA           NA         <NA>          M79+
#> 340  MTA NYCT     <NA>         NA           NA         <NA>           S48
#> 341  MTA NYCT     <NA>         NA           NA         <NA>           S48
#> 342  MTA NYCT     <NA>         NA           NA         <NA>           Q38
#> 343  MTA NYCT     <NA>         NA           NA         <NA>           Q38
#> 344  MTA NYCT     <NA>         NA           NA         <NA>           Q42
#> 345  MTA NYCT     <NA>         NA           NA         <NA>           Q42
#> 346  MTA NYCT     <NA>         NA           NA         <NA>           B63
#> 347  MTA NYCT     <NA>         NA           NA         <NA>           B63
#> 348  MTA NYCT     <NA>         NA           NA         <NA>           M15
#> 349  MTA NYCT     <NA>         NA           NA         <NA>           M15
#> 350  MTA NYCT     <NA>         NA           NA         <NA>           BX8
#> 351  MTA NYCT     <NA>         NA           NA         <NA>           BX8
#> 352  MTA NYCT     <NA>         NA           NA         <NA>           B69
#> 353  MTA NYCT     <NA>         NA           NA         <NA>           B69
#> 354  MTA NYCT     <NA>         NA           NA         <NA>           B82
#> 355  MTA NYCT     <NA>         NA           NA         <NA>           B82
#> 356  MTA NYCT      S54         NA           NA         <NA>          <NA>
#> 357  MTA NYCT      S74         NA           NA         <NA>          <NA>
#> 358  MTA NYCT      S78         NA           NA         <NA>          <NA>
#> 359  MTA NYCT     <NA>         NA           NA         <NA>           B65
#> 360  MTA NYCT     <NA>         NA           NA         <NA>           B65
#> 361  MTA NYCT    SIM33         NA           NA         <NA>          <NA>
#> 362  MTA NYCT    SIM34         NA           NA         <NA>          <NA>
#> 363  MTA NYCT      Q85         NA           NA         <NA>          <NA>
#> 364  MTA NYCT       Q4         NA           NA         <NA>          <NA>
#> 365  MTA NYCT      Q87         NA           NA         <NA>          <NA>
#> 366  MTA NYCT     QM63         NA           NA         <NA>          <NA>
#> 367  MTA NYCT      Q84         NA           NA         <NA>          <NA>
#> 368  MTA NYCT      Q89         NA           NA         <NA>          <NA>
#> 369  MTA NYCT       Q5         NA           NA         <NA>          <NA>
#> 370  MTA NYCT     <NA>         NA           NA         <NA>           S40
#> 371  MTA NYCT     <NA>         NA           NA         <NA>           S40
#> 372  MTA NYCT     <NA>         NA           NA         <NA>         SIM26
#> 373  MTA NYCT     <NA>         NA           NA         <NA>         SIM26
#> 374  MTA NYCT      M20         NA           NA         <NA>          <NA>
#> 375  MTA NYCT      M55         NA           NA         <NA>          <NA>
#> 376  MTA NYCT     <NA>         NA           NA         <NA>           M35
#> 377  MTA NYCT     <NA>         NA           NA         <NA>           M35
#> 378  MTA NYCT    SIM35         NA           NA         <NA>          <NA>
#> 379  MTA NYCT    SIM15         NA           NA         <NA>          <NA>
#> 380  MTA NYCT     SIM5         NA           NA         <NA>          <NA>
#> 381  MTA NYCT     SIM4         NA           NA         <NA>          <NA>
#> 382  MTA NYCT     SIM8         NA           NA         <NA>          <NA>
#> 383  MTA NYCT     <NA>         NA           NA         <NA>           B20
#> 384  MTA NYCT     <NA>         NA           NA         <NA>           B20
#> 385  MTA NYCT     <NA>         NA           NA         <NA>          BX39
#> 386  MTA NYCT     <NA>         NA           NA         <NA>          BX39
#> 387  MTA NYCT     <NA>         NA           NA         <NA>           M98
#> 388  MTA NYCT     <NA>         NA           NA         <NA>           M98
#> 389  MTA NYCT     <NA>         NA           NA         <NA>           B60
#> 390  MTA NYCT     <NA>         NA           NA         <NA>           B60
#> 391  MTA NYCT     M15+         NA           NA         <NA>          <NA>
#> 392  MTA NYCT      M15         NA           NA         <NA>          <NA>
#> 393  MTA NYCT      M55         NA           NA         <NA>          <NA>
#> 394  MTA NYCT      M20         NA           NA         <NA>          <NA>
#> 395  MTA NYCT     <NA>         NA           NA         <NA>           Q61
#> 396  MTA NYCT     <NA>         NA           NA         <NA>           Q61
#> 397  MTA NYCT      M15         NA           NA         <NA>          <NA>
#> 398  MTA NYCT     M15+         NA           NA         <NA>          <NA>
#> 399  MTA NYCT     <NA>         NA           NA         <NA>           B12
#> 400  MTA NYCT     <NA>         NA           NA         <NA>           B12
#> 401     MTABC     BXM2         NA           NA         <NA>          <NA>
#> 402     MTABC     BXM4         NA           NA         <NA>          <NA>
#> 403     MTABC    BXM11         NA           NA         <NA>          <NA>
#> 404     MTABC     BXM3         NA           NA         <NA>          <NA>
#> 405     MTABC     BXM4         NA           NA         <NA>          <NA>
#> 406     MTABC     BXM3         NA           NA         <NA>          <NA>
#> 407     MTABC     <NA>         NA           NA         <NA>           Q47
#> 408     MTABC     <NA>         NA           NA         <NA>           Q47
#> 409     MTABC     <NA>         NA           NA         <NA>           Q32
#> 410     MTABC     <NA>         NA           NA         <NA>           Q32
#> 411     MTABC     <NA>         NA           NA         <NA>           QM6
#> 412     MTABC     <NA>         NA           NA         <NA>           QM6
#> 413     MTABC     <NA>         NA           NA         <NA>           Q18
#> 414     MTABC     <NA>         NA           NA         <NA>           Q18
#> 415     MTABC     <NA>         NA           NA         <NA>           Q07
#> 416     MTABC     <NA>         NA           NA         <NA>           Q07
#> 417     MTABC     <NA>         NA           NA         <NA>         BXM11
#> 418     MTABC     <NA>         NA           NA         <NA>         BXM11
#> 419     MTABC     <NA>         NA           NA         <NA>           QM5
#> 420     MTABC     <NA>         NA           NA         <NA>           QM5
#> 421     MTABC     Q52+         NA           NA         <NA>          <NA>
#> 422     MTABC     Q53+         NA           NA         <NA>          <NA>
#> 423     MTABC     <NA>         NA           NA         <NA>           Q25
#> 424     MTABC     <NA>         NA           NA         <NA>           Q25
#> 425     MTABC     <NA>         NA           NA         <NA>           Q47
#> 426     MTABC     <NA>         NA           NA         <NA>           Q47
#> 427     MTABC     <NA>         NA           NA         <NA>           Q50
#> 428     MTABC     <NA>         NA           NA         <NA>           Q50
#> 429     MTABC      Q32         NA           NA         <NA>          <NA>
#> 430     MTABC      Q60         NA           NA         <NA>          <NA>
#>     trip_direction_id trip_start_time trip_start_date
#> 1                  NA            <NA>            <NA>
#> 2                  NA            <NA>            <NA>
#> 3                   0            <NA>            <NA>
#> 4                   1            <NA>            <NA>
#> 5                   1            <NA>            <NA>
#> 6                   0            <NA>            <NA>
#> 7                  NA            <NA>            <NA>
#> 8                  NA            <NA>            <NA>
#> 9                  NA            <NA>            <NA>
#> 10                  1            <NA>            <NA>
#> 11                  0            <NA>            <NA>
#> 12                  0            <NA>            <NA>
#> 13                  1            <NA>            <NA>
#> 14                  1            <NA>            <NA>
#> 15                  0            <NA>            <NA>
#> 16                 NA            <NA>            <NA>
#> 17                 NA            <NA>            <NA>
#> 18                 NA            <NA>            <NA>
#> 19                 NA            <NA>            <NA>
#> 20                 NA            <NA>            <NA>
#> 21                 NA            <NA>            <NA>
#> 22                 NA            <NA>            <NA>
#> 23                 NA            <NA>            <NA>
#> 24                 NA            <NA>            <NA>
#> 25                 NA            <NA>            <NA>
#> 26                 NA            <NA>            <NA>
#> 27                 NA            <NA>            <NA>
#> 28                 NA            <NA>            <NA>
#> 29                 NA            <NA>            <NA>
#> 30                 NA            <NA>            <NA>
#> 31                 NA            <NA>            <NA>
#> 32                 NA            <NA>            <NA>
#> 33                 NA            <NA>            <NA>
#> 34                 NA            <NA>            <NA>
#> 35                 NA            <NA>            <NA>
#> 36                 NA            <NA>            <NA>
#> 37                 NA            <NA>            <NA>
#> 38                 NA            <NA>            <NA>
#> 39                 NA            <NA>            <NA>
#> 40                 NA            <NA>            <NA>
#> 41                 NA            <NA>            <NA>
#> 42                 NA            <NA>            <NA>
#> 43                 NA            <NA>            <NA>
#> 44                 NA            <NA>            <NA>
#> 45                 NA            <NA>            <NA>
#> 46                 NA            <NA>            <NA>
#> 47                 NA            <NA>            <NA>
#> 48                 NA            <NA>            <NA>
#> 49                 NA            <NA>            <NA>
#> 50                 NA            <NA>            <NA>
#> 51                 NA            <NA>            <NA>
#> 52                 NA            <NA>            <NA>
#> 53                 NA            <NA>            <NA>
#> 54                 NA            <NA>            <NA>
#> 55                 NA            <NA>            <NA>
#> 56                 NA            <NA>            <NA>
#> 57                 NA            <NA>            <NA>
#> 58                 NA            <NA>            <NA>
#> 59                 NA            <NA>            <NA>
#> 60                 NA            <NA>            <NA>
#> 61                 NA            <NA>            <NA>
#> 62                 NA            <NA>            <NA>
#> 63                 NA            <NA>            <NA>
#> 64                 NA            <NA>            <NA>
#> 65                 NA            <NA>            <NA>
#> 66                 NA            <NA>            <NA>
#> 67                 NA            <NA>            <NA>
#> 68                 NA            <NA>            <NA>
#> 69                 NA            <NA>            <NA>
#> 70                 NA            <NA>            <NA>
#> 71                 NA            <NA>            <NA>
#> 72                 NA            <NA>            <NA>
#> 73                 NA            <NA>            <NA>
#> 74                 NA            <NA>            <NA>
#> 75                 NA            <NA>            <NA>
#> 76                 NA            <NA>            <NA>
#> 77                 NA            <NA>            <NA>
#> 78                 NA            <NA>            <NA>
#> 79                 NA            <NA>            <NA>
#> 80                 NA            <NA>            <NA>
#> 81                 NA            <NA>            <NA>
#> 82                 NA            <NA>            <NA>
#> 83                 NA            <NA>            <NA>
#> 84                 NA            <NA>            <NA>
#> 85                 NA            <NA>            <NA>
#> 86                 NA            <NA>            <NA>
#> 87                 NA            <NA>            <NA>
#> 88                 NA            <NA>            <NA>
#> 89                 NA            <NA>            <NA>
#> 90                 NA            <NA>            <NA>
#> 91                 NA            <NA>            <NA>
#> 92                 NA            <NA>            <NA>
#> 93                 NA            <NA>            <NA>
#> 94                 NA            <NA>            <NA>
#> 95                 NA            <NA>            <NA>
#> 96                 NA            <NA>            <NA>
#> 97                 NA            <NA>            <NA>
#> 98                 NA            <NA>            <NA>
#> 99                 NA            <NA>            <NA>
#> 100                NA            <NA>            <NA>
#> 101                NA            <NA>            <NA>
#> 102                NA            <NA>            <NA>
#> 103                NA            <NA>            <NA>
#> 104                NA            <NA>            <NA>
#> 105                NA            <NA>            <NA>
#> 106                NA            <NA>            <NA>
#> 107                NA            <NA>            <NA>
#> 108                NA            <NA>            <NA>
#> 109                NA            <NA>            <NA>
#> 110                NA            <NA>            <NA>
#> 111                NA            <NA>            <NA>
#> 112                NA            <NA>            <NA>
#> 113                NA            <NA>            <NA>
#> 114                NA            <NA>            <NA>
#> 115                NA            <NA>            <NA>
#> 116                NA            <NA>            <NA>
#> 117                NA            <NA>            <NA>
#> 118                NA            <NA>            <NA>
#> 119                NA            <NA>            <NA>
#> 120                NA            <NA>            <NA>
#> 121                NA            <NA>            <NA>
#> 122                NA            <NA>            <NA>
#> 123                NA            <NA>            <NA>
#> 124                NA            <NA>            <NA>
#> 125                NA            <NA>            <NA>
#> 126                NA            <NA>            <NA>
#> 127                NA            <NA>            <NA>
#> 128                NA            <NA>            <NA>
#> 129                NA            <NA>            <NA>
#> 130                NA            <NA>            <NA>
#> 131                NA            <NA>            <NA>
#> 132                NA            <NA>            <NA>
#> 133                NA            <NA>            <NA>
#> 134                NA            <NA>            <NA>
#> 135                NA            <NA>            <NA>
#> 136                 0            <NA>            <NA>
#> 137                 1            <NA>            <NA>
#> 138                NA            <NA>            <NA>
#> 139                NA            <NA>            <NA>
#> 140                NA            <NA>            <NA>
#> 141                NA            <NA>            <NA>
#> 142                NA            <NA>            <NA>
#> 143                NA            <NA>            <NA>
#> 144                NA            <NA>            <NA>
#> 145                 0            <NA>            <NA>
#> 146                 1            <NA>            <NA>
#> 147                 1            <NA>            <NA>
#> 148                 0            <NA>            <NA>
#> 149                 1            <NA>            <NA>
#> 150                 0            <NA>            <NA>
#> 151                NA            <NA>            <NA>
#> 152                NA            <NA>            <NA>
#> 153                 1            <NA>            <NA>
#> 154                 0            <NA>            <NA>
#> 155                 0            <NA>            <NA>
#> 156                 1            <NA>            <NA>
#> 157                NA            <NA>            <NA>
#> 158                NA            <NA>            <NA>
#> 159                 0            <NA>            <NA>
#> 160                 1            <NA>            <NA>
#> 161                 1            <NA>            <NA>
#> 162                 0            <NA>            <NA>
#> 163                NA            <NA>            <NA>
#> 164                NA            <NA>            <NA>
#> 165                 1            <NA>            <NA>
#> 166                 0            <NA>            <NA>
#> 167                NA            <NA>            <NA>
#> 168                NA            <NA>            <NA>
#> 169                NA            <NA>            <NA>
#> 170                NA            <NA>            <NA>
#> 171                NA            <NA>            <NA>
#> 172                NA            <NA>            <NA>
#> 173                NA            <NA>            <NA>
#> 174                NA            <NA>            <NA>
#> 175                NA            <NA>            <NA>
#> 176                NA            <NA>            <NA>
#> 177                NA            <NA>            <NA>
#> 178                NA            <NA>            <NA>
#> 179                NA            <NA>            <NA>
#> 180                NA            <NA>            <NA>
#> 181                NA            <NA>            <NA>
#> 182                NA            <NA>            <NA>
#> 183                NA            <NA>            <NA>
#> 184                NA            <NA>            <NA>
#> 185                NA            <NA>            <NA>
#> 186                NA            <NA>            <NA>
#> 187                NA            <NA>            <NA>
#> 188                NA            <NA>            <NA>
#> 189                NA            <NA>            <NA>
#> 190                NA            <NA>            <NA>
#> 191                NA            <NA>            <NA>
#> 192                NA            <NA>            <NA>
#> 193                NA            <NA>            <NA>
#> 194                NA            <NA>            <NA>
#> 195                NA            <NA>            <NA>
#> 196                NA            <NA>            <NA>
#> 197                NA            <NA>            <NA>
#> 198                NA            <NA>            <NA>
#> 199                NA            <NA>            <NA>
#> 200                NA            <NA>            <NA>
#> 201                NA            <NA>            <NA>
#> 202                NA            <NA>            <NA>
#> 203                NA            <NA>            <NA>
#> 204                NA            <NA>            <NA>
#> 205                NA            <NA>            <NA>
#> 206                NA            <NA>            <NA>
#> 207                NA            <NA>            <NA>
#> 208                NA            <NA>            <NA>
#> 209                NA            <NA>            <NA>
#> 210                NA            <NA>            <NA>
#> 211                NA            <NA>            <NA>
#> 212                NA            <NA>            <NA>
#> 213                NA            <NA>            <NA>
#> 214                NA            <NA>            <NA>
#> 215                NA            <NA>            <NA>
#> 216                NA            <NA>            <NA>
#> 217                NA            <NA>            <NA>
#> 218                NA            <NA>            <NA>
#> 219                NA            <NA>            <NA>
#> 220                NA            <NA>            <NA>
#> 221                NA            <NA>            <NA>
#> 222                NA            <NA>            <NA>
#> 223                NA            <NA>            <NA>
#> 224                NA            <NA>            <NA>
#> 225                NA            <NA>            <NA>
#> 226                NA            <NA>            <NA>
#> 227                NA            <NA>            <NA>
#> 228                NA            <NA>            <NA>
#> 229                NA            <NA>            <NA>
#> 230                NA            <NA>            <NA>
#> 231                NA            <NA>            <NA>
#> 232                NA            <NA>            <NA>
#> 233                NA            <NA>            <NA>
#> 234                NA            <NA>            <NA>
#> 235                NA            <NA>            <NA>
#> 236                NA            <NA>            <NA>
#> 237                NA            <NA>            <NA>
#> 238                NA            <NA>            <NA>
#> 239                NA            <NA>            <NA>
#> 240                NA            <NA>            <NA>
#> 241                NA            <NA>            <NA>
#> 242                NA            <NA>            <NA>
#> 243                NA            <NA>            <NA>
#> 244                NA            <NA>            <NA>
#> 245                NA            <NA>            <NA>
#> 246                NA            <NA>            <NA>
#> 247                NA            <NA>            <NA>
#> 248                NA            <NA>            <NA>
#> 249                NA            <NA>            <NA>
#> 250                NA            <NA>            <NA>
#> 251                NA            <NA>            <NA>
#> 252                NA            <NA>            <NA>
#> 253                NA            <NA>            <NA>
#> 254                NA            <NA>            <NA>
#> 255                NA            <NA>            <NA>
#> 256                NA            <NA>            <NA>
#> 257                NA            <NA>            <NA>
#> 258                NA            <NA>            <NA>
#> 259                NA            <NA>            <NA>
#> 260                NA            <NA>            <NA>
#> 261                NA            <NA>            <NA>
#> 262                NA            <NA>            <NA>
#> 263                NA            <NA>            <NA>
#> 264                NA            <NA>            <NA>
#> 265                NA            <NA>            <NA>
#> 266                NA            <NA>            <NA>
#> 267                NA            <NA>            <NA>
#> 268                NA            <NA>            <NA>
#> 269                NA            <NA>            <NA>
#> 270                NA            <NA>            <NA>
#> 271                NA            <NA>            <NA>
#> 272                NA            <NA>            <NA>
#> 273                NA            <NA>            <NA>
#> 274                NA            <NA>            <NA>
#> 275                NA            <NA>            <NA>
#> 276                NA            <NA>            <NA>
#> 277                NA            <NA>            <NA>
#> 278                NA            <NA>            <NA>
#> 279                NA            <NA>            <NA>
#> 280                NA            <NA>            <NA>
#> 281                NA            <NA>            <NA>
#> 282                NA            <NA>            <NA>
#> 283                NA            <NA>            <NA>
#> 284                NA            <NA>            <NA>
#> 285                NA            <NA>            <NA>
#> 286                NA            <NA>            <NA>
#> 287                NA            <NA>            <NA>
#> 288                NA            <NA>            <NA>
#> 289                NA            <NA>            <NA>
#> 290                NA            <NA>            <NA>
#> 291                NA            <NA>            <NA>
#> 292                NA            <NA>            <NA>
#> 293                NA            <NA>            <NA>
#> 294                NA            <NA>            <NA>
#> 295                NA            <NA>            <NA>
#> 296                NA            <NA>            <NA>
#> 297                NA            <NA>            <NA>
#> 298                NA            <NA>            <NA>
#> 299                NA            <NA>            <NA>
#> 300                NA            <NA>            <NA>
#> 301                NA            <NA>            <NA>
#> 302                NA            <NA>            <NA>
#> 303                NA            <NA>            <NA>
#> 304                NA            <NA>            <NA>
#> 305                NA            <NA>            <NA>
#> 306                NA            <NA>            <NA>
#> 307                NA            <NA>            <NA>
#> 308                NA            <NA>            <NA>
#> 309                NA            <NA>            <NA>
#> 310                NA            <NA>            <NA>
#> 311                NA            <NA>            <NA>
#> 312                NA            <NA>            <NA>
#> 313                NA            <NA>            <NA>
#> 314                NA            <NA>            <NA>
#> 315                NA            <NA>            <NA>
#> 316                NA            <NA>            <NA>
#> 317                NA            <NA>            <NA>
#> 318                NA            <NA>            <NA>
#> 319                NA            <NA>            <NA>
#> 320                NA            <NA>            <NA>
#> 321                NA            <NA>            <NA>
#> 322                NA            <NA>            <NA>
#> 323                NA            <NA>            <NA>
#> 324                NA            <NA>            <NA>
#> 325                NA            <NA>            <NA>
#> 326                NA            <NA>            <NA>
#> 327                NA            <NA>            <NA>
#> 328                NA            <NA>            <NA>
#> 329                NA            <NA>            <NA>
#> 330                NA            <NA>            <NA>
#> 331                NA            <NA>            <NA>
#> 332                NA            <NA>            <NA>
#> 333                NA            <NA>            <NA>
#> 334                 1            <NA>            <NA>
#> 335                 0            <NA>            <NA>
#> 336                 0            <NA>            <NA>
#> 337                 1            <NA>            <NA>
#> 338                 1            <NA>            <NA>
#> 339                 0            <NA>            <NA>
#> 340                 0            <NA>            <NA>
#> 341                 1            <NA>            <NA>
#> 342                 0            <NA>            <NA>
#> 343                 1            <NA>            <NA>
#> 344                 1            <NA>            <NA>
#> 345                 0            <NA>            <NA>
#> 346                 0            <NA>            <NA>
#> 347                 1            <NA>            <NA>
#> 348                 0            <NA>            <NA>
#> 349                 1            <NA>            <NA>
#> 350                 0            <NA>            <NA>
#> 351                 1            <NA>            <NA>
#> 352                 0            <NA>            <NA>
#> 353                 1            <NA>            <NA>
#> 354                 1            <NA>            <NA>
#> 355                 0            <NA>            <NA>
#> 356                NA            <NA>            <NA>
#> 357                NA            <NA>            <NA>
#> 358                NA            <NA>            <NA>
#> 359                 1            <NA>            <NA>
#> 360                 0            <NA>            <NA>
#> 361                NA            <NA>            <NA>
#> 362                NA            <NA>            <NA>
#> 363                NA            <NA>            <NA>
#> 364                NA            <NA>            <NA>
#> 365                NA            <NA>            <NA>
#> 366                NA            <NA>            <NA>
#> 367                NA            <NA>            <NA>
#> 368                NA            <NA>            <NA>
#> 369                NA            <NA>            <NA>
#> 370                 0            <NA>            <NA>
#> 371                 1            <NA>            <NA>
#> 372                 0            <NA>            <NA>
#> 373                 1            <NA>            <NA>
#> 374                NA            <NA>            <NA>
#> 375                NA            <NA>            <NA>
#> 376                 1            <NA>            <NA>
#> 377                 0            <NA>            <NA>
#> 378                NA            <NA>            <NA>
#> 379                NA            <NA>            <NA>
#> 380                NA            <NA>            <NA>
#> 381                NA            <NA>            <NA>
#> 382                NA            <NA>            <NA>
#> 383                 0            <NA>            <NA>
#> 384                 1            <NA>            <NA>
#> 385                 1            <NA>            <NA>
#> 386                 0            <NA>            <NA>
#> 387                 0            <NA>            <NA>
#> 388                 1            <NA>            <NA>
#> 389                 1            <NA>            <NA>
#> 390                 0            <NA>            <NA>
#> 391                NA            <NA>            <NA>
#> 392                NA            <NA>            <NA>
#> 393                NA            <NA>            <NA>
#> 394                NA            <NA>            <NA>
#> 395                 0            <NA>            <NA>
#> 396                 1            <NA>            <NA>
#> 397                NA            <NA>            <NA>
#> 398                NA            <NA>            <NA>
#> 399                 1            <NA>            <NA>
#> 400                 0            <NA>            <NA>
#> 401                NA            <NA>            <NA>
#> 402                NA            <NA>            <NA>
#> 403                NA            <NA>            <NA>
#> 404                NA            <NA>            <NA>
#> 405                NA            <NA>            <NA>
#> 406                NA            <NA>            <NA>
#> 407                 0            <NA>            <NA>
#> 408                 1            <NA>            <NA>
#> 409                 0            <NA>            <NA>
#> 410                 1            <NA>            <NA>
#> 411                 1            <NA>            <NA>
#> 412                 0            <NA>            <NA>
#> 413                 0            <NA>            <NA>
#> 414                 1            <NA>            <NA>
#> 415                 1            <NA>            <NA>
#> 416                 0            <NA>            <NA>
#> 417                 0            <NA>            <NA>
#> 418                 1            <NA>            <NA>
#> 419                 0            <NA>            <NA>
#> 420                 1            <NA>            <NA>
#> 421                NA            <NA>            <NA>
#> 422                NA            <NA>            <NA>
#> 423                 1            <NA>            <NA>
#> 424                 0            <NA>            <NA>
#> 425                 1            <NA>            <NA>
#> 426                 0            <NA>            <NA>
#> 427                 1            <NA>            <NA>
#> 428                 0            <NA>            <NA>
#> 429                NA            <NA>            <NA>
#> 430                NA            <NA>            <NA>
#>     trip_schedule_relationship trip_modification_id stop_id cause effect
#> 1                         <NA>                 <NA>    <NA>  <NA>   <NA>
#> 2                         <NA>                 <NA>    <NA>  <NA>   <NA>
#> 3                         <NA>                 <NA>    <NA>  <NA>   <NA>
#> 4                         <NA>                 <NA>    <NA>  <NA>   <NA>
#> 5                         <NA>                 <NA>    <NA>  <NA>   <NA>
#> 6                         <NA>                 <NA>    <NA>  <NA>   <NA>
#> 7                         <NA>                 <NA>    <NA>  <NA>   <NA>
#> 8                         <NA>                 <NA>    <NA>  <NA>   <NA>
#> 9                         <NA>                 <NA>    <NA>  <NA>   <NA>
#> 10                        <NA>                 <NA>    <NA>  <NA>   <NA>
#> 11                        <NA>                 <NA>    <NA>  <NA>   <NA>
#> 12                        <NA>                 <NA>    <NA>  <NA>   <NA>
#> 13                        <NA>                 <NA>    <NA>  <NA>   <NA>
#> 14                        <NA>                 <NA>    <NA>  <NA>   <NA>
#> 15                        <NA>                 <NA>    <NA>  <NA>   <NA>
#> 16                        <NA>                 <NA>    <NA>  <NA>   <NA>
#> 17                        <NA>                 <NA>    <NA>  <NA>   <NA>
#> 18                        <NA>                 <NA>    <NA>  <NA>   <NA>
#> 19                        <NA>                 <NA>    <NA>  <NA>   <NA>
#> 20                        <NA>                 <NA>    <NA>  <NA>   <NA>
#> 21                        <NA>                 <NA>    <NA>  <NA>   <NA>
#> 22                        <NA>                 <NA>    <NA>  <NA>   <NA>
#> 23                        <NA>                 <NA>    <NA>  <NA>   <NA>
#> 24                        <NA>                 <NA>    <NA>  <NA>   <NA>
#> 25                        <NA>                 <NA>    <NA>  <NA>   <NA>
#> 26                        <NA>                 <NA>    <NA>  <NA>   <NA>
#> 27                        <NA>                 <NA>    <NA>  <NA>   <NA>
#> 28                        <NA>                 <NA>    <NA>  <NA>   <NA>
#> 29                        <NA>                 <NA>    <NA>  <NA>   <NA>
#> 30                        <NA>                 <NA>    <NA>  <NA>   <NA>
#> 31                        <NA>                 <NA>    <NA>  <NA>   <NA>
#> 32                        <NA>                 <NA>    <NA>  <NA>   <NA>
#> 33                        <NA>                 <NA>    <NA>  <NA>   <NA>
#> 34                        <NA>                 <NA>    <NA>  <NA>   <NA>
#> 35                        <NA>                 <NA>    <NA>  <NA>   <NA>
#> 36                        <NA>                 <NA>    <NA>  <NA>   <NA>
#> 37                        <NA>                 <NA>    <NA>  <NA>   <NA>
#> 38                        <NA>                 <NA>    <NA>  <NA>   <NA>
#> 39                        <NA>                 <NA>    <NA>  <NA>   <NA>
#> 40                        <NA>                 <NA>    <NA>  <NA>   <NA>
#> 41                        <NA>                 <NA>    <NA>  <NA>   <NA>
#> 42                        <NA>                 <NA>    <NA>  <NA>   <NA>
#> 43                        <NA>                 <NA>    <NA>  <NA>   <NA>
#> 44                        <NA>                 <NA>    <NA>  <NA>   <NA>
#> 45                        <NA>                 <NA>    <NA>  <NA>   <NA>
#> 46                        <NA>                 <NA>    <NA>  <NA>   <NA>
#> 47                        <NA>                 <NA>    <NA>  <NA>   <NA>
#> 48                        <NA>                 <NA>    <NA>  <NA>   <NA>
#> 49                        <NA>                 <NA>    <NA>  <NA>   <NA>
#> 50                        <NA>                 <NA>    <NA>  <NA>   <NA>
#> 51                        <NA>                 <NA>    <NA>  <NA>   <NA>
#> 52                        <NA>                 <NA>    <NA>  <NA>   <NA>
#> 53                        <NA>                 <NA>    <NA>  <NA>   <NA>
#> 54                        <NA>                 <NA>    <NA>  <NA>   <NA>
#> 55                        <NA>                 <NA>    <NA>  <NA>   <NA>
#> 56                        <NA>                 <NA>    <NA>  <NA>   <NA>
#> 57                        <NA>                 <NA>    <NA>  <NA>   <NA>
#> 58                        <NA>                 <NA>    <NA>  <NA>   <NA>
#> 59                        <NA>                 <NA>    <NA>  <NA>   <NA>
#> 60                        <NA>                 <NA>    <NA>  <NA>   <NA>
#> 61                        <NA>                 <NA>    <NA>  <NA>   <NA>
#> 62                        <NA>                 <NA>    <NA>  <NA>   <NA>
#> 63                        <NA>                 <NA>    <NA>  <NA>   <NA>
#> 64                        <NA>                 <NA>    <NA>  <NA>   <NA>
#> 65                        <NA>                 <NA>    <NA>  <NA>   <NA>
#> 66                        <NA>                 <NA>    <NA>  <NA>   <NA>
#> 67                        <NA>                 <NA>    <NA>  <NA>   <NA>
#> 68                        <NA>                 <NA>    <NA>  <NA>   <NA>
#> 69                        <NA>                 <NA>    <NA>  <NA>   <NA>
#> 70                        <NA>                 <NA>    <NA>  <NA>   <NA>
#> 71                        <NA>                 <NA>    <NA>  <NA>   <NA>
#> 72                        <NA>                 <NA>    <NA>  <NA>   <NA>
#> 73                        <NA>                 <NA>    <NA>  <NA>   <NA>
#> 74                        <NA>                 <NA>    <NA>  <NA>   <NA>
#> 75                        <NA>                 <NA>    <NA>  <NA>   <NA>
#> 76                        <NA>                 <NA>    <NA>  <NA>   <NA>
#> 77                        <NA>                 <NA>    <NA>  <NA>   <NA>
#> 78                        <NA>                 <NA>    <NA>  <NA>   <NA>
#> 79                        <NA>                 <NA>    <NA>  <NA>   <NA>
#> 80                        <NA>                 <NA>    <NA>  <NA>   <NA>
#> 81                        <NA>                 <NA>    <NA>  <NA>   <NA>
#> 82                        <NA>                 <NA>    <NA>  <NA>   <NA>
#> 83                        <NA>                 <NA>    <NA>  <NA>   <NA>
#> 84                        <NA>                 <NA>    <NA>  <NA>   <NA>
#> 85                        <NA>                 <NA>    <NA>  <NA>   <NA>
#> 86                        <NA>                 <NA>    <NA>  <NA>   <NA>
#> 87                        <NA>                 <NA>    <NA>  <NA>   <NA>
#> 88                        <NA>                 <NA>    <NA>  <NA>   <NA>
#> 89                        <NA>                 <NA>    <NA>  <NA>   <NA>
#> 90                        <NA>                 <NA>    <NA>  <NA>   <NA>
#> 91                        <NA>                 <NA>    <NA>  <NA>   <NA>
#> 92                        <NA>                 <NA>    <NA>  <NA>   <NA>
#> 93                        <NA>                 <NA>    <NA>  <NA>   <NA>
#> 94                        <NA>                 <NA>    <NA>  <NA>   <NA>
#> 95                        <NA>                 <NA>    <NA>  <NA>   <NA>
#> 96                        <NA>                 <NA>    <NA>  <NA>   <NA>
#> 97                        <NA>                 <NA>    <NA>  <NA>   <NA>
#> 98                        <NA>                 <NA>    <NA>  <NA>   <NA>
#> 99                        <NA>                 <NA>    <NA>  <NA>   <NA>
#> 100                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 101                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 102                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 103                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 104                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 105                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 106                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 107                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 108                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 109                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 110                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 111                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 112                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 113                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 114                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 115                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 116                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 117                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 118                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 119                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 120                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 121                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 122                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 123                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 124                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 125                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 126                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 127                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 128                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 129                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 130                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 131                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 132                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 133                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 134                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 135                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 136                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 137                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 138                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 139                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 140                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 141                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 142                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 143                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 144                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 145                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 146                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 147                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 148                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 149                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 150                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 151                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 152                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 153                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 154                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 155                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 156                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 157                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 158                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 159                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 160                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 161                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 162                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 163                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 164                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 165                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 166                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 167                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 168                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 169                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 170                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 171                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 172                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 173                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 174                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 175                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 176                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 177                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 178                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 179                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 180                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 181                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 182                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 183                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 184                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 185                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 186                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 187                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 188                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 189                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 190                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 191                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 192                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 193                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 194                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 195                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 196                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 197                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 198                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 199                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 200                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 201                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 202                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 203                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 204                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 205                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 206                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 207                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 208                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 209                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 210                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 211                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 212                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 213                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 214                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 215                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 216                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 217                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 218                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 219                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 220                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 221                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 222                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 223                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 224                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 225                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 226                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 227                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 228                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 229                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 230                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 231                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 232                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 233                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 234                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 235                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 236                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 237                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 238                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 239                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 240                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 241                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 242                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 243                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 244                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 245                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 246                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 247                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 248                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 249                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 250                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 251                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 252                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 253                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 254                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 255                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 256                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 257                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 258                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 259                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 260                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 261                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 262                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 263                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 264                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 265                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 266                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 267                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 268                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 269                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 270                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 271                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 272                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 273                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 274                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 275                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 276                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 277                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 278                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 279                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 280                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 281                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 282                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 283                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 284                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 285                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 286                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 287                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 288                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 289                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 290                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 291                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 292                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 293                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 294                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 295                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 296                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 297                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 298                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 299                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 300                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 301                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 302                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 303                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 304                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 305                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 306                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 307                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 308                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 309                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 310                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 311                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 312                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 313                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 314                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 315                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 316                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 317                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 318                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 319                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 320                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 321                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 322                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 323                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 324                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 325                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 326                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 327                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 328                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 329                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 330                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 331                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 332                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 333                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 334                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 335                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 336                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 337                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 338                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 339                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 340                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 341                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 342                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 343                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 344                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 345                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 346                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 347                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 348                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 349                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 350                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 351                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 352                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 353                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 354                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 355                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 356                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 357                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 358                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 359                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 360                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 361                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 362                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 363                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 364                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 365                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 366                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 367                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 368                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 369                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 370                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 371                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 372                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 373                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 374                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 375                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 376                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 377                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 378                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 379                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 380                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 381                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 382                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 383                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 384                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 385                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 386                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 387                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 388                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 389                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 390                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 391                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 392                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 393                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 394                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 395                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 396                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 397                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 398                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 399                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 400                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 401                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 402                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 403                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 404                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 405                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 406                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 407                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 408                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 409                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 410                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 411                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 412                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 413                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 414                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 415                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 416                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 417                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 418                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 419                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 420                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 421                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 422                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 423                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 424                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 425                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 426                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 427                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 428                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 429                       <NA>                 <NA>    <NA>  <NA>   <NA>
#> 430                       <NA>                 <NA>    <NA>  <NA>   <NA>
#>     language cause_detail effect_detail  url
#> 1         EN         <NA>          <NA> <NA>
#> 2         EN         <NA>          <NA> <NA>
#> 3         EN         <NA>          <NA> <NA>
#> 4         EN         <NA>          <NA> <NA>
#> 5         EN         <NA>          <NA> <NA>
#> 6         EN         <NA>          <NA> <NA>
#> 7         EN         <NA>          <NA> <NA>
#> 8         EN         <NA>          <NA> <NA>
#> 9         EN         <NA>          <NA> <NA>
#> 10        EN         <NA>          <NA> <NA>
#> 11        EN         <NA>          <NA> <NA>
#> 12        EN         <NA>          <NA> <NA>
#> 13        EN         <NA>          <NA> <NA>
#> 14        EN         <NA>          <NA> <NA>
#> 15        EN         <NA>          <NA> <NA>
#> 16        EN         <NA>          <NA> <NA>
#> 17        EN         <NA>          <NA> <NA>
#> 18        EN         <NA>          <NA> <NA>
#> 19        EN         <NA>          <NA> <NA>
#> 20        EN         <NA>          <NA> <NA>
#> 21        EN         <NA>          <NA> <NA>
#> 22        EN         <NA>          <NA> <NA>
#> 23        EN         <NA>          <NA> <NA>
#> 24        EN         <NA>          <NA> <NA>
#> 25        EN         <NA>          <NA> <NA>
#> 26        EN         <NA>          <NA> <NA>
#> 27        EN         <NA>          <NA> <NA>
#> 28        EN         <NA>          <NA> <NA>
#> 29        EN         <NA>          <NA> <NA>
#> 30        EN         <NA>          <NA> <NA>
#> 31        EN         <NA>          <NA> <NA>
#> 32        EN         <NA>          <NA> <NA>
#> 33        EN         <NA>          <NA> <NA>
#> 34        EN         <NA>          <NA> <NA>
#> 35        EN         <NA>          <NA> <NA>
#> 36        EN         <NA>          <NA> <NA>
#> 37        EN         <NA>          <NA> <NA>
#> 38        EN         <NA>          <NA> <NA>
#> 39        EN         <NA>          <NA> <NA>
#> 40        EN         <NA>          <NA> <NA>
#> 41        EN         <NA>          <NA> <NA>
#> 42        EN         <NA>          <NA> <NA>
#> 43        EN         <NA>          <NA> <NA>
#> 44        EN         <NA>          <NA> <NA>
#> 45        EN         <NA>          <NA> <NA>
#> 46        EN         <NA>          <NA> <NA>
#> 47        EN         <NA>          <NA> <NA>
#> 48        EN         <NA>          <NA> <NA>
#> 49        EN         <NA>          <NA> <NA>
#> 50        EN         <NA>          <NA> <NA>
#> 51        EN         <NA>          <NA> <NA>
#> 52        EN         <NA>          <NA> <NA>
#> 53        EN         <NA>          <NA> <NA>
#> 54        EN         <NA>          <NA> <NA>
#> 55        EN         <NA>          <NA> <NA>
#> 56        EN         <NA>          <NA> <NA>
#> 57        EN         <NA>          <NA> <NA>
#> 58        EN         <NA>          <NA> <NA>
#> 59        EN         <NA>          <NA> <NA>
#> 60        EN         <NA>          <NA> <NA>
#> 61        EN         <NA>          <NA> <NA>
#> 62        EN         <NA>          <NA> <NA>
#> 63        EN         <NA>          <NA> <NA>
#> 64        EN         <NA>          <NA> <NA>
#> 65        EN         <NA>          <NA> <NA>
#> 66        EN         <NA>          <NA> <NA>
#> 67        EN         <NA>          <NA> <NA>
#> 68        EN         <NA>          <NA> <NA>
#> 69        EN         <NA>          <NA> <NA>
#> 70        EN         <NA>          <NA> <NA>
#> 71        EN         <NA>          <NA> <NA>
#> 72        EN         <NA>          <NA> <NA>
#> 73        EN         <NA>          <NA> <NA>
#> 74        EN         <NA>          <NA> <NA>
#> 75        EN         <NA>          <NA> <NA>
#> 76        EN         <NA>          <NA> <NA>
#> 77        EN         <NA>          <NA> <NA>
#> 78        EN         <NA>          <NA> <NA>
#> 79        EN         <NA>          <NA> <NA>
#> 80        EN         <NA>          <NA> <NA>
#> 81        EN         <NA>          <NA> <NA>
#> 82        EN         <NA>          <NA> <NA>
#> 83        EN         <NA>          <NA> <NA>
#> 84        EN         <NA>          <NA> <NA>
#> 85        EN         <NA>          <NA> <NA>
#> 86        EN         <NA>          <NA> <NA>
#> 87        EN         <NA>          <NA> <NA>
#> 88        EN         <NA>          <NA> <NA>
#> 89        EN         <NA>          <NA> <NA>
#> 90        EN         <NA>          <NA> <NA>
#> 91        EN         <NA>          <NA> <NA>
#> 92        EN         <NA>          <NA> <NA>
#> 93        EN         <NA>          <NA> <NA>
#> 94        EN         <NA>          <NA> <NA>
#> 95        EN         <NA>          <NA> <NA>
#> 96        EN         <NA>          <NA> <NA>
#> 97        EN         <NA>          <NA> <NA>
#> 98        EN         <NA>          <NA> <NA>
#> 99        EN         <NA>          <NA> <NA>
#> 100       EN         <NA>          <NA> <NA>
#> 101       EN         <NA>          <NA> <NA>
#> 102       EN         <NA>          <NA> <NA>
#> 103       EN         <NA>          <NA> <NA>
#> 104       EN         <NA>          <NA> <NA>
#> 105       EN         <NA>          <NA> <NA>
#> 106       EN         <NA>          <NA> <NA>
#> 107       EN         <NA>          <NA> <NA>
#> 108       EN         <NA>          <NA> <NA>
#> 109       EN         <NA>          <NA> <NA>
#> 110       EN         <NA>          <NA> <NA>
#> 111       EN         <NA>          <NA> <NA>
#> 112       EN         <NA>          <NA> <NA>
#> 113       EN         <NA>          <NA> <NA>
#> 114       EN         <NA>          <NA> <NA>
#> 115       EN         <NA>          <NA> <NA>
#> 116       EN         <NA>          <NA> <NA>
#> 117       EN         <NA>          <NA> <NA>
#> 118       EN         <NA>          <NA> <NA>
#> 119       EN         <NA>          <NA> <NA>
#> 120       EN         <NA>          <NA> <NA>
#> 121       EN         <NA>          <NA> <NA>
#> 122       EN         <NA>          <NA> <NA>
#> 123       EN         <NA>          <NA> <NA>
#> 124       EN         <NA>          <NA> <NA>
#> 125       EN         <NA>          <NA> <NA>
#> 126       EN         <NA>          <NA> <NA>
#> 127       EN         <NA>          <NA> <NA>
#> 128       EN         <NA>          <NA> <NA>
#> 129       EN         <NA>          <NA> <NA>
#> 130       EN         <NA>          <NA> <NA>
#> 131       EN         <NA>          <NA> <NA>
#> 132       EN         <NA>          <NA> <NA>
#> 133       EN         <NA>          <NA> <NA>
#> 134       EN         <NA>          <NA> <NA>
#> 135       EN         <NA>          <NA> <NA>
#> 136       EN         <NA>          <NA> <NA>
#> 137       EN         <NA>          <NA> <NA>
#> 138       EN         <NA>          <NA> <NA>
#> 139       EN         <NA>          <NA> <NA>
#> 140       EN         <NA>          <NA> <NA>
#> 141       EN         <NA>          <NA> <NA>
#> 142       EN         <NA>          <NA> <NA>
#> 143       EN         <NA>          <NA> <NA>
#> 144       EN         <NA>          <NA> <NA>
#> 145       EN         <NA>          <NA> <NA>
#> 146       EN         <NA>          <NA> <NA>
#> 147       EN         <NA>          <NA> <NA>
#> 148       EN         <NA>          <NA> <NA>
#> 149       EN         <NA>          <NA> <NA>
#> 150       EN         <NA>          <NA> <NA>
#> 151       EN         <NA>          <NA> <NA>
#> 152       EN         <NA>          <NA> <NA>
#> 153       EN         <NA>          <NA> <NA>
#> 154       EN         <NA>          <NA> <NA>
#> 155       EN         <NA>          <NA> <NA>
#> 156       EN         <NA>          <NA> <NA>
#> 157       EN         <NA>          <NA> <NA>
#> 158       EN         <NA>          <NA> <NA>
#> 159       EN         <NA>          <NA> <NA>
#> 160       EN         <NA>          <NA> <NA>
#> 161       EN         <NA>          <NA> <NA>
#> 162       EN         <NA>          <NA> <NA>
#> 163       EN         <NA>          <NA> <NA>
#> 164       EN         <NA>          <NA> <NA>
#> 165       EN         <NA>          <NA> <NA>
#> 166       EN         <NA>          <NA> <NA>
#> 167       EN         <NA>          <NA> <NA>
#> 168       EN         <NA>          <NA> <NA>
#> 169       EN         <NA>          <NA> <NA>
#> 170       EN         <NA>          <NA> <NA>
#> 171       EN         <NA>          <NA> <NA>
#> 172       EN         <NA>          <NA> <NA>
#> 173       EN         <NA>          <NA> <NA>
#> 174       EN         <NA>          <NA> <NA>
#> 175       EN         <NA>          <NA> <NA>
#> 176       EN         <NA>          <NA> <NA>
#> 177       EN         <NA>          <NA> <NA>
#> 178       EN         <NA>          <NA> <NA>
#> 179       EN         <NA>          <NA> <NA>
#> 180       EN         <NA>          <NA> <NA>
#> 181       EN         <NA>          <NA> <NA>
#> 182       EN         <NA>          <NA> <NA>
#> 183       EN         <NA>          <NA> <NA>
#> 184       EN         <NA>          <NA> <NA>
#> 185       EN         <NA>          <NA> <NA>
#> 186       EN         <NA>          <NA> <NA>
#> 187       EN         <NA>          <NA> <NA>
#> 188       EN         <NA>          <NA> <NA>
#> 189       EN         <NA>          <NA> <NA>
#> 190       EN         <NA>          <NA> <NA>
#> 191       EN         <NA>          <NA> <NA>
#> 192       EN         <NA>          <NA> <NA>
#> 193       EN         <NA>          <NA> <NA>
#> 194       EN         <NA>          <NA> <NA>
#> 195       EN         <NA>          <NA> <NA>
#> 196       EN         <NA>          <NA> <NA>
#> 197       EN         <NA>          <NA> <NA>
#> 198       EN         <NA>          <NA> <NA>
#> 199       EN         <NA>          <NA> <NA>
#> 200       EN         <NA>          <NA> <NA>
#> 201       EN         <NA>          <NA> <NA>
#> 202       EN         <NA>          <NA> <NA>
#> 203       EN         <NA>          <NA> <NA>
#> 204       EN         <NA>          <NA> <NA>
#> 205       EN         <NA>          <NA> <NA>
#> 206       EN         <NA>          <NA> <NA>
#> 207       EN         <NA>          <NA> <NA>
#> 208       EN         <NA>          <NA> <NA>
#> 209       EN         <NA>          <NA> <NA>
#> 210       EN         <NA>          <NA> <NA>
#> 211       EN         <NA>          <NA> <NA>
#> 212       EN         <NA>          <NA> <NA>
#> 213       EN         <NA>          <NA> <NA>
#> 214       EN         <NA>          <NA> <NA>
#> 215       EN         <NA>          <NA> <NA>
#> 216       EN         <NA>          <NA> <NA>
#> 217       EN         <NA>          <NA> <NA>
#> 218       EN         <NA>          <NA> <NA>
#> 219       EN         <NA>          <NA> <NA>
#> 220       EN         <NA>          <NA> <NA>
#> 221       EN         <NA>          <NA> <NA>
#> 222       EN         <NA>          <NA> <NA>
#> 223       EN         <NA>          <NA> <NA>
#> 224       EN         <NA>          <NA> <NA>
#> 225       EN         <NA>          <NA> <NA>
#> 226       EN         <NA>          <NA> <NA>
#> 227       EN         <NA>          <NA> <NA>
#> 228       EN         <NA>          <NA> <NA>
#> 229       EN         <NA>          <NA> <NA>
#> 230       EN         <NA>          <NA> <NA>
#> 231       EN         <NA>          <NA> <NA>
#> 232       EN         <NA>          <NA> <NA>
#> 233       EN         <NA>          <NA> <NA>
#> 234       EN         <NA>          <NA> <NA>
#> 235       EN         <NA>          <NA> <NA>
#> 236       EN         <NA>          <NA> <NA>
#> 237       EN         <NA>          <NA> <NA>
#> 238       EN         <NA>          <NA> <NA>
#> 239       EN         <NA>          <NA> <NA>
#> 240       EN         <NA>          <NA> <NA>
#> 241       EN         <NA>          <NA> <NA>
#> 242       EN         <NA>          <NA> <NA>
#> 243       EN         <NA>          <NA> <NA>
#> 244       EN         <NA>          <NA> <NA>
#> 245       EN         <NA>          <NA> <NA>
#> 246       EN         <NA>          <NA> <NA>
#> 247       EN         <NA>          <NA> <NA>
#> 248       EN         <NA>          <NA> <NA>
#> 249       EN         <NA>          <NA> <NA>
#> 250       EN         <NA>          <NA> <NA>
#> 251       EN         <NA>          <NA> <NA>
#> 252       EN         <NA>          <NA> <NA>
#> 253       EN         <NA>          <NA> <NA>
#> 254       EN         <NA>          <NA> <NA>
#> 255       EN         <NA>          <NA> <NA>
#> 256       EN         <NA>          <NA> <NA>
#> 257       EN         <NA>          <NA> <NA>
#> 258       EN         <NA>          <NA> <NA>
#> 259       EN         <NA>          <NA> <NA>
#> 260       EN         <NA>          <NA> <NA>
#> 261       EN         <NA>          <NA> <NA>
#> 262       EN         <NA>          <NA> <NA>
#> 263       EN         <NA>          <NA> <NA>
#> 264       EN         <NA>          <NA> <NA>
#> 265       EN         <NA>          <NA> <NA>
#> 266       EN         <NA>          <NA> <NA>
#> 267       EN         <NA>          <NA> <NA>
#> 268       EN         <NA>          <NA> <NA>
#> 269       EN         <NA>          <NA> <NA>
#> 270       EN         <NA>          <NA> <NA>
#> 271       EN         <NA>          <NA> <NA>
#> 272       EN         <NA>          <NA> <NA>
#> 273       EN         <NA>          <NA> <NA>
#> 274       EN         <NA>          <NA> <NA>
#> 275       EN         <NA>          <NA> <NA>
#> 276       EN         <NA>          <NA> <NA>
#> 277       EN         <NA>          <NA> <NA>
#> 278       EN         <NA>          <NA> <NA>
#> 279       EN         <NA>          <NA> <NA>
#> 280       EN         <NA>          <NA> <NA>
#> 281       EN         <NA>          <NA> <NA>
#> 282       EN         <NA>          <NA> <NA>
#> 283       EN         <NA>          <NA> <NA>
#> 284       EN         <NA>          <NA> <NA>
#> 285       EN         <NA>          <NA> <NA>
#> 286       EN         <NA>          <NA> <NA>
#> 287       EN         <NA>          <NA> <NA>
#> 288       EN         <NA>          <NA> <NA>
#> 289       EN         <NA>          <NA> <NA>
#> 290       EN         <NA>          <NA> <NA>
#> 291       EN         <NA>          <NA> <NA>
#> 292       EN         <NA>          <NA> <NA>
#> 293       EN         <NA>          <NA> <NA>
#> 294       EN         <NA>          <NA> <NA>
#> 295       EN         <NA>          <NA> <NA>
#> 296       EN         <NA>          <NA> <NA>
#> 297       EN         <NA>          <NA> <NA>
#> 298       EN         <NA>          <NA> <NA>
#> 299       EN         <NA>          <NA> <NA>
#> 300       EN         <NA>          <NA> <NA>
#> 301       EN         <NA>          <NA> <NA>
#> 302       EN         <NA>          <NA> <NA>
#> 303       EN         <NA>          <NA> <NA>
#> 304       EN         <NA>          <NA> <NA>
#> 305       EN         <NA>          <NA> <NA>
#> 306       EN         <NA>          <NA> <NA>
#> 307       EN         <NA>          <NA> <NA>
#> 308       EN         <NA>          <NA> <NA>
#> 309       EN         <NA>          <NA> <NA>
#> 310       EN         <NA>          <NA> <NA>
#> 311       EN         <NA>          <NA> <NA>
#> 312       EN         <NA>          <NA> <NA>
#> 313       EN         <NA>          <NA> <NA>
#> 314       EN         <NA>          <NA> <NA>
#> 315       EN         <NA>          <NA> <NA>
#> 316       EN         <NA>          <NA> <NA>
#> 317       EN         <NA>          <NA> <NA>
#> 318       EN         <NA>          <NA> <NA>
#> 319       EN         <NA>          <NA> <NA>
#> 320       EN         <NA>          <NA> <NA>
#> 321       EN         <NA>          <NA> <NA>
#> 322       EN         <NA>          <NA> <NA>
#> 323       EN         <NA>          <NA> <NA>
#> 324       EN         <NA>          <NA> <NA>
#> 325       EN         <NA>          <NA> <NA>
#> 326       EN         <NA>          <NA> <NA>
#> 327       EN         <NA>          <NA> <NA>
#> 328       EN         <NA>          <NA> <NA>
#> 329       EN         <NA>          <NA> <NA>
#> 330       EN         <NA>          <NA> <NA>
#> 331       EN         <NA>          <NA> <NA>
#> 332       EN         <NA>          <NA> <NA>
#> 333       EN         <NA>          <NA> <NA>
#> 334       EN         <NA>          <NA> <NA>
#> 335       EN         <NA>          <NA> <NA>
#> 336       EN         <NA>          <NA> <NA>
#> 337       EN         <NA>          <NA> <NA>
#> 338       EN         <NA>          <NA> <NA>
#> 339       EN         <NA>          <NA> <NA>
#> 340       EN         <NA>          <NA> <NA>
#> 341       EN         <NA>          <NA> <NA>
#> 342       EN         <NA>          <NA> <NA>
#> 343       EN         <NA>          <NA> <NA>
#> 344       EN         <NA>          <NA> <NA>
#> 345       EN         <NA>          <NA> <NA>
#> 346       EN         <NA>          <NA> <NA>
#> 347       EN         <NA>          <NA> <NA>
#> 348       EN         <NA>          <NA> <NA>
#> 349       EN         <NA>          <NA> <NA>
#> 350       EN         <NA>          <NA> <NA>
#> 351       EN         <NA>          <NA> <NA>
#> 352       EN         <NA>          <NA> <NA>
#> 353       EN         <NA>          <NA> <NA>
#> 354       EN         <NA>          <NA> <NA>
#> 355       EN         <NA>          <NA> <NA>
#> 356       EN         <NA>          <NA> <NA>
#> 357       EN         <NA>          <NA> <NA>
#> 358       EN         <NA>          <NA> <NA>
#> 359       EN         <NA>          <NA> <NA>
#> 360       EN         <NA>          <NA> <NA>
#> 361       EN         <NA>          <NA> <NA>
#> 362       EN         <NA>          <NA> <NA>
#> 363       EN         <NA>          <NA> <NA>
#> 364       EN         <NA>          <NA> <NA>
#> 365       EN         <NA>          <NA> <NA>
#> 366       EN         <NA>          <NA> <NA>
#> 367       EN         <NA>          <NA> <NA>
#> 368       EN         <NA>          <NA> <NA>
#> 369       EN         <NA>          <NA> <NA>
#> 370       EN         <NA>          <NA> <NA>
#> 371       EN         <NA>          <NA> <NA>
#> 372       EN         <NA>          <NA> <NA>
#> 373       EN         <NA>          <NA> <NA>
#> 374       EN         <NA>          <NA> <NA>
#> 375       EN         <NA>          <NA> <NA>
#> 376       EN         <NA>          <NA> <NA>
#> 377       EN         <NA>          <NA> <NA>
#> 378       EN         <NA>          <NA> <NA>
#> 379       EN         <NA>          <NA> <NA>
#> 380       EN         <NA>          <NA> <NA>
#> 381       EN         <NA>          <NA> <NA>
#> 382       EN         <NA>          <NA> <NA>
#> 383       EN         <NA>          <NA> <NA>
#> 384       EN         <NA>          <NA> <NA>
#> 385       EN         <NA>          <NA> <NA>
#> 386       EN         <NA>          <NA> <NA>
#> 387       EN         <NA>          <NA> <NA>
#> 388       EN         <NA>          <NA> <NA>
#> 389       EN         <NA>          <NA> <NA>
#> 390       EN         <NA>          <NA> <NA>
#> 391       EN         <NA>          <NA> <NA>
#> 392       EN         <NA>          <NA> <NA>
#> 393       EN         <NA>          <NA> <NA>
#> 394       EN         <NA>          <NA> <NA>
#> 395       EN         <NA>          <NA> <NA>
#> 396       EN         <NA>          <NA> <NA>
#> 397       EN         <NA>          <NA> <NA>
#> 398       EN         <NA>          <NA> <NA>
#> 399       EN         <NA>          <NA> <NA>
#> 400       EN         <NA>          <NA> <NA>
#> 401       EN         <NA>          <NA> <NA>
#> 402       EN         <NA>          <NA> <NA>
#> 403       EN         <NA>          <NA> <NA>
#> 404       EN         <NA>          <NA> <NA>
#> 405       EN         <NA>          <NA> <NA>
#> 406       EN         <NA>          <NA> <NA>
#> 407       EN         <NA>          <NA> <NA>
#> 408       EN         <NA>          <NA> <NA>
#> 409       EN         <NA>          <NA> <NA>
#> 410       EN         <NA>          <NA> <NA>
#> 411       EN         <NA>          <NA> <NA>
#> 412       EN         <NA>          <NA> <NA>
#> 413       EN         <NA>          <NA> <NA>
#> 414       EN         <NA>          <NA> <NA>
#> 415       EN         <NA>          <NA> <NA>
#> 416       EN         <NA>          <NA> <NA>
#> 417       EN         <NA>          <NA> <NA>
#> 418       EN         <NA>          <NA> <NA>
#> 419       EN         <NA>          <NA> <NA>
#> 420       EN         <NA>          <NA> <NA>
#> 421       EN         <NA>          <NA> <NA>
#> 422       EN         <NA>          <NA> <NA>
#> 423       EN         <NA>          <NA> <NA>
#> 424       EN         <NA>          <NA> <NA>
#> 425       EN         <NA>          <NA> <NA>
#> 426       EN         <NA>          <NA> <NA>
#> 427       EN         <NA>          <NA> <NA>
#> 428       EN         <NA>          <NA> <NA>
#> 429       EN         <NA>          <NA> <NA>
#> 430       EN         <NA>          <NA> <NA>
#>                                                                                                                                                          header_text
#> 1                                                                                         X28  and X38  stops on Surf Ave at W 21st St are closed in both directions
#> 2                                                                                         X28  and X38  stops on Surf Ave at W 21st St are closed in both directions
#> 3                               Northbound M15-SBS stop on Water St at Pine St is closed, buses make a temporary stop across the intersection on Water St at Pine St
#> 4                               Northbound M15-SBS stop on Water St at Pine St is closed, buses make a temporary stop across the intersection on Water St at Pine St
#> 5                                                                               S66 buses are detoured in both directions due to icy roads at Arlo Rd/Stratford Ave.
#> 6                                                                               S66 buses are detoured in both directions due to icy roads at Arlo Rd/Stratford Ave.
#> 7                                                                                                                 You may wait longer for these buses:\nB9, B35, B63
#> 8                                                                                                                 You may wait longer for these buses:\nB9, B35, B63
#> 9                                                                                                                 You may wait longer for these buses:\nB9, B35, B63
#> 10                                 Southbound Bx39 stop on White Plains Rd at E 215th St is closed; please use the new stop on White Plains Rd at E 216th St instead
#> 11                                 Southbound Bx39 stop on White Plains Rd at E 215th St is closed; please use the new stop on White Plains Rd at E 216th St instead
#> 12       M5 stop on W 31st St at 6th Ave is closed; for service, consider the southbound stop on 5th Ave at W 33rd St or the northbound stop on 6th Ave at W 31st St
#> 13       M5 stop on W 31st St at 6th Ave is closed; for service, consider the southbound stop on 5th Ave at W 33rd St or the northbound stop on 6th Ave at W 31st St
#> 14                                                       Westbound M66 stop on E 67th St at York Ave is closed, the first stop will be made on E 68th St at York Ave
#> 15                                                       Westbound M66 stop on E 67th St at York Ave is closed, the first stop will be made on E 68th St at York Ave
#> 16    All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 17    All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 18    All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 19    All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 20    All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 21    All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 22    All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 23    All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 24    All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 25    All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 26    All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 27    All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 28    All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 29    All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 30    All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 31    All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 32    All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 33    All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 34    All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 35    All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 36    All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 37    All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 38    All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 39    All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 40    All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 41    All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 42    All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 43    All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 44    All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 45    All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 46    All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 47    All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 48    All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 49    All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 50    All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 51    All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 52    All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 53    All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 54    All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 55    All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 56    All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 57    All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 58    All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 59    All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 60    All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 61    All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 62    All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 63    All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 64    All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 65    All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 66    All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 67    All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 68    All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 69    All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 70    All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 71    All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 72    All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 73                                                                                                                       You may wait longer for Queens North buses.
#> 74                                                                                                                       You may wait longer for Queens North buses.
#> 75                                                                                                                       You may wait longer for Queens North buses.
#> 76                                                                                                                       You may wait longer for Queens North buses.
#> 77                                                                                                                       You may wait longer for Queens North buses.
#> 78                                                                                                                       You may wait longer for Queens North buses.
#> 79                                                                                                                       You may wait longer for Queens North buses.
#> 80                                                                                                                       You may wait longer for Queens North buses.
#> 81                                                                                                                       You may wait longer for Queens North buses.
#> 82                                                                                                                       You may wait longer for Queens North buses.
#> 83                                                                                                                       You may wait longer for Queens North buses.
#> 84                                                                                                                       You may wait longer for Queens North buses.
#> 85                                                                                                                       You may wait longer for Queens North buses.
#> 86                                                                                                                       You may wait longer for Queens North buses.
#> 87                                                                                                                       You may wait longer for Queens North buses.
#> 88                                                                                                                       You may wait longer for Queens North buses.
#> 89                                                                                                                       You may wait longer for Queens North buses.
#> 90                                                                                                                       You may wait longer for Queens North buses.
#> 91                                                                                                                       You may wait longer for Queens North buses.
#> 92                                                                                                                       You may wait longer for Queens North buses.
#> 93                                                                                                                       You may wait longer for Queens North buses.
#> 94                                                                                                                       You may wait longer for Queens North buses.
#> 95                                                                                                                       You may wait longer for Queens North buses.
#> 96                                                                                                                       You may wait longer for Queens North buses.
#> 97                                                                                                                       You may wait longer for Queens North buses.
#> 98                                                                                                                       You may wait longer for Queens North buses.
#> 99                                                                                                                       You may wait longer for Queens North buses.
#> 100                                                                                                                      You may wait longer for Queens North buses.
#> 101                                                                                                                      You may wait longer for Queens North buses.
#> 102                                                                                                                      You may wait longer for Queens North buses.
#> 103                                                                                                                      You may wait longer for Queens North buses.
#> 104                                                                                                                      You may wait longer for Queens North buses.
#> 105                                                                                                                      You may wait longer for Queens North buses.
#> 106                                                                                                                      You may wait longer for Queens North buses.
#> 107                                                                                                                      You may wait longer for Queens North buses.
#> 108                                                                                                                      You may wait longer for Queens North buses.
#> 109                                                                                                                      You may wait longer for Queens North buses.
#> 110                                                                                                                      You may wait longer for Queens North buses.
#> 111                                                                                                                      You may wait longer for Queens North buses.
#> 112                                                                                                                      You may wait longer for Queens North buses.
#> 113                                                                                                                      You may wait longer for Queens North buses.
#> 114                                                                                                                      You may wait longer for Queens North buses.
#> 115                                                                                                                      You may wait longer for Queens North buses.
#> 116                                                                                                                      You may wait longer for Queens North buses.
#> 117                                                                                                                      You may wait longer for Queens North buses.
#> 118                                                                                                                      You may wait longer for Queens North buses.
#> 119                                                                                                                      You may wait longer for Queens North buses.
#> 120                                                                                                                      You may wait longer for Queens North buses.
#> 121                                                                                                                      You may wait longer for Queens North buses.
#> 122                                                                                                                      You may wait longer for Queens North buses.
#> 123                                                                                                                      You may wait longer for Queens North buses.
#> 124                                                                                                                      You may wait longer for Queens North buses.
#> 125                                                                                                                      You may wait longer for Queens North buses.
#> 126                                                                                                                      You may wait longer for Queens North buses.
#> 127                                                                                                                      You may wait longer for Queens North buses.
#> 128                                                                                                                      You may wait longer for Queens North buses.
#> 129                                                                                                                      You may wait longer for Queens North buses.
#> 130                                                                                                                      You may wait longer for Queens North buses.
#> 131                                                                                                                      You may wait longer for Queens North buses.
#> 132                                                                                                                      You may wait longer for Queens North buses.
#> 133                                                                                                                      You may wait longer for Queens North buses.
#> 134                                                                                                                      You may wait longer for Queens North buses.
#> 135                                                                                                                      You may wait longer for Queens North buses.
#> 136                                         Southbound B44 stop on Nostrand Ave at Avenue N is closed; use nearby stops on Nostrand Ave at Avenue M or Kings Highway
#> 137                                         Southbound B44 stop on Nostrand Ave at Avenue N is closed; use nearby stops on Nostrand Ave at Avenue M or Kings Highway
#> 138                                  Southbound B61 and northbound B57 stop on Otsego St at Lorraine St is closed, use the temporary stop on Otsego St at Creamer St
#> 139                                  Southbound B61 and northbound B57 stop on Otsego St at Lorraine St is closed, use the temporary stop on Otsego St at Creamer St
#> 140           Southbound M101, M102 and M103 buses are skipping the stop on Lexington Ave at E 46th St; use the temporary stop on Lexington Ave at E 45th St instead
#> 141           Southbound M101, M102 and M103 buses are skipping the stop on Lexington Ave at E 46th St; use the temporary stop on Lexington Ave at E 45th St instead
#> 142           Southbound M101, M102 and M103 buses are skipping the stop on Lexington Ave at E 46th St; use the temporary stop on Lexington Ave at E 45th St instead
#> 143     Southbound SIM22 and SIM26 are skipping the stop on Lexington Ave at E 45th St - use stops on Lexington Ave at E 53rd St or E 42nd St at Madison Ave instead
#> 144     Southbound SIM22 and SIM26 are skipping the stop on Lexington Ave at E 45th St - use stops on Lexington Ave at E 53rd St or E 42nd St at Madison Ave instead
#> 145                                                   Eastbound B8 stop on 18th Ave at 53rd St is closed; please use nearby stops on 18th Ave at 55th St or 50th St.
#> 146                                                   Eastbound B8 stop on 18th Ave at 53rd St is closed; please use nearby stops on 18th Ave at 55th St or 50th St.
#> 147                                                    Eastbound M66 stop on W 65th St at Columbus Ave is being bypassed; use the stop on W 65th at Broadway instead
#> 148                                                    Eastbound M66 stop on W 65th St at Columbus Ave is being bypassed; use the stop on W 65th at Broadway instead
#> 149                           Northbound B44 stop on Nostrand Ave at Avenue I is closed, please use nearby stops on Nostrand Ave at Avenue J or Flatbush Ave instead
#> 150                           Northbound B44 stop on Nostrand Ave at Avenue I is closed, please use nearby stops on Nostrand Ave at Avenue J or Flatbush Ave instead
#> 151                                      Northbound M15 and M15-SBS temporary stop on 1st Ave at E 81st St has been removed, use the stops on E 82nd St or E 79th St
#> 152                                      Northbound M15 and M15-SBS temporary stop on 1st Ave at E 81st St has been removed, use the stops on E 82nd St or E 79th St
#> 153                                         Westbound B65 stop on Smith St at Atlantic Ave is closed. Please use the stops on Smith St at Bergen St or Livingston St
#> 154                                         Westbound B65 stop on Smith St at Atlantic Ave is closed. Please use the stops on Smith St at Bergen St or Livingston St
#> 155                        Northbound B61 stop on Smith St at Atlantic Ave is closed - Please use the stops on Atlantic Ave at Court St or Smith St at Livingston St
#> 156                        Northbound B61 stop on Smith St at Atlantic Ave is closed - Please use the stops on Atlantic Ave at Court St or Smith St at Livingston St
#> 157                                                                            Williamsburg-bound Q54 and Q59 buses are bypassing the stop on Grand St at Graham Ave
#> 158                                                                            Williamsburg-bound Q54 and Q59 buses are bypassing the stop on Grand St at Graham Ave
#> 159                                              Eastbound M66 stop on Madison Ave at E 67th St will be bypassed, use the stop on E 68th St at Lexington Ave instead
#> 160                                              Eastbound M66 stop on Madison Ave at E 67th St will be bypassed, use the stop on E 68th St at Lexington Ave instead
#> 161                                                                           Southbound M15 stops on Water St at Hanover Square and Broad St at South St are closed
#> 162                                                                           Southbound M15 stops on Water St at Hanover Square and Broad St at South St are closed
#> 163               Southbound SIM6 and SIM11 buses are skipping the stop on Lexington Ave at E 45th St - use the temporary stop on Lexington Ave at E 43rd St instead
#> 164               Southbound SIM6 and SIM11 buses are skipping the stop on Lexington Ave at E 45th St - use the temporary stop on Lexington Ave at E 43rd St instead
#> 165                                                                                                                       M8 buses are not serving the Avenue D Loop
#> 166                                                                                                                       M8 buses are not serving the Avenue D Loop
#> 167                                          All Manhattan local and select buses are running with delays in both directions while we recover from the winter storm.
#> 168                                          All Manhattan local and select buses are running with delays in both directions while we recover from the winter storm.
#> 169                                          All Manhattan local and select buses are running with delays in both directions while we recover from the winter storm.
#> 170                                          All Manhattan local and select buses are running with delays in both directions while we recover from the winter storm.
#> 171                                          All Manhattan local and select buses are running with delays in both directions while we recover from the winter storm.
#> 172                                          All Manhattan local and select buses are running with delays in both directions while we recover from the winter storm.
#> 173                                          All Manhattan local and select buses are running with delays in both directions while we recover from the winter storm.
#> 174                                          All Manhattan local and select buses are running with delays in both directions while we recover from the winter storm.
#> 175                                          All Manhattan local and select buses are running with delays in both directions while we recover from the winter storm.
#> 176                                          All Manhattan local and select buses are running with delays in both directions while we recover from the winter storm.
#> 177                                          All Manhattan local and select buses are running with delays in both directions while we recover from the winter storm.
#> 178                                          All Manhattan local and select buses are running with delays in both directions while we recover from the winter storm.
#> 179                                          All Manhattan local and select buses are running with delays in both directions while we recover from the winter storm.
#> 180                                          All Manhattan local and select buses are running with delays in both directions while we recover from the winter storm.
#> 181                                          All Manhattan local and select buses are running with delays in both directions while we recover from the winter storm.
#> 182                                          All Manhattan local and select buses are running with delays in both directions while we recover from the winter storm.
#> 183                                          All Manhattan local and select buses are running with delays in both directions while we recover from the winter storm.
#> 184                                          All Manhattan local and select buses are running with delays in both directions while we recover from the winter storm.
#> 185                                          All Manhattan local and select buses are running with delays in both directions while we recover from the winter storm.
#> 186                                          All Manhattan local and select buses are running with delays in both directions while we recover from the winter storm.
#> 187                                          All Manhattan local and select buses are running with delays in both directions while we recover from the winter storm.
#> 188                                          All Manhattan local and select buses are running with delays in both directions while we recover from the winter storm.
#> 189                                          All Manhattan local and select buses are running with delays in both directions while we recover from the winter storm.
#> 190                                          All Manhattan local and select buses are running with delays in both directions while we recover from the winter storm.
#> 191                                          All Manhattan local and select buses are running with delays in both directions while we recover from the winter storm.
#> 192                                          All Manhattan local and select buses are running with delays in both directions while we recover from the winter storm.
#> 193                                          All Manhattan local and select buses are running with delays in both directions while we recover from the winter storm.
#> 194                                          All Manhattan local and select buses are running with delays in both directions while we recover from the winter storm.
#> 195                                          All Manhattan local and select buses are running with delays in both directions while we recover from the winter storm.
#> 196                                          All Manhattan local and select buses are running with delays in both directions while we recover from the winter storm.
#> 197                                          All Manhattan local and select buses are running with delays in both directions while we recover from the winter storm.
#> 198                                          All Manhattan local and select buses are running with delays in both directions while we recover from the winter storm.
#> 199                                          All Manhattan local and select buses are running with delays in both directions while we recover from the winter storm.
#> 200                                          All Manhattan local and select buses are running with delays in both directions while we recover from the winter storm.
#> 201                                          All Manhattan local and select buses are running with delays in both directions while we recover from the winter storm.
#> 202                                          All Manhattan local and select buses are running with delays in both directions while we recover from the winter storm.
#> 203                                          All Manhattan local and select buses are running with delays in both directions while we recover from the winter storm.
#> 204                                          All Manhattan local and select buses are running with delays in both directions while we recover from the winter storm.
#> 205                                          All Manhattan local and select buses are running with delays in both directions while we recover from the winter storm.
#> 206                                          All Manhattan local and select buses are running with delays in both directions while we recover from the winter storm.
#> 207                                          All Manhattan local and select buses are running with delays in both directions while we recover from the winter storm.
#> 208                                          All Manhattan local and select buses are running with delays in both directions while we recover from the winter storm.
#> 209                        All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.
#> 210                        All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.
#> 211                        All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.
#> 212                        All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.
#> 213                        All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.
#> 214                        All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.
#> 215                        All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.
#> 216                        All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.
#> 217                        All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.
#> 218                        All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.
#> 219                        All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.
#> 220                        All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.
#> 221                        All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.
#> 222                        All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.
#> 223                        All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.
#> 224                        All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.
#> 225                        All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.
#> 226                        All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.
#> 227                        All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.
#> 228                        All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.
#> 229                        All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.
#> 230                        All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.
#> 231                        All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.
#> 232                        All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.
#> 233                        All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.
#> 234                        All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.
#> 235                        All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.
#> 236                        All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.
#> 237                        All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.
#> 238                        All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.
#> 239                        All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.
#> 240                        All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.
#> 241                        All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.
#> 242                        All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.
#> 243                        All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.
#> 244                        All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.
#> 245                        All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.
#> 246                        All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.
#> 247                        All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.
#> 248                        All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.
#> 249                        All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.
#> 250                        All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.
#> 251                        All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.
#> 252                        All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.
#> 253                        All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.
#> 254                        All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.
#> 255                        All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.
#> 256                        All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.
#> 257                        All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.
#> 258                        All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.
#> 259                        All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.
#> 260                        All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.
#> 261                        All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.
#> 262                        All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.
#> 263                        All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.
#> 264                        All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.
#> 265                        All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.
#> 266                        All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.
#> 267                        All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.
#> 268                        All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.
#> 269                        All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.
#> 270                        All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.
#> 271                        All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.
#> 272                        All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.
#> 273                        All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.
#> 274                        All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.
#> 275                        All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.
#> 276                    All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.
#> 277                    All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.
#> 278                    All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.
#> 279                    All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.
#> 280                    All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.
#> 281                    All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.
#> 282                    All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.
#> 283                    All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.
#> 284                    All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.
#> 285                    All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.
#> 286                    All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.
#> 287                    All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.
#> 288                    All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.
#> 289                    All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.
#> 290                    All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.
#> 291                    All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.
#> 292                    All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.
#> 293                    All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.
#> 294                    All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.
#> 295                    All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.
#> 296                    All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.
#> 297                    All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.
#> 298                    All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.
#> 299                    All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.
#> 300                    All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.
#> 301                    All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.
#> 302                    All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.
#> 303                    All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.
#> 304                    All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.
#> 305                    All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.
#> 306                    All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.
#> 307                    All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.
#> 308                    All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.
#> 309                    All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.
#> 310                    All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.
#> 311                    All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.
#> 312                    All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.
#> 313                    All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.
#> 314                    All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.
#> 315                    All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.
#> 316                    All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.
#> 317                    All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.
#> 318                    All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.
#> 319                    All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.
#> 320                    All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.
#> 321                    All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.
#> 322                    All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.
#> 323                    All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.
#> 324                    All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.
#> 325                    All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.
#> 326                    All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.
#> 327                    All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.
#> 328                    All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.
#> 329                    All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.
#> 330                    All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.
#> 331                    All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.
#> 332                    All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.
#> 333                    All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.
#> 334                                                                                                        Southbound B67 stop on Flatbush Ave at State St is closed
#> 335                                                                                                        Southbound B67 stop on Flatbush Ave at State St is closed
#> 336                            Northbound B16 stop on Caton Ave at Ocean Pkwy is closed; buses will make a stop on Caton Ave between Ocean Pkwy and E 7th St instead
#> 337                            Northbound B16 stop on Caton Ave at Ocean Pkwy is closed; buses will make a stop on Caton Ave between Ocean Pkwy and E 7th St instead
#> 338                          Eastbound M79-SBS stop on W 81st St at Central Park West is temporarily closed; please use the nearby stop on W 81st St at Columbus Ave
#> 339                          Eastbound M79-SBS stop on W 81st St at Central Park West is temporarily closed; please use the nearby stop on W 81st St at Columbus Ave
#> 340                                                       St George-bound S48 buses are detoured from Richmond Terrace at Arlington Ave to Arlington Pl at South Ave
#> 341                                                       St George-bound S48 buses are detoured from Richmond Terrace at Arlington Ave to Arlington Pl at South Ave
#> 342                                           Maspeth-bound Q38 stop on 63rd Dr at Austin St has been temporarily relocated north between Wetherole St and Austin St
#> 343                                           Maspeth-bound Q38 stop on 63rd Dr at Austin St has been temporarily relocated north between Wetherole St and Austin St
#> 344                                                                           Q42 buses are detoured in both directions due to snow removal at Polhemus Ave/Wren Pl.
#> 345                                                                           Q42 buses are detoured in both directions due to snow removal at Polhemus Ave/Wren Pl.
#> 346                                                  B63 buses may experience delays on 5th Ave between 64th St and 65th St and will wait out any temporary closures
#> 347                                                  B63 buses may experience delays on 5th Ave between 64th St and 65th St and will wait out any temporary closures
#> 348                                                                                                    Northbound M15 stop on 1st Ave at E 77th St is being bypassed
#> 349                                                                                                    Northbound M15 stop on 1st Ave at E 77th St is being bypassed
#> 350                                                                      Bx8 buses are detoured in both directions due to snow removal  at Clarence Ave/Phillip Ave.
#> 351                                                                      Bx8 buses are detoured in both directions due to snow removal  at Clarence Ave/Phillip Ave.
#> 352                                          B69 northbound stop on Vanderbilt Ave at Sterling Pl and southbound stop on Vanderbilt Ave at Park Pl is being bypassed
#> 353                                          B69 northbound stop on Vanderbilt Ave at Sterling Pl and southbound stop on Vanderbilt Ave at Park Pl is being bypassed
#> 354                                                                                                    Southbound B82 stop on and Mermaid Ave at W 17th St is closed
#> 355                                                                                                    Southbound B82 stop on and Mermaid Ave at W 17th St is closed
#> 356                                                                                                              You may wait longer for these buses:\nS54, S74, S78
#> 357                                                                                                              You may wait longer for these buses:\nS54, S74, S78
#> 358                                                                                                              You may wait longer for these buses:\nS54, S74, S78
#> 359                                                                                                               Eastbound B65 stop on Dean St at 4th Ave is closed
#> 360                                                                                                               Eastbound B65 stop on Dean St at 4th Ave is closed
#> 361                                                                          SIM33 and SIM34 buses will be detoured in both directions at South Ave/Arlington Place.
#> 362                                                                          SIM33 and SIM34 buses will be detoured in both directions at South Ave/Arlington Place.
#> 363                    Q4, Q5, Q84, Q85, Q86, Q87, Q89 and QM63 buses are running with delays due to utility work on Merrick Blvd between 110th Ave and Linden Blvd.
#> 364                    Q4, Q5, Q84, Q85, Q86, Q87, Q89 and QM63 buses are running with delays due to utility work on Merrick Blvd between 110th Ave and Linden Blvd.
#> 365                    Q4, Q5, Q84, Q85, Q86, Q87, Q89 and QM63 buses are running with delays due to utility work on Merrick Blvd between 110th Ave and Linden Blvd.
#> 366                    Q4, Q5, Q84, Q85, Q86, Q87, Q89 and QM63 buses are running with delays due to utility work on Merrick Blvd between 110th Ave and Linden Blvd.
#> 367                    Q4, Q5, Q84, Q85, Q86, Q87, Q89 and QM63 buses are running with delays due to utility work on Merrick Blvd between 110th Ave and Linden Blvd.
#> 368                    Q4, Q5, Q84, Q85, Q86, Q87, Q89 and QM63 buses are running with delays due to utility work on Merrick Blvd between 110th Ave and Linden Blvd.
#> 369                    Q4, Q5, Q84, Q85, Q86, Q87, Q89 and QM63 buses are running with delays due to utility work on Merrick Blvd between 110th Ave and Linden Blvd.
#> 370                                            S40 buses are detoured in both directions between South Ave at Arlington Place and Richmond Terrace at Arlington Ave.
#> 371                                            S40 buses are detoured in both directions between South Ave at Arlington Place and Richmond Terrace at Arlington Ave.
#> 372                                        Northbound SIM26 drop-off on Madison Ave at 48th St will be made at the BxM4/BxM11 stop on Madison Ave at 47th St instead
#> 373                                        Northbound SIM26 drop-off on Madison Ave at 48th St will be made at the BxM4/BxM11 stop on Madison Ave at 47th St instead
#> 374                               Northbound M20 and M55 stop on State St at Bridge St is closed; use the temporary stop on State St before the intersection instead
#> 375                               Northbound M20 and M55 stop on State St at Bridge St is closed; use the temporary stop on State St before the intersection instead
#> 376                                                                                   Eastbound M35 stop on Rivers Edge Rd at Manhattan Psychiatric Center is closed
#> 377                                                                                   Eastbound M35 stop on Rivers Edge Rd at Manhattan Psychiatric Center is closed
#> 378                Staten Island-bound SIM5, SIM15 and SIM35 stop on State St at Bridge St is closed; use the temporary stop on State St closer to Bridge St instead
#> 379                Staten Island-bound SIM5, SIM15 and SIM35 stop on State St at Bridge St is closed; use the temporary stop on State St closer to Bridge St instead
#> 380                Staten Island-bound SIM5, SIM15 and SIM35 stop on State St at Bridge St is closed; use the temporary stop on State St closer to Bridge St instead
#> 381                                                                                           SIM4X/SIM8X trips will be discontinued, more trips coming to SIM4/SIM8
#> 382                                                                                           SIM4X/SIM8X trips will be discontinued, more trips coming to SIM4/SIM8
#> 383 Southbound B20 stop on Fairview Ave at Putnam Ave is discontinued; northbound buses are now stopping at the existing southbound stop on Putnam Ave at Forest Ave
#> 384 Southbound B20 stop on Fairview Ave at Putnam Ave is discontinued; northbound buses are now stopping at the existing southbound stop on Putnam Ave at Forest Ave
#> 385                                                           Bx39 and BxM11 buses are detoured in both directions due to FDNY activity at White Plains Rd/Adee Ave.
#> 386                                                           Bx39 and BxM11 buses are detoured in both directions due to FDNY activity at White Plains Rd/Adee Ave.
#> 387                                                           Southbound M98 are detouring in Harlem; you will experience a few extra turns, but no stops are missed
#> 388                                                           Southbound M98 are detouring in Harlem; you will experience a few extra turns, but no stops are missed
#> 389                                                            Northbound B60 buses are detoured due to utility work on Meserole St between Union Ave and Marcy Ave.
#> 390                                                            Northbound B60 buses are detoured due to utility work on Meserole St between Union Ave and Marcy Ave.
#> 391                                                    Southbound M15, M15-SBS, M20, and M55 buses are detoured due to snow removal operations at Broad St/South St.
#> 392                                                    Southbound M15, M15-SBS, M20, and M55 buses are detoured due to snow removal operations at Broad St/South St.
#> 393                                                    Southbound M15, M15-SBS, M20, and M55 buses are detoured due to snow removal operations at Broad St/South St.
#> 394                                                    Southbound M15, M15-SBS, M20, and M55 buses are detoured due to snow removal operations at Broad St/South St.
#> 395                                                   Northbound Q61 stop on Linden Pl at 35th Ave is closed; buses make a stop on Farrington St at 35th Ave instead
#> 396                                                   Northbound Q61 stop on Linden Pl at 35th Ave is closed; buses make a stop on Farrington St at 35th Ave instead
#> 397                                                                                             Northbound M15 and M15-SBS stop on 1st Ave at 79th has been restored
#> 398                                                                                             Northbound M15 and M15-SBS stop on 1st Ave at 79th has been restored
#> 399                             B12 buses are detoured between Pitkin Ave at Rockaway Ave and Eastern Parkway main roadway at Rockaway Ave - no stops will be missed
#> 400                             B12 buses are detoured between Pitkin Ave at Rockaway Ave and Eastern Parkway main roadway at Rockaway Ave - no stops will be missed
#> 401                                                                    Northbound BxM2, BxM3, BxM4, and BxM11 buses are bypassing the stop on Madison Ave at 99th St
#> 402                                                                    Northbound BxM2, BxM3, BxM4, and BxM11 buses are bypassing the stop on Madison Ave at 99th St
#> 403                                                                    Northbound BxM2, BxM3, BxM4, and BxM11 buses are bypassing the stop on Madison Ave at 99th St
#> 404                                                                    Northbound BxM2, BxM3, BxM4, and BxM11 buses are bypassing the stop on Madison Ave at 99th St
#> 405                              Northbound BxM3 and BxM4 stop on Madison Ave at E 46th St is closed, please use the M3 bus stop on Madison Ave at E 45th St instead
#> 406                              Northbound BxM3 and BxM4 stop on Madison Ave at E 46th St is closed, please use the M3 bus stop on Madison Ave at E 45th St instead
#> 407                                                                        Southbound Q47 buses are detoured due to an illegally parked vehicle at 25th Ave/78th St.
#> 408                                                                        Southbound Q47 buses are detoured due to an illegally parked vehicle at 25th Ave/78th St.
#> 409              Eastbound Q32 stop on Roosevelt Ave at 61st St has been temporarily relocated to Roosevelt Ave between 60th St and 61st St, before the intersection
#> 410              Eastbound Q32 stop on Roosevelt Ave at 61st St has been temporarily relocated to Roosevelt Ave between 60th St and 61st St, before the intersection
#> 411                                                                                    The 8:30 AM QM6 bus trip scheduled to depart North Shore Towers will not run.
#> 412                                                                                    The 8:30 AM QM6 bus trip scheduled to depart North Shore Towers will not run.
#> 413                                                              Q18 buses are detoured in both directions due to icy roads on 53rd Ave between 69th St and 65th Pl.
#> 414                                                              Q18 buses are detoured in both directions due to icy roads on 53rd Ave between 69th St and 65th Pl.
#> 415                                          Westbound Q7 stop on Jamaica Ave at Eldert Ln is closed; customers will be dropped off on Eldert Ln at 87th Ave instead
#> 416                                          Westbound Q7 stop on Jamaica Ave at Eldert Ln is closed; customers will be dropped off on Eldert Ln at 87th Ave instead
#> 417                          Southbound BxM11 buses are detouring from E 177th St at Bronx River Parkway to Bruckner Blvd at Bruckner Expy - no stops will be missed
#> 418                          Southbound BxM11 buses are detouring from E 177th St at Bronx River Parkway to Bruckner Blvd at Bruckner Expy - no stops will be missed
#> 419                                                                               The 8:30 AM QM5 bus trip scheduled to depart Union Turnpike/260th St will not run.
#> 420                                                                               The 8:30 AM QM5 bus trip scheduled to depart Union Turnpike/260th St will not run.
#> 421                              Southbound Q52-SBS and Q53-SBS stop on Cross Bay Blvd at Liberty Ave has been temporarily relocated down the block before 107th Ave
#> 422                              Southbound Q52-SBS and Q53-SBS stop on Cross Bay Blvd at Liberty Ave has been temporarily relocated down the block before 107th Ave
#> 423                                                   Northbound Q25 stop on Linden Pl at 35th Ave is closed; buses make a stop on Farrington St at 35th Ave instead
#> 424                                                   Northbound Q25 stop on Linden Pl at 35th Ave is closed; buses make a stop on Farrington St at 35th Ave instead
#> 425                                                                     Q47 buses are detoured in both directions because of a road blockage at Calamus Ave/79th St.
#> 426                                                                     Q47 buses are detoured in both directions because of a road blockage at Calamus Ave/79th St.
#> 427                               Bronx-bound Q50 buses may experience delays in Flushing, but no stops are missed; buses use Farrington St from 35th Ave to 31st Rd
#> 428                               Bronx-bound Q50 buses may experience delays in Flushing, but no stops are missed; buses use Farrington St from 35th Ave to 31st Rd
#> 429                                Eastbound Q32 and Q60 stop on Queens Blvd at 45th St is closed; please use the stops on Queens Blvd at 41st St or 46th St instead
#> 430                                Eastbound Q32 and Q60 stop on Queens Blvd at 45th St is closed; please use the stops on Queens Blvd at 41st St or 46th St instead
#>                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         description_text
#> 1                                                                                                                                                                                                                                                                                                                     X28  and X38  stops on Surf Ave at W 21st St are closed in both directions\nFor northbound service, use the temporary stop on Surf Ave at W 22nd St.\nFor southbound service, use the stop on W 17th St at Mermaid Ave.\n\nWhat's happening?\nConstruction\n\nNote: Bus arrival information may not be available/accurate while buses are detoured
#> 2                                                                                                                                                                                                                                                                                                                     X28  and X38  stops on Surf Ave at W 21st St are closed in both directions\nFor northbound service, use the temporary stop on Surf Ave at W 22nd St.\nFor southbound service, use the stop on W 17th St at Mermaid Ave.\n\nWhat's happening?\nConstruction\n\nNote: Bus arrival information may not be available/accurate while buses are detoured
#> 3                                                                                                                                                                                                                                                        Northbound M15-SBS stop on Water St at Pine St is closed, buses make a temporary stop across the intersection on Water St at Pine St\nSee a map of this stop change.\n\n~If paying with MetroCard, obtain tickets from the vending machine at the original stop on Water St/Pine St.\n\nWhat's happening?\nConstruction\n\nNote: Bus arrival information may not be available/accurate while buses are detoured
#> 4                                                                                                                                                                                                                                                        Northbound M15-SBS stop on Water St at Pine St is closed, buses make a temporary stop across the intersection on Water St at Pine St\nSee a map of this stop change.\n\n~If paying with MetroCard, obtain tickets from the vending machine at the original stop on Water St/Pine St.\n\nWhat's happening?\nConstruction\n\nNote: Bus arrival information may not be available/accurate while buses are detoured
#> 5                                                                                                                                                                                                                                                                                                          S66 buses are detoured in both directions due to icy roads at Arlo Rd/Stratford Ave.\nS66 buses in both directions will not serve stops from Clove Rd/Niagara St to Arlo Rd/Stratford Ave.\n\nWhile detoured, S66 buses will make requested stops along Clove Rd.\n\nNote: Bus arrival information may not be accurate or available while buses are detoured.
#> 6                                                                                                                                                                                                                                                                                                          S66 buses are detoured in both directions due to icy roads at Arlo Rd/Stratford Ave.\nS66 buses in both directions will not serve stops from Clove Rd/Niagara St to Arlo Rd/Stratford Ave.\n\nWhile detoured, S66 buses will make requested stops along Clove Rd.\n\nNote: Bus arrival information may not be accurate or available while buses are detoured.
#> 7                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      You may wait longer for these buses:\nB9, B35, B63\nWe're running as much service as we can with the operators we have available.
#> 8                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      You may wait longer for these buses:\nB9, B35, B63\nWe're running as much service as we can with the operators we have available.
#> 9                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      You may wait longer for these buses:\nB9, B35, B63\nWe're running as much service as we can with the operators we have available.
#> 10                                                                                                                                                                                                                                                                                                                                                                                                                                                               Southbound Bx39 stop on White Plains Rd at E 215th St is closed; please use the new stop on White Plains Rd at E 216th St instead\nSee a map of the boarding change.\n\nWhat's happening?\nConstruction
#> 11                                                                                                                                                                                                                                                                                                                                                                                                                                                               Southbound Bx39 stop on White Plains Rd at E 215th St is closed; please use the new stop on White Plains Rd at E 216th St instead\nSee a map of the boarding change.\n\nWhat's happening?\nConstruction
#> 12                                                                                                                                                                                                                                                          M5 stop on W 31st St at 6th Ave is closed; for service, consider the southbound stop on 5th Ave at W 33rd St or the northbound stop on 6th Ave at W 31st St\nBuses detour via 23rd St (see map).\n\nSouthbound customers may ride through the detour to 6th Ave at W 31st St.\n\nWhat's happening?\nConstruction\n\nNote: Bus arrival information may not be accurate or available while buses are detoured.
#> 13                                                                                                                                                                                                                                                          M5 stop on W 31st St at 6th Ave is closed; for service, consider the southbound stop on 5th Ave at W 33rd St or the northbound stop on 6th Ave at W 31st St\nBuses detour via 23rd St (see map).\n\nSouthbound customers may ride through the detour to 6th Ave at W 31st St.\n\nWhat's happening?\nConstruction\n\nNote: Bus arrival information may not be accurate or available while buses are detoured.
#> 14                                                                                                                                                                                                                                                                                                                                                                                                                                  Westbound M66 stop on E 67th St at York Ave is closed, the first stop will be made on E 68th St at York Ave\nWhat's happening?\nConstruction\n\nNote: Bus arrival information may not be available/accurate while buses are detoured
#> 15                                                                                                                                                                                                                                                                                                                                                                                                                                  Westbound M66 stop on E 67th St at York Ave is closed, the first stop will be made on E 68th St at York Ave\nWhat's happening?\nConstruction\n\nNote: Bus arrival information may not be available/accurate while buses are detoured
#> 16                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 17                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 18                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 19                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 20                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 21                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 22                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 23                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 24                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 25                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 26                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 27                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 28                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 29                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 30                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 31                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 32                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 33                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 34                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 35                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 36                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 37                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 38                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 39                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 40                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 41                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 42                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 43                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 44                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 45                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 46                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 47                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 48                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 49                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 50                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 51                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 52                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 53                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 54                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 55                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 56                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 57                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 58                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 59                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 60                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 61                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 62                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 63                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 64                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 65                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 66                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 67                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 68                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 69                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 70                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 71                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 72                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        All Bronx express, limited, local, and select buses are running with delays in both directions due to inclement weather.\nPlease allow additional travel time.
#> 73                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                You may wait longer for Queens North buses.\nWe?re running as much service as we can with the buses we have available.
#> 74                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                You may wait longer for Queens North buses.\nWe?re running as much service as we can with the buses we have available.
#> 75                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                You may wait longer for Queens North buses.\nWe?re running as much service as we can with the buses we have available.
#> 76                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                You may wait longer for Queens North buses.\nWe?re running as much service as we can with the buses we have available.
#> 77                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                You may wait longer for Queens North buses.\nWe?re running as much service as we can with the buses we have available.
#> 78                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                You may wait longer for Queens North buses.\nWe?re running as much service as we can with the buses we have available.
#> 79                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                You may wait longer for Queens North buses.\nWe?re running as much service as we can with the buses we have available.
#> 80                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                You may wait longer for Queens North buses.\nWe?re running as much service as we can with the buses we have available.
#> 81                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                You may wait longer for Queens North buses.\nWe?re running as much service as we can with the buses we have available.
#> 82                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                You may wait longer for Queens North buses.\nWe?re running as much service as we can with the buses we have available.
#> 83                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                You may wait longer for Queens North buses.\nWe?re running as much service as we can with the buses we have available.
#> 84                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                You may wait longer for Queens North buses.\nWe?re running as much service as we can with the buses we have available.
#> 85                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                You may wait longer for Queens North buses.\nWe?re running as much service as we can with the buses we have available.
#> 86                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                You may wait longer for Queens North buses.\nWe?re running as much service as we can with the buses we have available.
#> 87                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                You may wait longer for Queens North buses.\nWe?re running as much service as we can with the buses we have available.
#> 88                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                You may wait longer for Queens North buses.\nWe?re running as much service as we can with the buses we have available.
#> 89                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                You may wait longer for Queens North buses.\nWe?re running as much service as we can with the buses we have available.
#> 90                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                You may wait longer for Queens North buses.\nWe?re running as much service as we can with the buses we have available.
#> 91                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                You may wait longer for Queens North buses.\nWe?re running as much service as we can with the buses we have available.
#> 92                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                You may wait longer for Queens North buses.\nWe?re running as much service as we can with the buses we have available.
#> 93                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                You may wait longer for Queens North buses.\nWe?re running as much service as we can with the buses we have available.
#> 94                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                You may wait longer for Queens North buses.\nWe?re running as much service as we can with the buses we have available.
#> 95                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                You may wait longer for Queens North buses.\nWe?re running as much service as we can with the buses we have available.
#> 96                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                You may wait longer for Queens North buses.\nWe?re running as much service as we can with the buses we have available.
#> 97                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                You may wait longer for Queens North buses.\nWe?re running as much service as we can with the buses we have available.
#> 98                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                You may wait longer for Queens North buses.\nWe?re running as much service as we can with the buses we have available.
#> 99                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                You may wait longer for Queens North buses.\nWe?re running as much service as we can with the buses we have available.
#> 100                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               You may wait longer for Queens North buses.\nWe?re running as much service as we can with the buses we have available.
#> 101                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               You may wait longer for Queens North buses.\nWe?re running as much service as we can with the buses we have available.
#> 102                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               You may wait longer for Queens North buses.\nWe?re running as much service as we can with the buses we have available.
#> 103                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               You may wait longer for Queens North buses.\nWe?re running as much service as we can with the buses we have available.
#> 104                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               You may wait longer for Queens North buses.\nWe?re running as much service as we can with the buses we have available.
#> 105                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               You may wait longer for Queens North buses.\nWe?re running as much service as we can with the buses we have available.
#> 106                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               You may wait longer for Queens North buses.\nWe?re running as much service as we can with the buses we have available.
#> 107                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               You may wait longer for Queens North buses.\nWe?re running as much service as we can with the buses we have available.
#> 108                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               You may wait longer for Queens North buses.\nWe?re running as much service as we can with the buses we have available.
#> 109                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               You may wait longer for Queens North buses.\nWe?re running as much service as we can with the buses we have available.
#> 110                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               You may wait longer for Queens North buses.\nWe?re running as much service as we can with the buses we have available.
#> 111                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               You may wait longer for Queens North buses.\nWe?re running as much service as we can with the buses we have available.
#> 112                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               You may wait longer for Queens North buses.\nWe?re running as much service as we can with the buses we have available.
#> 113                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               You may wait longer for Queens North buses.\nWe?re running as much service as we can with the buses we have available.
#> 114                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               You may wait longer for Queens North buses.\nWe?re running as much service as we can with the buses we have available.
#> 115                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               You may wait longer for Queens North buses.\nWe?re running as much service as we can with the buses we have available.
#> 116                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               You may wait longer for Queens North buses.\nWe?re running as much service as we can with the buses we have available.
#> 117                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               You may wait longer for Queens North buses.\nWe?re running as much service as we can with the buses we have available.
#> 118                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               You may wait longer for Queens North buses.\nWe?re running as much service as we can with the buses we have available.
#> 119                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               You may wait longer for Queens North buses.\nWe?re running as much service as we can with the buses we have available.
#> 120                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               You may wait longer for Queens North buses.\nWe?re running as much service as we can with the buses we have available.
#> 121                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               You may wait longer for Queens North buses.\nWe?re running as much service as we can with the buses we have available.
#> 122                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               You may wait longer for Queens North buses.\nWe?re running as much service as we can with the buses we have available.
#> 123                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               You may wait longer for Queens North buses.\nWe?re running as much service as we can with the buses we have available.
#> 124                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               You may wait longer for Queens North buses.\nWe?re running as much service as we can with the buses we have available.
#> 125                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               You may wait longer for Queens North buses.\nWe?re running as much service as we can with the buses we have available.
#> 126                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               You may wait longer for Queens North buses.\nWe?re running as much service as we can with the buses we have available.
#> 127                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               You may wait longer for Queens North buses.\nWe?re running as much service as we can with the buses we have available.
#> 128                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               You may wait longer for Queens North buses.\nWe?re running as much service as we can with the buses we have available.
#> 129                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               You may wait longer for Queens North buses.\nWe?re running as much service as we can with the buses we have available.
#> 130                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               You may wait longer for Queens North buses.\nWe?re running as much service as we can with the buses we have available.
#> 131                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               You may wait longer for Queens North buses.\nWe?re running as much service as we can with the buses we have available.
#> 132                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               You may wait longer for Queens North buses.\nWe?re running as much service as we can with the buses we have available.
#> 133                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               You may wait longer for Queens North buses.\nWe?re running as much service as we can with the buses we have available.
#> 134                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               You may wait longer for Queens North buses.\nWe?re running as much service as we can with the buses we have available.
#> 135                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               You may wait longer for Queens North buses.\nWe?re running as much service as we can with the buses we have available.
#> 136                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             Southbound B44 stop on Nostrand Ave at Avenue N is closed; use nearby stops on Nostrand Ave at Avenue M or Kings Highway\nSee a map of this stop change.
#> 137                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             Southbound B44 stop on Nostrand Ave at Avenue N is closed; use nearby stops on Nostrand Ave at Avenue M or Kings Highway\nSee a map of this stop change.
#> 138                                                                                                                                                                                                                                                                                                                                                                             Southbound B61 and northbound B57 stop on Otsego St at Lorraine St is closed, use the temporary stop on Otsego St at Creamer St\nSee a map of the stop change.\n\nWhat's happening?\nConstruction\n\nNote: Bus arrival information may not be available/accurate while buses are detoure
#> 139                                                                                                                                                                                                                                                                                                                                                                             Southbound B61 and northbound B57 stop on Otsego St at Lorraine St is closed, use the temporary stop on Otsego St at Creamer St\nSee a map of the stop change.\n\nWhat's happening?\nConstruction\n\nNote: Bus arrival information may not be available/accurate while buses are detoure
#> 140                                                                                                                                                                                                                                                                                                                                                                                                                              Southbound M101, M102 and M103 buses are skipping the stop on Lexington Ave at E 46th St; use the temporary stop on Lexington Ave at E 45th St instead\nSee a map of the boarding change.\n\nWhat's happening?\nWater main construction
#> 141                                                                                                                                                                                                                                                                                                                                                                                                                              Southbound M101, M102 and M103 buses are skipping the stop on Lexington Ave at E 46th St; use the temporary stop on Lexington Ave at E 45th St instead\nSee a map of the boarding change.\n\nWhat's happening?\nWater main construction
#> 142                                                                                                                                                                                                                                                                                                                                                                                                                              Southbound M101, M102 and M103 buses are skipping the stop on Lexington Ave at E 46th St; use the temporary stop on Lexington Ave at E 45th St instead\nSee a map of the boarding change.\n\nWhat's happening?\nWater main construction
#> 143                                                                                                                                                                                                                                                                                                                                                                                                                                 Southbound SIM22 and SIM26 are skipping the stop on Lexington Ave at E 45th St - use stops on Lexington Ave at E 53rd St or E 42nd St at Madison Ave instead\nSee a map of the bypass.\n\nWhat's happening?\nWater main construction
#> 144                                                                                                                                                                                                                                                                                                                                                                                                                                 Southbound SIM22 and SIM26 are skipping the stop on Lexington Ave at E 45th St - use stops on Lexington Ave at E 53rd St or E 42nd St at Madison Ave instead\nSee a map of the bypass.\n\nWhat's happening?\nWater main construction
#> 145                                                                                                                                                                                                                                                                                                                                                               Eastbound B8 stop on 18th Ave at 53rd St is closed; please use nearby stops on 18th Ave at 55th St or 50th St.\nSee a map of the stops.\n\nWhat's happening?\n18th Avenue Bridge maintenance and rehabilitation\n\nNote: Bus arrival information may not be available/accurate while buses are detoure
#> 146                                                                                                                                                                                                                                                                                                                                                               Eastbound B8 stop on 18th Ave at 53rd St is closed; please use nearby stops on 18th Ave at 55th St or 50th St.\nSee a map of the stops.\n\nWhat's happening?\n18th Avenue Bridge maintenance and rehabilitation\n\nNote: Bus arrival information may not be available/accurate while buses are detoure
#> 147                                                                                                                                                                                                                                                                                                                                                                                    Eastbound M66 stop on W 65th St at Columbus Ave is being bypassed; use the stop on W 65th at Broadway instead\nSee a map of this stop change.\n\nWhat's happening?\nBuilding Construction\n\nNote: Bus arrival information may not be available/accurate while buses are detoured
#> 148                                                                                                                                                                                                                                                                                                                                                                                    Eastbound M66 stop on W 65th St at Columbus Ave is being bypassed; use the stop on W 65th at Broadway instead\nSee a map of this stop change.\n\nWhat's happening?\nBuilding Construction\n\nNote: Bus arrival information may not be available/accurate while buses are detoured
#> 149                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 Northbound B44 stop on Nostrand Ave at Avenue I is closed, please use nearby stops on Nostrand Ave at Avenue J or Flatbush Ave instead\n(see map)\n\nWhat's happening?\nConstruction
#> 150                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 Northbound B44 stop on Nostrand Ave at Avenue I is closed, please use nearby stops on Nostrand Ave at Avenue J or Flatbush Ave instead\n(see map)\n\nWhat's happening?\nConstruction
#> 151                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    Northbound M15 and M15-SBS temporary stop on 1st Ave at E 81st St has been removed, use the stops on E 82nd St or E 79th St\nSee map\n\nWhat's happening?\nConstruction completed
#> 152                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    Northbound M15 and M15-SBS temporary stop on 1st Ave at E 81st St has been removed, use the stops on E 82nd St or E 79th St\nSee map\n\nWhat's happening?\nConstruction completed
#> 153                                                                                                                                                                                                                                                                                                                                                                                  Westbound B65 stop on Smith St at Atlantic Ave is closed. Please use the stops on Smith St at Bergen St or Livingston St\nSee a map of this stop change.\n\nWhat's happening?\nConstruction\n\nNote: Bus arrival information may not be available/accurate while buses are detoured
#> 154                                                                                                                                                                                                                                                                                                                                                                                  Westbound B65 stop on Smith St at Atlantic Ave is closed. Please use the stops on Smith St at Bergen St or Livingston St\nSee a map of this stop change.\n\nWhat's happening?\nConstruction\n\nNote: Bus arrival information may not be available/accurate while buses are detoured
#> 155                                                                                                                                                                                                                                                                                                                                                                  Northbound B61 stop on Smith St at Atlantic Ave is closed - Please use the stops on Atlantic Ave at Court St or Smith St at Livingston St\nSee a map of this stop change.\n\nWhat's happening?\nConstruction\n\nNote: Bus arrival information may not be available/accurate while buses are detoure
#> 156                                                                                                                                                                                                                                                                                                                                                                  Northbound B61 stop on Smith St at Atlantic Ave is closed - Please use the stops on Atlantic Ave at Court St or Smith St at Livingston St\nSee a map of this stop change.\n\nWhat's happening?\nConstruction\n\nNote: Bus arrival information may not be available/accurate while buses are detoure
#> 157                                                                                                                                                                                                                                                                                                                                               Williamsburg-bound Q54 and Q59 buses are bypassing the stop on Grand St at Graham Ave\nFor service, use the stops on Grand St at Bushwick Ave or Lorimer St\nSee a map of this stop change.\n\nWhat's happening?\nConstruction\n\nNote: Bus arrival information may not be available/accurate while buses are detoured
#> 158                                                                                                                                                                                                                                                                                                                                               Williamsburg-bound Q54 and Q59 buses are bypassing the stop on Grand St at Graham Ave\nFor service, use the stops on Grand St at Bushwick Ave or Lorimer St\nSee a map of this stop change.\n\nWhat's happening?\nConstruction\n\nNote: Bus arrival information may not be available/accurate while buses are detoured
#> 159                                                                                                                                                                                                                                                                                                                                                                                     Eastbound M66 stop on Madison Ave at E 67th St will be bypassed, use the stop on E 68th St at Lexington Ave instead\nHere's a map of the stop change.\n\nWhat's happening?\nConstruction\n\nNote: Bus arrival information may not be available/accurate while buses are detoured
#> 160                                                                                                                                                                                                                                                                                                                                                                                     Eastbound M66 stop on Madison Ave at E 67th St will be bypassed, use the stop on E 68th St at Lexington Ave instead\nHere's a map of the stop change.\n\nWhat's happening?\nConstruction\n\nNote: Bus arrival information may not be available/accurate while buses are detoured
#> 161                                                                                                                                                                                                                                                                                            Southbound M15 stops on Water St at Hanover Square and Broad St at South St are closed\nFor service, use the stops on Water St at Gouverneur Lane or South St at Whitehall St.\nSee map\n\nBuses operate via Old Slip and South St.\n\n\nWhat's happening?\nWinter storm recovery\n\nNote: Bus arrival information may not be available/accurate while buses are detoured
#> 162                                                                                                                                                                                                                                                                                            Southbound M15 stops on Water St at Hanover Square and Broad St at South St are closed\nFor service, use the stops on Water St at Gouverneur Lane or South St at Whitehall St.\nSee map\n\nBuses operate via Old Slip and South St.\n\n\nWhat's happening?\nWinter storm recovery\n\nNote: Bus arrival information may not be available/accurate while buses are detoured
#> 163                                                                                                                                                                                                                                                                                                                                                                                                                                                                       Southbound SIM6 and SIM11 buses are skipping the stop on Lexington Ave at E 45th St - use the temporary stop on Lexington Ave at E 43rd St instead\nWhat's happening?\nWater main construction
#> 164                                                                                                                                                                                                                                                                                                                                                                                                                                                                       Southbound SIM6 and SIM11 buses are skipping the stop on Lexington Ave at E 45th St - use the temporary stop on Lexington Ave at E 43rd St instead\nWhat's happening?\nWater main construction
#> 165                                                                                                                                                                                                                                                                                                                                                                                                   M8 buses are not serving the Avenue D Loop\nThe first and last stops will be made on E 10th St at Avenue D in both directions.\nSee map\n\nWhat's happening?\nConstruction\n\nNote: Bus arrival information may not be available/accurate while buses are detoured
#> 166                                                                                                                                                                                                                                                                                                                                                                                                   M8 buses are not serving the Avenue D Loop\nThe first and last stops will be made on E 10th St at Avenue D in both directions.\nSee map\n\nWhat's happening?\nConstruction\n\nNote: Bus arrival information may not be available/accurate while buses are detoured
#> 167                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               All Manhattan local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 168                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               All Manhattan local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 169                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               All Manhattan local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 170                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               All Manhattan local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 171                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               All Manhattan local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 172                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               All Manhattan local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 173                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               All Manhattan local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 174                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               All Manhattan local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 175                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               All Manhattan local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 176                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               All Manhattan local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 177                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               All Manhattan local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 178                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               All Manhattan local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 179                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               All Manhattan local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 180                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               All Manhattan local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 181                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               All Manhattan local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 182                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               All Manhattan local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 183                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               All Manhattan local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 184                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               All Manhattan local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 185                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               All Manhattan local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 186                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               All Manhattan local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 187                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               All Manhattan local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 188                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               All Manhattan local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 189                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               All Manhattan local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 190                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               All Manhattan local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 191                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               All Manhattan local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 192                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               All Manhattan local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 193                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               All Manhattan local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 194                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               All Manhattan local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 195                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               All Manhattan local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 196                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               All Manhattan local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 197                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               All Manhattan local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 198                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               All Manhattan local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 199                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               All Manhattan local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 200                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               All Manhattan local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 201                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               All Manhattan local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 202                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               All Manhattan local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 203                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               All Manhattan local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 204                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               All Manhattan local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 205                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               All Manhattan local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 206                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               All Manhattan local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 207                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               All Manhattan local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 208                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               All Manhattan local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 209                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 210                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 211                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 212                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 213                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 214                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 215                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 216                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 217                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 218                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 219                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 220                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 221                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 222                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 223                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 224                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 225                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 226                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 227                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 228                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 229                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 230                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 231                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 232                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 233                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 234                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 235                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 236                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 237                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 238                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 239                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 240                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 241                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 242                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 243                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 244                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 245                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 246                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 247                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 248                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 249                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 250                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 251                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 252                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 253                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 254                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 255                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 256                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 257                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 258                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 259                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 260                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 261                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 262                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 263                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 264                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 265                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 266                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 267                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 268                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 269                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 270                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 271                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 272                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 273                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 274                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 275                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             All Brooklyn express, limited, local, and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 276                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 277                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 278                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 279                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 280                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 281                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 282                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 283                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 284                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 285                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 286                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 287                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 288                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 289                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 290                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 291                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 292                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 293                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 294                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 295                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 296                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 297                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 298                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 299                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 300                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 301                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 302                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 303                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 304                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 305                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 306                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 307                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 308                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 309                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 310                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 311                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 312                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 313                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 314                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 315                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 316                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 317                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 318                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 319                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 320                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 321                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 322                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 323                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 324                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 325                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 326                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 327                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 328                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 329                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 330                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 331                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 332                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 333                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         All Staten Island express, limited, local and select buses are running with delays in both directions while we recover from the winter storm.\nAllow additional travel time.
#> 334                                                                                                                                                                                                                                                                                                                                                                                       Southbound B67 stop on Flatbush Ave at State St is closed\nBuses will stop on Flatbush Ave and Atlantic Ave at the B41 stop.\n\nWhat's happening?\nDOT - Stop permanently discontinued\n\nNote: Bus arrival information may not be available/accurate while buses are detoured
#> 335                                                                                                                                                                                                                                                                                                                                                                                       Southbound B67 stop on Flatbush Ave at State St is closed\nBuses will stop on Flatbush Ave and Atlantic Ave at the B41 stop.\n\nWhat's happening?\nDOT - Stop permanently discontinued\n\nNote: Bus arrival information may not be available/accurate while buses are detoured
#> 336                                                                                                                                                                                                                                                                                                                                                                                                                                                          Northbound B16 stop on Caton Ave at Ocean Pkwy is closed; buses will make a stop on Caton Ave between Ocean Pkwy and E 7th St instead\nSee a map of the boarding change.\n\nWhat's happening?\nConstruction
#> 337                                                                                                                                                                                                                                                                                                                                                                                                                                                          Northbound B16 stop on Caton Ave at Ocean Pkwy is closed; buses will make a stop on Caton Ave between Ocean Pkwy and E 7th St instead\nSee a map of the boarding change.\n\nWhat's happening?\nConstruction
#> 338                                                                                                                                                                                                                                                                                                                                                                                                                      Eastbound M79-SBS stop on W 81st St at Central Park West is temporarily closed; please use the nearby stop on W 81st St at Columbus Ave\nSee a map of this stop change.\n\nWhat's happening?\nElevator construction at the 81 St Subway Station
#> 339                                                                                                                                                                                                                                                                                                                                                                                                                      Eastbound M79-SBS stop on W 81st St at Central Park West is temporarily closed; please use the nearby stop on W 81st St at Columbus Ave\nSee a map of this stop change.\n\nWhat's happening?\nElevator construction at the 81 St Subway Station
#> 340                                                                                                                                                                                                                                                                  St George-bound S48 buses are detoured from Richmond Terrace at Arlington Ave to Arlington Pl at South Ave\nBuses will not stop at Richmond Terrace at Arlington Ave and South Ave at Richmond Terrace.\n\nBuses make stops as requested along Arlington Ave.\nSee map\n\nWhat's happening?\nDOT- road work\n\nNote: Bus arrival information may not be available/accurate while buses are detoured
#> 341                                                                                                                                                                                                                                                                  St George-bound S48 buses are detoured from Richmond Terrace at Arlington Ave to Arlington Pl at South Ave\nBuses will not stop at Richmond Terrace at Arlington Ave and South Ave at Richmond Terrace.\n\nBuses make stops as requested along Arlington Ave.\nSee map\n\nWhat's happening?\nDOT- road work\n\nNote: Bus arrival information may not be available/accurate while buses are detoured
#> 342                                                                                                                                                                                                                                                                                                                                                                                                                                                                         Maspeth-bound Q38 stop on 63rd Dr at Austin St has been temporarily relocated north between Wetherole St and Austin St\nSee a map of the boarding change.\n\nWhat's happening?\nConstruction
#> 343                                                                                                                                                                                                                                                                                                                                                                                                                                                                         Maspeth-bound Q38 stop on 63rd Dr at Austin St has been temporarily relocated north between Wetherole St and Austin St\nSee a map of the boarding change.\n\nWhat's happening?\nConstruction
#> 344                                                                                                                                                                                                             Q42 buses are detoured in both directions due to snow removal at Polhemus Ave/Wren Pl.\nNorthbound buses will not serve stops from 174th St/110th Ave to Liberty Ave/170th St. Stops will be made along Merrick Blvd as requested.\nSouthbound buses will not serve stops from 177th St/106th Ave to Sayres Ave/178th St. Stops will be made along 180th St as requested. \nNote: Bus arrival information may not be available while buses are detoured.
#> 345                                                                                                                                                                                                             Q42 buses are detoured in both directions due to snow removal at Polhemus Ave/Wren Pl.\nNorthbound buses will not serve stops from 174th St/110th Ave to Liberty Ave/170th St. Stops will be made along Merrick Blvd as requested.\nSouthbound buses will not serve stops from 177th St/106th Ave to Sayres Ave/178th St. Stops will be made along 180th St as requested. \nNote: Bus arrival information may not be available while buses are detoured.
#> 346                                                                                                                                                                                                                                                                                                                                                                                                                                           B63 buses may experience delays on 5th Ave between 64th St and 65th St and will wait out any temporary closures\nPlease allow additional travel time.\n\nWhat's happening?\nDOT - Lane Shift/5th Avenue Bridge/65th Street
#> 347                                                                                                                                                                                                                                                                                                                                                                                                                                           B63 buses may experience delays on 5th Ave between 64th St and 65th St and will wait out any temporary closures\nPlease allow additional travel time.\n\nWhat's happening?\nDOT - Lane Shift/5th Avenue Bridge/65th Street
#> 348                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 Northbound M15 stop on 1st Ave at E 77th St is being bypassed\nPlease use the stops on 1st Ave at E 75th St or E 81st St instead.\n\nWhat's happening?\nConstruction
#> 349                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 Northbound M15 stop on 1st Ave at E 77th St is being bypassed\nPlease use the stops on 1st Ave at E 75th St or E 81st St instead.\n\nWhat's happening?\nConstruction
#> 350                                                                                                                                                                                                                                                                                         Bx8 buses are detoured in both directions due to snow removal  at Clarence Ave/Phillip Ave.\nBuses will not serve stops between Clarence Ave/Randall Ave and Layton Ave/Vincent Ave.\nWhile detoured, stops will be made along Throngs Neck Expy Service Rd. in both directions as requested. \nNote: Bus arrival information may not be available while buses are detoured.
#> 351                                                                                                                                                                                                                                                                                         Bx8 buses are detoured in both directions due to snow removal  at Clarence Ave/Phillip Ave.\nBuses will not serve stops between Clarence Ave/Randall Ave and Layton Ave/Vincent Ave.\nWhile detoured, stops will be made along Throngs Neck Expy Service Rd. in both directions as requested. \nNote: Bus arrival information may not be available while buses are detoured.
#> 352                                                                                                                                                                                                                                                                                B69 northbound stop on Vanderbilt Ave at Sterling Pl and southbound stop on Vanderbilt Ave at Park Pl is being bypassed\nFor northbound service, use the stops on Vanderbilt Ave at Plaza St East or Prospect Pl.\n\nFor southbound service, use the stops on Vanderbilt Ave at St Marks Ave or Plaza St East.\n\nSee a map of these stop changes.\n\nWhat's happening?\nConstruction
#> 353                                                                                                                                                                                                                                                                                B69 northbound stop on Vanderbilt Ave at Sterling Pl and southbound stop on Vanderbilt Ave at Park Pl is being bypassed\nFor northbound service, use the stops on Vanderbilt Ave at Plaza St East or Prospect Pl.\n\nFor southbound service, use the stops on Vanderbilt Ave at St Marks Ave or Plaza St East.\n\nSee a map of these stop changes.\n\nWhat's happening?\nConstruction
#> 354                                                                                                                                                                                                                                                                                                              Southbound B82 stop on and Mermaid Ave at W 17th St is closed\nFor service, use the stops on W 17th St at Neptune Ave or Mermaid Ave at Stillwell Ave.\nBuses also make requested stops along Neptune Ave and Stillwell Ave.\n\nWhat's happening?\nConstruction\n\nNote: Bus arrival information may not be available/accurate while buses are detoured
#> 355                                                                                                                                                                                                                                                                                                              Southbound B82 stop on and Mermaid Ave at W 17th St is closed\nFor service, use the stops on W 17th St at Neptune Ave or Mermaid Ave at Stillwell Ave.\nBuses also make requested stops along Neptune Ave and Stillwell Ave.\n\nWhat's happening?\nConstruction\n\nNote: Bus arrival information may not be available/accurate while buses are detoured
#> 356                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   You may wait longer for these buses:\nS54, S74, S78\nWe're running as much service as we can with the operators we have available.
#> 357                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   You may wait longer for these buses:\nS54, S74, S78\nWe're running as much service as we can with the operators we have available.
#> 358                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   You may wait longer for these buses:\nS54, S74, S78\nWe're running as much service as we can with the operators we have available.
#> 359                                                                                                                                                                                                                                                                                                                                                                                                                                         Eastbound B65 stop on Dean St at 4th Ave is closed\nPlease use the stop on Dean St at 3rd Ave.\n\nWhat's happening?\nNYC DDC project\n\nNote: Bus arrival information may not be available/accurate while buses are detoured
#> 360                                                                                                                                                                                                                                                                                                                                                                                                                                         Eastbound B65 stop on Dean St at 4th Ave is closed\nPlease use the stop on Dean St at 3rd Ave.\n\nWhat's happening?\nNYC DDC project\n\nNote: Bus arrival information may not be available/accurate while buses are detoured
#> 361                                                                                                                                                                                               SIM33 and SIM34 buses will be detoured in both directions at South Ave/Arlington Place.\nEastbound stop missed:\nSouth Ave/Richmond Terrace\nUse the South Ave/Arlington Place stop instead.\n\nWestbound stop missed:\nSouth Ave/Arlington Place\nUse the Richmond Terrace/South Ave stop instead.\n\nBuses will run along Arlington Ave.\n\nWhat's Happening?\nUtility work\n\nNOTE: Bus arrival information may not be accurate/available while buses are detoured.
#> 362                                                                                                                                                                                               SIM33 and SIM34 buses will be detoured in both directions at South Ave/Arlington Place.\nEastbound stop missed:\nSouth Ave/Richmond Terrace\nUse the South Ave/Arlington Place stop instead.\n\nWestbound stop missed:\nSouth Ave/Arlington Place\nUse the Richmond Terrace/South Ave stop instead.\n\nBuses will run along Arlington Ave.\n\nWhat's Happening?\nUtility work\n\nNOTE: Bus arrival information may not be accurate/available while buses are detoured.
#> 363                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         Q4, Q5, Q84, Q85, Q86, Q87, Q89 and QM63 buses are running with delays due to utility work on Merrick Blvd between 110th Ave and Linden Blvd.\nAllow additional travel time.
#> 364                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         Q4, Q5, Q84, Q85, Q86, Q87, Q89 and QM63 buses are running with delays due to utility work on Merrick Blvd between 110th Ave and Linden Blvd.\nAllow additional travel time.
#> 365                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         Q4, Q5, Q84, Q85, Q86, Q87, Q89 and QM63 buses are running with delays due to utility work on Merrick Blvd between 110th Ave and Linden Blvd.\nAllow additional travel time.
#> 366                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         Q4, Q5, Q84, Q85, Q86, Q87, Q89 and QM63 buses are running with delays due to utility work on Merrick Blvd between 110th Ave and Linden Blvd.\nAllow additional travel time.
#> 367                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         Q4, Q5, Q84, Q85, Q86, Q87, Q89 and QM63 buses are running with delays due to utility work on Merrick Blvd between 110th Ave and Linden Blvd.\nAllow additional travel time.
#> 368                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         Q4, Q5, Q84, Q85, Q86, Q87, Q89 and QM63 buses are running with delays due to utility work on Merrick Blvd between 110th Ave and Linden Blvd.\nAllow additional travel time.
#> 369                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         Q4, Q5, Q84, Q85, Q86, Q87, Q89 and QM63 buses are running with delays due to utility work on Merrick Blvd between 110th Ave and Linden Blvd.\nAllow additional travel time.
#> 370 S40 buses are detoured in both directions between South Ave at Arlington Place and Richmond Terrace at Arlington Ave.\nFor eastbound service, use the stops on South Ave at Brabant St or Richmond Terrace at South Ave.\nFor westbound service, use the stops on Richmond Terrace at Grandview Ave or South Ave at Arlington Place.\nSee map\n\nBuses operate via Richmond Terrace, Arlington Ave and Arlington Place.\n\nEastbound stop missed:\nSouth Ave at Arlington Place\n\nWestbound stop missed:\nSouth Ave at Richmond Terrace\n\nWhat's Happening?\nUtility Work\n\nNOTE: Bus arrival information may not be accurate/available while buses are detoured.
#> 371 S40 buses are detoured in both directions between South Ave at Arlington Place and Richmond Terrace at Arlington Ave.\nFor eastbound service, use the stops on South Ave at Brabant St or Richmond Terrace at South Ave.\nFor westbound service, use the stops on Richmond Terrace at Grandview Ave or South Ave at Arlington Place.\nSee map\n\nBuses operate via Richmond Terrace, Arlington Ave and Arlington Place.\n\nEastbound stop missed:\nSouth Ave at Arlington Place\n\nWestbound stop missed:\nSouth Ave at Richmond Terrace\n\nWhat's Happening?\nUtility Work\n\nNOTE: Bus arrival information may not be accurate/available while buses are detoured.
#> 372                                                                                                                                                                                                                                                                                                                                                                                                                      Northbound SIM26 drop-off on Madison Ave at 48th St will be made at the BxM4/BxM11 stop on Madison Ave at 47th St instead\nWhat's happening?\nConstruction\n\nNote: Real-time tracking on BusTime may be inaccurate in the service change area.
#> 373                                                                                                                                                                                                                                                                                                                                                                                                                      Northbound SIM26 drop-off on Madison Ave at 48th St will be made at the BxM4/BxM11 stop on Madison Ave at 47th St instead\nWhat's happening?\nConstruction\n\nNote: Real-time tracking on BusTime may be inaccurate in the service change area.
#> 374                                                                                                                                                                                                                                                                                                                                                                                                                                                           Northbound M20 and M55 stop on State St at Bridge St is closed; use the temporary stop on State St before the intersection instead\nSee a map of the new stop location.\n\nWhat's happening?\nConstruction
#> 375                                                                                                                                                                                                                                                                                                                                                                                                                                                           Northbound M20 and M55 stop on State St at Bridge St is closed; use the temporary stop on State St before the intersection instead\nSee a map of the new stop location.\n\nWhat's happening?\nConstruction
#> 376                                                                                                                                                                                                                                                                                                                                         Eastbound M35 stop on Rivers Edge Rd at Manhattan Psychiatric Center is closed\nFor service, use the stop on Main Roadway at Manhattan Psych Center instead.\n\nSee a map of this stop change.\n\nWhat's happening?\nDemolition Work\n\nNote: Bus arrival information may not be available/accurate while buses are detoured
#> 377                                                                                                                                                                                                                                                                                                                                         Eastbound M35 stop on Rivers Edge Rd at Manhattan Psychiatric Center is closed\nFor service, use the stop on Main Roadway at Manhattan Psych Center instead.\n\nSee a map of this stop change.\n\nWhat's happening?\nDemolition Work\n\nNote: Bus arrival information may not be available/accurate while buses are detoured
#> 378                                                                                                                                                                                                                                                                                                                                                                                                                                            Staten Island-bound SIM5, SIM15 and SIM35 stop on State St at Bridge St is closed; use the temporary stop on State St closer to Bridge St instead\nSee a map of the new stop location.\n\nWhat's happening?\nConstruction
#> 379                                                                                                                                                                                                                                                                                                                                                                                                                                            Staten Island-bound SIM5, SIM15 and SIM35 stop on State St at Bridge St is closed; use the temporary stop on State St closer to Bridge St instead\nSee a map of the new stop location.\n\nWhat's happening?\nConstruction
#> 380                                                                                                                                                                                                                                                                                                                                                                                                                                            Staten Island-bound SIM5, SIM15 and SIM35 stop on State St at Bridge St is closed; use the temporary stop on State St closer to Bridge St instead\nSee a map of the new stop location.\n\nWhat's happening?\nConstruction
#> 381                                                                                                  SIM4X/SIM8X trips will be discontinued, more trips coming to SIM4/SIM8\nThings to know:\nSIM4X and SIM8X are special express trips of the SIM4 and SIM8 routes.\nRegular SIM4 and SIM8 trips will not be reduced.\nSome of the express SIM4X and SIM8X trips will be converted to full route SIM4 and SIM8 trips.\nThese changes are part of our regular process of reviewing bus schedules and ridership demand. Discontinuing the SIM4X and SIM8X and converting some of these trips into full-route SIM4 and SIM8 trips will better use underutilized resources.
#> 382                                                                                                  SIM4X/SIM8X trips will be discontinued, more trips coming to SIM4/SIM8\nThings to know:\nSIM4X and SIM8X are special express trips of the SIM4 and SIM8 routes.\nRegular SIM4 and SIM8 trips will not be reduced.\nSome of the express SIM4X and SIM8X trips will be converted to full route SIM4 and SIM8 trips.\nThese changes are part of our regular process of reviewing bus schedules and ridership demand. Discontinuing the SIM4X and SIM8X and converting some of these trips into full-route SIM4 and SIM8 trips will better use underutilized resources.
#> 383                                                                                                                                                                                                                                                                                                                                                               Southbound B20 stop on Fairview Ave at Putnam Ave is discontinued; northbound buses are now stopping at the existing southbound stop on Putnam Ave at Forest Ave\nBuses operate via Putnam Ave, Forest Ave and 67th Ave.\nSee a map of the boarding change.\n\nWhat's happening?\nTraffic improvements
#> 384                                                                                                                                                                                                                                                                                                                                                               Southbound B20 stop on Fairview Ave at Putnam Ave is discontinued; northbound buses are now stopping at the existing southbound stop on Putnam Ave at Forest Ave\nBuses operate via Putnam Ave, Forest Ave and 67th Ave.\nSee a map of the boarding change.\n\nWhat's happening?\nTraffic improvements
#> 385                                                                                                           Bx39 and BxM11 buses are detoured in both directions due to FDNY activity at White Plains Rd/Adee Ave.\nBx39 southbound buses will not serve stops on White Plains Rd at Arnow Ave and Allerton Ave.\nBx39 northbound buses will not serve stops on White Plains Rd at Arnow Ave and Burke Ave.\nBxM11 southbound buses will not serve the White Plains Rd/Allerton Ave bus stop.\nBxM11 northbound buses will not serve the White Plains Rd/Burke Ave bus stops.\nWhile detoured, stops will be made along Bronxwood Ave in both directions as requested.
#> 386                                                                                                           Bx39 and BxM11 buses are detoured in both directions due to FDNY activity at White Plains Rd/Adee Ave.\nBx39 southbound buses will not serve stops on White Plains Rd at Arnow Ave and Allerton Ave.\nBx39 northbound buses will not serve stops on White Plains Rd at Arnow Ave and Burke Ave.\nBxM11 southbound buses will not serve the White Plains Rd/Allerton Ave bus stop.\nBxM11 northbound buses will not serve the White Plains Rd/Burke Ave bus stops.\nWhile detoured, stops will be made along Bronxwood Ave in both directions as requested.
#> 387                                                                                                                                                                                                                                                                                                                                                                                                                             Southbound M98 are detouring in Harlem; you will experience a few extra turns, but no stops are missed\nBuses operate via 5th Ave from W 142nd St to W 125th St. Please allow additional travel time.\n\nWhat's happening?\nConstruction
#> 388                                                                                                                                                                                                                                                                                                                                                                                                                             Southbound M98 are detouring in Harlem; you will experience a few extra turns, but no stops are missed\nBuses operate via 5th Ave from W 142nd St to W 125th St. Please allow additional travel time.\n\nWhat's happening?\nConstruction
#> 389                                                                                                                                                                                                                                                                    Northbound B60 buses are detoured due to utility work on Meserole St between Union Ave and Marcy Ave.\nNorthbound B60 buses will not serve stops from S 4th St/Hewes St to S 4th St/Rodney St.\n\nWhile detoured, northbound B60 buses will make stops as requested along Union Ave and Borinquen Pl.\n\nNote: Bus arrival information may not be accurate or available while buses are detoured.
#> 390                                                                                                                                                                                                                                                                    Northbound B60 buses are detoured due to utility work on Meserole St between Union Ave and Marcy Ave.\nNorthbound B60 buses will not serve stops from S 4th St/Hewes St to S 4th St/Rodney St.\n\nWhile detoured, northbound B60 buses will make stops as requested along Union Ave and Borinquen Pl.\n\nNote: Bus arrival information may not be accurate or available while buses are detoured.
#> 391                                                                                                                                                                                                                                                                                                                                                                                                                                                        Southbound M15, M15-SBS, M20, and M55 buses are detoured due to snow removal operations at Broad St/South St.\nWhile detoured, you may notice a few extra turns and some delays, but no stops will be missed.
#> 392                                                                                                                                                                                                                                                                                                                                                                                                                                                        Southbound M15, M15-SBS, M20, and M55 buses are detoured due to snow removal operations at Broad St/South St.\nWhile detoured, you may notice a few extra turns and some delays, but no stops will be missed.
#> 393                                                                                                                                                                                                                                                                                                                                                                                                                                                        Southbound M15, M15-SBS, M20, and M55 buses are detoured due to snow removal operations at Broad St/South St.\nWhile detoured, you may notice a few extra turns and some delays, but no stops will be missed.
#> 394                                                                                                                                                                                                                                                                                                                                                                                                                                                        Southbound M15, M15-SBS, M20, and M55 buses are detoured due to snow removal operations at Broad St/South St.\nWhile detoured, you may notice a few extra turns and some delays, but no stops will be missed.
#> 395                                                                                                                                                                                                                                                                                        Northbound Q61 stop on Linden Pl at 35th Ave is closed; buses make a stop on Farrington St at 35th Ave instead\nAlso, consider the northbound stops on Main St at Northern Blvd or Linden Pl at 32nd Ave.\n\nSee a map of the detour.\n\nWhat's happening?\nNYC DEP sewer maintenance\n\nNote: Bus arrival information may not be available/accurate while buses are detoured
#> 396                                                                                                                                                                                                                                                                                        Northbound Q61 stop on Linden Pl at 35th Ave is closed; buses make a stop on Farrington St at 35th Ave instead\nAlso, consider the northbound stops on Main St at Northern Blvd or Linden Pl at 32nd Ave.\n\nSee a map of the detour.\n\nWhat's happening?\nNYC DEP sewer maintenance\n\nNote: Bus arrival information may not be available/accurate while buses are detoured
#> 397                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       Northbound M15 and M15-SBS stop on 1st Ave at 79th has been restored\nSee a map of this stop change.\n\nWhat happened?\nConstruction completed
#> 398                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       Northbound M15 and M15-SBS stop on 1st Ave at 79th has been restored\nSee a map of this stop change.\n\nWhat happened?\nConstruction completed
#> 399                                                                                                                                                                                    B12 buses are detoured between Pitkin Ave at Rockaway Ave and Eastern Parkway main roadway at Rockaway Ave - no stops will be missed\nEastbound buses operate via Pitkin Ave, Rockaway Ave, Eastern Parkway and Dean St.\nWestbound buses operate via Pacific St, Eastern Pkwy, Rockaway Ave and Pitkin Ave.\nHere is a map of the detoured route.\n\nWhat?s happening?\nDEP-Construction\n\nNote: Bus arrival information may not be available/accurate while buses are detoured
#> 400                                                                                                                                                                                    B12 buses are detoured between Pitkin Ave at Rockaway Ave and Eastern Parkway main roadway at Rockaway Ave - no stops will be missed\nEastbound buses operate via Pitkin Ave, Rockaway Ave, Eastern Parkway and Dean St.\nWestbound buses operate via Pacific St, Eastern Pkwy, Rockaway Ave and Pitkin Ave.\nHere is a map of the detoured route.\n\nWhat?s happening?\nDEP-Construction\n\nNote: Bus arrival information may not be available/accurate while buses are detoured
#> 401                                                                                                                                               Northbound BxM2, BxM3, BxM4, and BxM11 buses are bypassing the stop on Madison Ave at 99th St\nCustomers may temporarily board or exit at the M1 stop on Madison Ave at E 102nd St.\n\nFor BxM2, BxM3, and BxM4 service, use the stops on Madison Ave at E 84th St or 123rd St.\n\nFor BxM11 service, use the stop on Madison Ave at E 84th St. See a map of this stop change.\n\nWhat's happening?\nRoad Blockage at bus stop\n\nNote: Bus arrival information may not be available/accurate while buses are detoured
#> 402                                                                                                                                               Northbound BxM2, BxM3, BxM4, and BxM11 buses are bypassing the stop on Madison Ave at 99th St\nCustomers may temporarily board or exit at the M1 stop on Madison Ave at E 102nd St.\n\nFor BxM2, BxM3, and BxM4 service, use the stops on Madison Ave at E 84th St or 123rd St.\n\nFor BxM11 service, use the stop on Madison Ave at E 84th St. See a map of this stop change.\n\nWhat's happening?\nRoad Blockage at bus stop\n\nNote: Bus arrival information may not be available/accurate while buses are detoured
#> 403                                                                                                                                               Northbound BxM2, BxM3, BxM4, and BxM11 buses are bypassing the stop on Madison Ave at 99th St\nCustomers may temporarily board or exit at the M1 stop on Madison Ave at E 102nd St.\n\nFor BxM2, BxM3, and BxM4 service, use the stops on Madison Ave at E 84th St or 123rd St.\n\nFor BxM11 service, use the stop on Madison Ave at E 84th St. See a map of this stop change.\n\nWhat's happening?\nRoad Blockage at bus stop\n\nNote: Bus arrival information may not be available/accurate while buses are detoured
#> 404                                                                                                                                               Northbound BxM2, BxM3, BxM4, and BxM11 buses are bypassing the stop on Madison Ave at 99th St\nCustomers may temporarily board or exit at the M1 stop on Madison Ave at E 102nd St.\n\nFor BxM2, BxM3, and BxM4 service, use the stops on Madison Ave at E 84th St or 123rd St.\n\nFor BxM11 service, use the stop on Madison Ave at E 84th St. See a map of this stop change.\n\nWhat's happening?\nRoad Blockage at bus stop\n\nNote: Bus arrival information may not be available/accurate while buses are detoured
#> 405                                                                                                                                                                                                                                                                                                                                             Northbound BxM3 and BxM4 stop on Madison Ave at E 46th St is closed, please use the M3 bus stop on Madison Ave at E 45th St instead\nHere is a map of the this stop change.\n\nWhat's happening?\nConstruction - 383 Madison Ave\n\nNote: Bus arrival information may not be available/accurate while buses are detoured
#> 406                                                                                                                                                                                                                                                                                                                                             Northbound BxM3 and BxM4 stop on Madison Ave at E 46th St is closed, please use the M3 bus stop on Madison Ave at E 45th St instead\nHere is a map of the this stop change.\n\nWhat's happening?\nConstruction - 383 Madison Ave\n\nNote: Bus arrival information may not be available/accurate while buses are detoured
#> 407                                                                                                                                                                                                                                                                                                      Southbound Q47 buses are detoured due to an illegally parked vehicle at 25th Ave/78th St.\nSouthbound Q47 buses will not serve the 78th St/30th Ave and 31st Ave/77th St stops.\n\nWhile detoured, southbound Q47 buses will make stops as requested along 75th St.\n\nNote: Bus arrival information may not be accurate or available while buses are detoured.
#> 408                                                                                                                                                                                                                                                                                                      Southbound Q47 buses are detoured due to an illegally parked vehicle at 25th Ave/78th St.\nSouthbound Q47 buses will not serve the 78th St/30th Ave and 31st Ave/77th St stops.\n\nWhile detoured, southbound Q47 buses will make stops as requested along 75th St.\n\nNote: Bus arrival information may not be accurate or available while buses are detoured.
#> 409                                                                                                                                                                                                                                                                                                                                                                                                                                  Eastbound Q32 stop on Roosevelt Ave at 61st St has been temporarily relocated to Roosevelt Ave between 60th St and 61st St, before the intersection\nHere is a map of the temporary stop change.\n\nWhat's happening?\nConstruction
#> 410                                                                                                                                                                                                                                                                                                                                                                                                                                  Eastbound Q32 stop on Roosevelt Ave at 61st St has been temporarily relocated to Roosevelt Ave between 60th St and 61st St, before the intersection\nHere is a map of the temporary stop change.\n\nWhat's happening?\nConstruction
#> 411                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         The 8:30 AM QM6 bus trip scheduled to depart North Shore Towers will not run.\nWe're running as much service as we can with the operators we have available.
#> 412                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         The 8:30 AM QM6 bus trip scheduled to depart North Shore Towers will not run.\nWe're running as much service as we can with the operators we have available.
#> 413                                                                                                                                                                                                                                           Q18 buses are detoured in both directions due to icy roads on 53rd Ave between 69th St and 65th Pl.\nEastbound buses will not serve stops on 69th St from Calamus Ave to 52nd Dr.\nWestbound buses will not serve stops on 69th St from 53rd Ave to Maurice Ave.\nWhile detoured, stops will be made as requested along 65th Pl.\nNote: Bus arrival information may not be accurate or available while buses are detoured.
#> 414                                                                                                                                                                                                                                           Q18 buses are detoured in both directions due to icy roads on 53rd Ave between 69th St and 65th Pl.\nEastbound buses will not serve stops on 69th St from Calamus Ave to 52nd Dr.\nWestbound buses will not serve stops on 69th St from 53rd Ave to Maurice Ave.\nWhile detoured, stops will be made as requested along 65th Pl.\nNote: Bus arrival information may not be accurate or available while buses are detoured.
#> 415                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              Westbound Q7 stop on Jamaica Ave at Eldert Ln is closed; customers will be dropped off on Eldert Ln at 87th Ave instead\nWhat's happening\nConstruction
#> 416                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              Westbound Q7 stop on Jamaica Ave at Eldert Ln is closed; customers will be dropped off on Eldert Ln at 87th Ave instead\nWhat's happening\nConstruction
#> 417                                                                                                                                                                                                                                                                                                                                                                                    Southbound BxM11 buses are detouring from E 177th St at Bronx River Parkway to Bruckner Blvd at Bruckner Expy - no stops will be missed\nPlease allow additional travel time.\n\nSee a map of the detour\n\nWhat's happening?\nBronx River Southbound Ramp - Construction Project
#> 418                                                                                                                                                                                                                                                                                                                                                                                    Southbound BxM11 buses are detouring from E 177th St at Bronx River Parkway to Bruckner Blvd at Bruckner Expy - no stops will be missed\nPlease allow additional travel time.\n\nSee a map of the detour\n\nWhat's happening?\nBronx River Southbound Ramp - Construction Project
#> 419                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    The 8:30 AM QM5 bus trip scheduled to depart Union Turnpike/260th St will not run.\nWe're running as much service as we can with the operators we have available.
#> 420                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    The 8:30 AM QM5 bus trip scheduled to depart Union Turnpike/260th St will not run.\nWe're running as much service as we can with the operators we have available.
#> 421                                                                                                                                                                                                                                                                                                                                                                   Southbound Q52-SBS and Q53-SBS stop on Cross Bay Blvd at Liberty Ave has been temporarily relocated down the block before 107th Ave\nWhat's happening?\nRockaway Blvd Subway Station Accessibility Upgrades\n\nPlan your trip at mta.info or use the MTA app (download the app for iOS or Android)
#> 422                                                                                                                                                                                                                                                                                                                                                                   Southbound Q52-SBS and Q53-SBS stop on Cross Bay Blvd at Liberty Ave has been temporarily relocated down the block before 107th Ave\nWhat's happening?\nRockaway Blvd Subway Station Accessibility Upgrades\n\nPlan your trip at mta.info or use the MTA app (download the app for iOS or Android)
#> 423                                                                                                                                                                                                                                                                                         Northbound Q25 stop on Linden Pl at 35th Ave is closed; buses make a stop on Farrington St at 35th Ave instead\nAlso, consider the northbound stops on Main St at Northern Blvd or Linden Pl at 31st Rd.\n\nSee a map of the detour.\n\nWhat's happening?\nNYC DEP sewer maintenance\n\nNote: Bus arrival information may not be available/accurate while buses are detoured
#> 424                                                                                                                                                                                                                                                                                         Northbound Q25 stop on Linden Pl at 35th Ave is closed; buses make a stop on Farrington St at 35th Ave instead\nAlso, consider the northbound stops on Main St at Northern Blvd or Linden Pl at 31st Rd.\n\nSee a map of the detour.\n\nWhat's happening?\nNYC DEP sewer maintenance\n\nNote: Bus arrival information may not be available/accurate while buses are detoured
#> 425                                                                                                                                                                                                        Q47 buses are detoured in both directions because of a road blockage at Calamus Ave/79th St.\nWhile detoured, buses will not service stops along Calamus Ave at 74th St or 71st St in either direction.\n\nSouthbound buses will also not service the 79th St/Grand Ave stop.\n\nStops will instead be made along Grand Ave and along 69th St in both directions.\n\nNote: Bus arrival information may be inaccurate or unavailable while buses are detoured.
#> 426                                                                                                                                                                                                        Q47 buses are detoured in both directions because of a road blockage at Calamus Ave/79th St.\nWhile detoured, buses will not service stops along Calamus Ave at 74th St or 71st St in either direction.\n\nSouthbound buses will also not service the 79th St/Grand Ave stop.\n\nStops will instead be made along Grand Ave and along 69th St in both directions.\n\nNote: Bus arrival information may be inaccurate or unavailable while buses are detoured.
#> 427                                                                                                                                                                                                                                                                                                                                                     Bronx-bound Q50 buses may experience delays in Flushing, but no stops are missed; buses use Farrington St from 35th Ave to 31st Rd\nPlease allow additional travel time.\n\nWhat's happening?\nNYC DEP sewer maintenance\n\nNote: Bus arrival information may not be available/accurate while buses are detoured
#> 428                                                                                                                                                                                                                                                                                                                                                     Bronx-bound Q50 buses may experience delays in Flushing, but no stops are missed; buses use Farrington St from 35th Ave to 31st Rd\nPlease allow additional travel time.\n\nWhat's happening?\nNYC DEP sewer maintenance\n\nNote: Bus arrival information may not be available/accurate while buses are detoured
#> 429                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   Eastbound Q32 and Q60 stop on Queens Blvd at 45th St is closed; please use the stops on Queens Blvd at 41st St or 46th St instead\nWhat's happening?\nConstruction
#> 430                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   Eastbound Q32 and Q60 stop on Queens Blvd at 45th St is closed; please use the stops on Queens Blvd at 41st St or 46th St instead\nWhat's happening?\nConstruction
#>     tts_header_text tts_description_text severity_level
#> 1              <NA>                 <NA>           <NA>
#> 2              <NA>                 <NA>           <NA>
#> 3              <NA>                 <NA>           <NA>
#> 4              <NA>                 <NA>           <NA>
#> 5              <NA>                 <NA>           <NA>
#> 6              <NA>                 <NA>           <NA>
#> 7              <NA>                 <NA>           <NA>
#> 8              <NA>                 <NA>           <NA>
#> 9              <NA>                 <NA>           <NA>
#> 10             <NA>                 <NA>           <NA>
#> 11             <NA>                 <NA>           <NA>
#> 12             <NA>                 <NA>           <NA>
#> 13             <NA>                 <NA>           <NA>
#> 14             <NA>                 <NA>           <NA>
#> 15             <NA>                 <NA>           <NA>
#> 16             <NA>                 <NA>           <NA>
#> 17             <NA>                 <NA>           <NA>
#> 18             <NA>                 <NA>           <NA>
#> 19             <NA>                 <NA>           <NA>
#> 20             <NA>                 <NA>           <NA>
#> 21             <NA>                 <NA>           <NA>
#> 22             <NA>                 <NA>           <NA>
#> 23             <NA>                 <NA>           <NA>
#> 24             <NA>                 <NA>           <NA>
#> 25             <NA>                 <NA>           <NA>
#> 26             <NA>                 <NA>           <NA>
#> 27             <NA>                 <NA>           <NA>
#> 28             <NA>                 <NA>           <NA>
#> 29             <NA>                 <NA>           <NA>
#> 30             <NA>                 <NA>           <NA>
#> 31             <NA>                 <NA>           <NA>
#> 32             <NA>                 <NA>           <NA>
#> 33             <NA>                 <NA>           <NA>
#> 34             <NA>                 <NA>           <NA>
#> 35             <NA>                 <NA>           <NA>
#> 36             <NA>                 <NA>           <NA>
#> 37             <NA>                 <NA>           <NA>
#> 38             <NA>                 <NA>           <NA>
#> 39             <NA>                 <NA>           <NA>
#> 40             <NA>                 <NA>           <NA>
#> 41             <NA>                 <NA>           <NA>
#> 42             <NA>                 <NA>           <NA>
#> 43             <NA>                 <NA>           <NA>
#> 44             <NA>                 <NA>           <NA>
#> 45             <NA>                 <NA>           <NA>
#> 46             <NA>                 <NA>           <NA>
#> 47             <NA>                 <NA>           <NA>
#> 48             <NA>                 <NA>           <NA>
#> 49             <NA>                 <NA>           <NA>
#> 50             <NA>                 <NA>           <NA>
#> 51             <NA>                 <NA>           <NA>
#> 52             <NA>                 <NA>           <NA>
#> 53             <NA>                 <NA>           <NA>
#> 54             <NA>                 <NA>           <NA>
#> 55             <NA>                 <NA>           <NA>
#> 56             <NA>                 <NA>           <NA>
#> 57             <NA>                 <NA>           <NA>
#> 58             <NA>                 <NA>           <NA>
#> 59             <NA>                 <NA>           <NA>
#> 60             <NA>                 <NA>           <NA>
#> 61             <NA>                 <NA>           <NA>
#> 62             <NA>                 <NA>           <NA>
#> 63             <NA>                 <NA>           <NA>
#> 64             <NA>                 <NA>           <NA>
#> 65             <NA>                 <NA>           <NA>
#> 66             <NA>                 <NA>           <NA>
#> 67             <NA>                 <NA>           <NA>
#> 68             <NA>                 <NA>           <NA>
#> 69             <NA>                 <NA>           <NA>
#> 70             <NA>                 <NA>           <NA>
#> 71             <NA>                 <NA>           <NA>
#> 72             <NA>                 <NA>           <NA>
#> 73             <NA>                 <NA>           <NA>
#> 74             <NA>                 <NA>           <NA>
#> 75             <NA>                 <NA>           <NA>
#> 76             <NA>                 <NA>           <NA>
#> 77             <NA>                 <NA>           <NA>
#> 78             <NA>                 <NA>           <NA>
#> 79             <NA>                 <NA>           <NA>
#> 80             <NA>                 <NA>           <NA>
#> 81             <NA>                 <NA>           <NA>
#> 82             <NA>                 <NA>           <NA>
#> 83             <NA>                 <NA>           <NA>
#> 84             <NA>                 <NA>           <NA>
#> 85             <NA>                 <NA>           <NA>
#> 86             <NA>                 <NA>           <NA>
#> 87             <NA>                 <NA>           <NA>
#> 88             <NA>                 <NA>           <NA>
#> 89             <NA>                 <NA>           <NA>
#> 90             <NA>                 <NA>           <NA>
#> 91             <NA>                 <NA>           <NA>
#> 92             <NA>                 <NA>           <NA>
#> 93             <NA>                 <NA>           <NA>
#> 94             <NA>                 <NA>           <NA>
#> 95             <NA>                 <NA>           <NA>
#> 96             <NA>                 <NA>           <NA>
#> 97             <NA>                 <NA>           <NA>
#> 98             <NA>                 <NA>           <NA>
#> 99             <NA>                 <NA>           <NA>
#> 100            <NA>                 <NA>           <NA>
#> 101            <NA>                 <NA>           <NA>
#> 102            <NA>                 <NA>           <NA>
#> 103            <NA>                 <NA>           <NA>
#> 104            <NA>                 <NA>           <NA>
#> 105            <NA>                 <NA>           <NA>
#> 106            <NA>                 <NA>           <NA>
#> 107            <NA>                 <NA>           <NA>
#> 108            <NA>                 <NA>           <NA>
#> 109            <NA>                 <NA>           <NA>
#> 110            <NA>                 <NA>           <NA>
#> 111            <NA>                 <NA>           <NA>
#> 112            <NA>                 <NA>           <NA>
#> 113            <NA>                 <NA>           <NA>
#> 114            <NA>                 <NA>           <NA>
#> 115            <NA>                 <NA>           <NA>
#> 116            <NA>                 <NA>           <NA>
#> 117            <NA>                 <NA>           <NA>
#> 118            <NA>                 <NA>           <NA>
#> 119            <NA>                 <NA>           <NA>
#> 120            <NA>                 <NA>           <NA>
#> 121            <NA>                 <NA>           <NA>
#> 122            <NA>                 <NA>           <NA>
#> 123            <NA>                 <NA>           <NA>
#> 124            <NA>                 <NA>           <NA>
#> 125            <NA>                 <NA>           <NA>
#> 126            <NA>                 <NA>           <NA>
#> 127            <NA>                 <NA>           <NA>
#> 128            <NA>                 <NA>           <NA>
#> 129            <NA>                 <NA>           <NA>
#> 130            <NA>                 <NA>           <NA>
#> 131            <NA>                 <NA>           <NA>
#> 132            <NA>                 <NA>           <NA>
#> 133            <NA>                 <NA>           <NA>
#> 134            <NA>                 <NA>           <NA>
#> 135            <NA>                 <NA>           <NA>
#> 136            <NA>                 <NA>           <NA>
#> 137            <NA>                 <NA>           <NA>
#> 138            <NA>                 <NA>           <NA>
#> 139            <NA>                 <NA>           <NA>
#> 140            <NA>                 <NA>           <NA>
#> 141            <NA>                 <NA>           <NA>
#> 142            <NA>                 <NA>           <NA>
#> 143            <NA>                 <NA>           <NA>
#> 144            <NA>                 <NA>           <NA>
#> 145            <NA>                 <NA>           <NA>
#> 146            <NA>                 <NA>           <NA>
#> 147            <NA>                 <NA>           <NA>
#> 148            <NA>                 <NA>           <NA>
#> 149            <NA>                 <NA>           <NA>
#> 150            <NA>                 <NA>           <NA>
#> 151            <NA>                 <NA>           <NA>
#> 152            <NA>                 <NA>           <NA>
#> 153            <NA>                 <NA>           <NA>
#> 154            <NA>                 <NA>           <NA>
#> 155            <NA>                 <NA>           <NA>
#> 156            <NA>                 <NA>           <NA>
#> 157            <NA>                 <NA>           <NA>
#> 158            <NA>                 <NA>           <NA>
#> 159            <NA>                 <NA>           <NA>
#> 160            <NA>                 <NA>           <NA>
#> 161            <NA>                 <NA>           <NA>
#> 162            <NA>                 <NA>           <NA>
#> 163            <NA>                 <NA>           <NA>
#> 164            <NA>                 <NA>           <NA>
#> 165            <NA>                 <NA>           <NA>
#> 166            <NA>                 <NA>           <NA>
#> 167            <NA>                 <NA>           <NA>
#> 168            <NA>                 <NA>           <NA>
#> 169            <NA>                 <NA>           <NA>
#> 170            <NA>                 <NA>           <NA>
#> 171            <NA>                 <NA>           <NA>
#> 172            <NA>                 <NA>           <NA>
#> 173            <NA>                 <NA>           <NA>
#> 174            <NA>                 <NA>           <NA>
#> 175            <NA>                 <NA>           <NA>
#> 176            <NA>                 <NA>           <NA>
#> 177            <NA>                 <NA>           <NA>
#> 178            <NA>                 <NA>           <NA>
#> 179            <NA>                 <NA>           <NA>
#> 180            <NA>                 <NA>           <NA>
#> 181            <NA>                 <NA>           <NA>
#> 182            <NA>                 <NA>           <NA>
#> 183            <NA>                 <NA>           <NA>
#> 184            <NA>                 <NA>           <NA>
#> 185            <NA>                 <NA>           <NA>
#> 186            <NA>                 <NA>           <NA>
#> 187            <NA>                 <NA>           <NA>
#> 188            <NA>                 <NA>           <NA>
#> 189            <NA>                 <NA>           <NA>
#> 190            <NA>                 <NA>           <NA>
#> 191            <NA>                 <NA>           <NA>
#> 192            <NA>                 <NA>           <NA>
#> 193            <NA>                 <NA>           <NA>
#> 194            <NA>                 <NA>           <NA>
#> 195            <NA>                 <NA>           <NA>
#> 196            <NA>                 <NA>           <NA>
#> 197            <NA>                 <NA>           <NA>
#> 198            <NA>                 <NA>           <NA>
#> 199            <NA>                 <NA>           <NA>
#> 200            <NA>                 <NA>           <NA>
#> 201            <NA>                 <NA>           <NA>
#> 202            <NA>                 <NA>           <NA>
#> 203            <NA>                 <NA>           <NA>
#> 204            <NA>                 <NA>           <NA>
#> 205            <NA>                 <NA>           <NA>
#> 206            <NA>                 <NA>           <NA>
#> 207            <NA>                 <NA>           <NA>
#> 208            <NA>                 <NA>           <NA>
#> 209            <NA>                 <NA>           <NA>
#> 210            <NA>                 <NA>           <NA>
#> 211            <NA>                 <NA>           <NA>
#> 212            <NA>                 <NA>           <NA>
#> 213            <NA>                 <NA>           <NA>
#> 214            <NA>                 <NA>           <NA>
#> 215            <NA>                 <NA>           <NA>
#> 216            <NA>                 <NA>           <NA>
#> 217            <NA>                 <NA>           <NA>
#> 218            <NA>                 <NA>           <NA>
#> 219            <NA>                 <NA>           <NA>
#> 220            <NA>                 <NA>           <NA>
#> 221            <NA>                 <NA>           <NA>
#> 222            <NA>                 <NA>           <NA>
#> 223            <NA>                 <NA>           <NA>
#> 224            <NA>                 <NA>           <NA>
#> 225            <NA>                 <NA>           <NA>
#> 226            <NA>                 <NA>           <NA>
#> 227            <NA>                 <NA>           <NA>
#> 228            <NA>                 <NA>           <NA>
#> 229            <NA>                 <NA>           <NA>
#> 230            <NA>                 <NA>           <NA>
#> 231            <NA>                 <NA>           <NA>
#> 232            <NA>                 <NA>           <NA>
#> 233            <NA>                 <NA>           <NA>
#> 234            <NA>                 <NA>           <NA>
#> 235            <NA>                 <NA>           <NA>
#> 236            <NA>                 <NA>           <NA>
#> 237            <NA>                 <NA>           <NA>
#> 238            <NA>                 <NA>           <NA>
#> 239            <NA>                 <NA>           <NA>
#> 240            <NA>                 <NA>           <NA>
#> 241            <NA>                 <NA>           <NA>
#> 242            <NA>                 <NA>           <NA>
#> 243            <NA>                 <NA>           <NA>
#> 244            <NA>                 <NA>           <NA>
#> 245            <NA>                 <NA>           <NA>
#> 246            <NA>                 <NA>           <NA>
#> 247            <NA>                 <NA>           <NA>
#> 248            <NA>                 <NA>           <NA>
#> 249            <NA>                 <NA>           <NA>
#> 250            <NA>                 <NA>           <NA>
#> 251            <NA>                 <NA>           <NA>
#> 252            <NA>                 <NA>           <NA>
#> 253            <NA>                 <NA>           <NA>
#> 254            <NA>                 <NA>           <NA>
#> 255            <NA>                 <NA>           <NA>
#> 256            <NA>                 <NA>           <NA>
#> 257            <NA>                 <NA>           <NA>
#> 258            <NA>                 <NA>           <NA>
#> 259            <NA>                 <NA>           <NA>
#> 260            <NA>                 <NA>           <NA>
#> 261            <NA>                 <NA>           <NA>
#> 262            <NA>                 <NA>           <NA>
#> 263            <NA>                 <NA>           <NA>
#> 264            <NA>                 <NA>           <NA>
#> 265            <NA>                 <NA>           <NA>
#> 266            <NA>                 <NA>           <NA>
#> 267            <NA>                 <NA>           <NA>
#> 268            <NA>                 <NA>           <NA>
#> 269            <NA>                 <NA>           <NA>
#> 270            <NA>                 <NA>           <NA>
#> 271            <NA>                 <NA>           <NA>
#> 272            <NA>                 <NA>           <NA>
#> 273            <NA>                 <NA>           <NA>
#> 274            <NA>                 <NA>           <NA>
#> 275            <NA>                 <NA>           <NA>
#> 276            <NA>                 <NA>           <NA>
#> 277            <NA>                 <NA>           <NA>
#> 278            <NA>                 <NA>           <NA>
#> 279            <NA>                 <NA>           <NA>
#> 280            <NA>                 <NA>           <NA>
#> 281            <NA>                 <NA>           <NA>
#> 282            <NA>                 <NA>           <NA>
#> 283            <NA>                 <NA>           <NA>
#> 284            <NA>                 <NA>           <NA>
#> 285            <NA>                 <NA>           <NA>
#> 286            <NA>                 <NA>           <NA>
#> 287            <NA>                 <NA>           <NA>
#> 288            <NA>                 <NA>           <NA>
#> 289            <NA>                 <NA>           <NA>
#> 290            <NA>                 <NA>           <NA>
#> 291            <NA>                 <NA>           <NA>
#> 292            <NA>                 <NA>           <NA>
#> 293            <NA>                 <NA>           <NA>
#> 294            <NA>                 <NA>           <NA>
#> 295            <NA>                 <NA>           <NA>
#> 296            <NA>                 <NA>           <NA>
#> 297            <NA>                 <NA>           <NA>
#> 298            <NA>                 <NA>           <NA>
#> 299            <NA>                 <NA>           <NA>
#> 300            <NA>                 <NA>           <NA>
#> 301            <NA>                 <NA>           <NA>
#> 302            <NA>                 <NA>           <NA>
#> 303            <NA>                 <NA>           <NA>
#> 304            <NA>                 <NA>           <NA>
#> 305            <NA>                 <NA>           <NA>
#> 306            <NA>                 <NA>           <NA>
#> 307            <NA>                 <NA>           <NA>
#> 308            <NA>                 <NA>           <NA>
#> 309            <NA>                 <NA>           <NA>
#> 310            <NA>                 <NA>           <NA>
#> 311            <NA>                 <NA>           <NA>
#> 312            <NA>                 <NA>           <NA>
#> 313            <NA>                 <NA>           <NA>
#> 314            <NA>                 <NA>           <NA>
#> 315            <NA>                 <NA>           <NA>
#> 316            <NA>                 <NA>           <NA>
#> 317            <NA>                 <NA>           <NA>
#> 318            <NA>                 <NA>           <NA>
#> 319            <NA>                 <NA>           <NA>
#> 320            <NA>                 <NA>           <NA>
#> 321            <NA>                 <NA>           <NA>
#> 322            <NA>                 <NA>           <NA>
#> 323            <NA>                 <NA>           <NA>
#> 324            <NA>                 <NA>           <NA>
#> 325            <NA>                 <NA>           <NA>
#> 326            <NA>                 <NA>           <NA>
#> 327            <NA>                 <NA>           <NA>
#> 328            <NA>                 <NA>           <NA>
#> 329            <NA>                 <NA>           <NA>
#> 330            <NA>                 <NA>           <NA>
#> 331            <NA>                 <NA>           <NA>
#> 332            <NA>                 <NA>           <NA>
#> 333            <NA>                 <NA>           <NA>
#> 334            <NA>                 <NA>           <NA>
#> 335            <NA>                 <NA>           <NA>
#> 336            <NA>                 <NA>           <NA>
#> 337            <NA>                 <NA>           <NA>
#> 338            <NA>                 <NA>           <NA>
#> 339            <NA>                 <NA>           <NA>
#> 340            <NA>                 <NA>           <NA>
#> 341            <NA>                 <NA>           <NA>
#> 342            <NA>                 <NA>           <NA>
#> 343            <NA>                 <NA>           <NA>
#> 344            <NA>                 <NA>           <NA>
#> 345            <NA>                 <NA>           <NA>
#> 346            <NA>                 <NA>           <NA>
#> 347            <NA>                 <NA>           <NA>
#> 348            <NA>                 <NA>           <NA>
#> 349            <NA>                 <NA>           <NA>
#> 350            <NA>                 <NA>           <NA>
#> 351            <NA>                 <NA>           <NA>
#> 352            <NA>                 <NA>           <NA>
#> 353            <NA>                 <NA>           <NA>
#> 354            <NA>                 <NA>           <NA>
#> 355            <NA>                 <NA>           <NA>
#> 356            <NA>                 <NA>           <NA>
#> 357            <NA>                 <NA>           <NA>
#> 358            <NA>                 <NA>           <NA>
#> 359            <NA>                 <NA>           <NA>
#> 360            <NA>                 <NA>           <NA>
#> 361            <NA>                 <NA>           <NA>
#> 362            <NA>                 <NA>           <NA>
#> 363            <NA>                 <NA>           <NA>
#> 364            <NA>                 <NA>           <NA>
#> 365            <NA>                 <NA>           <NA>
#> 366            <NA>                 <NA>           <NA>
#> 367            <NA>                 <NA>           <NA>
#> 368            <NA>                 <NA>           <NA>
#> 369            <NA>                 <NA>           <NA>
#> 370            <NA>                 <NA>           <NA>
#> 371            <NA>                 <NA>           <NA>
#> 372            <NA>                 <NA>           <NA>
#> 373            <NA>                 <NA>           <NA>
#> 374            <NA>                 <NA>           <NA>
#> 375            <NA>                 <NA>           <NA>
#> 376            <NA>                 <NA>           <NA>
#> 377            <NA>                 <NA>           <NA>
#> 378            <NA>                 <NA>           <NA>
#> 379            <NA>                 <NA>           <NA>
#> 380            <NA>                 <NA>           <NA>
#> 381            <NA>                 <NA>           <NA>
#> 382            <NA>                 <NA>           <NA>
#> 383            <NA>                 <NA>           <NA>
#> 384            <NA>                 <NA>           <NA>
#> 385            <NA>                 <NA>           <NA>
#> 386            <NA>                 <NA>           <NA>
#> 387            <NA>                 <NA>           <NA>
#> 388            <NA>                 <NA>           <NA>
#> 389            <NA>                 <NA>           <NA>
#> 390            <NA>                 <NA>           <NA>
#> 391            <NA>                 <NA>           <NA>
#> 392            <NA>                 <NA>           <NA>
#> 393            <NA>                 <NA>           <NA>
#> 394            <NA>                 <NA>           <NA>
#> 395            <NA>                 <NA>           <NA>
#> 396            <NA>                 <NA>           <NA>
#> 397            <NA>                 <NA>           <NA>
#> 398            <NA>                 <NA>           <NA>
#> 399            <NA>                 <NA>           <NA>
#> 400            <NA>                 <NA>           <NA>
#> 401            <NA>                 <NA>           <NA>
#> 402            <NA>                 <NA>           <NA>
#> 403            <NA>                 <NA>           <NA>
#> 404            <NA>                 <NA>           <NA>
#> 405            <NA>                 <NA>           <NA>
#> 406            <NA>                 <NA>           <NA>
#> 407            <NA>                 <NA>           <NA>
#> 408            <NA>                 <NA>           <NA>
#> 409            <NA>                 <NA>           <NA>
#> 410            <NA>                 <NA>           <NA>
#> 411            <NA>                 <NA>           <NA>
#> 412            <NA>                 <NA>           <NA>
#> 413            <NA>                 <NA>           <NA>
#> 414            <NA>                 <NA>           <NA>
#> 415            <NA>                 <NA>           <NA>
#> 416            <NA>                 <NA>           <NA>
#> 417            <NA>                 <NA>           <NA>
#> 418            <NA>                 <NA>           <NA>
#> 419            <NA>                 <NA>           <NA>
#> 420            <NA>                 <NA>           <NA>
#> 421            <NA>                 <NA>           <NA>
#> 422            <NA>                 <NA>           <NA>
#> 423            <NA>                 <NA>           <NA>
#> 424            <NA>                 <NA>           <NA>
#> 425            <NA>                 <NA>           <NA>
#> 426            <NA>                 <NA>           <NA>
#> 427            <NA>                 <NA>           <NA>
#> 428            <NA>                 <NA>           <NA>
#> 429            <NA>                 <NA>           <NA>
#> 430            <NA>                 <NA>           <NA>
```
