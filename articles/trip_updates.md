# Trip updates

This vignette shows how to read GTFS-realtime trip updates into R using
the {gtfsrealtime} package. Trip updates describe the real-time progress
of a vehicle along a scheduled trip, including predicted arrival and
departure times, delays, skipped stops, canceled trips, and other
real-time information. The {gtfsrealtime} package reads the nested
GTFS-realtime trip update format and flattens it into a data frame that
is easier to inspect and analyze in R.

## Load libraries

First, we load {gtfsrealtime} to read GTFS-realtime files and {dplyr} to
inspect and summarize the resulting data frame.

``` r

library(gtfsrealtime)
library(dplyr)
```

## Load a GTFS-realtime trip updates feed

This example uses a New York City trip updates feed included with
{gtfsrealtime}. The file is compressed with bzip2 to save space.
{gtfsrealtime} can automatically detect and read uncompressed files as
well as files compressed with zip, gzip, or bzip2. Zip files can contain
multiple GTFS-realtime files, in which case {gtfsrealtime} will read all
of them. You can differentiate which file each update came from based on
the `file_index` field.

GTFS-realtime time values are stored as Unix timestamps, which are
interpreted relative to UTC. To convert to local time, we provide a
local time zone. Time zones are specified in standardized TZ database
format, generally Continent/City. If you do not want to convert times,
you can specify a time zone of Etc/UTC.

``` r

updates <- read_gtfsrt_trip_updates(
  system.file("nyc-trip-updates.pb.bz2", package = "gtfsrealtime"),
  "America/New_York"
)
```

When reading this example feed, {gtfsrealtime} warns that some
GTFS-realtime entity IDs are duplicated. In these cases, the package
appends suffixes such as `_duplicated_1` so that each row can be
represented with a unique `id`. There are quite a few of them, so they
are suppressed here to keep the vignette readable, but the first two
are:

    1: ! ID UP_A6-Weekday-SDon-094800_B6_243 is duplicated. Replacing with UP_A6-Weekday-SDon-094800_B6_243_duplicated_1 . This may cause joins between different
      GTFS-realtime files (even within a ZIP archive) to be incorrect.
    2: ! ID UP_A6-Weekday-SDon-094800_B6_243 is duplicated. Replacing with UP_A6-Weekday-SDon-094800_B6_243_duplicated_2 . This may cause joins between different
      GTFS-realtime files (even within a ZIP archive) to be incorrect.

These warnings are useful in practice: duplicated entity IDs can affect
workflows that join records across GTFS-realtime files or across
multiple files within a ZIP archive, as IDs may no longer match across
files.

## Explore trip updates

GTFS-realtime trip updates are
[hierarchical](https://gtfs.org/documentation/realtime/reference/#message-tripupdate);
one trip update can contain information about the trip as a whole as
well as updates for multiple stops along that trip.
[`read_gtfsrt_trip_updates()`](https://projects.indicatrix.org/gtfsrealtime-r/reference/read_gtfsrt_trip_updates.md)
flattens that structure into a data frame. As a result, the same
`trip_id` may appear in multiple rows when the feed contains stop-level
updates for multiple stops.

``` r

glimpse(updates)
#> Rows: 90,881
#> Columns: 26
#> $ id                            <chr> "MV_A6-Weekday-SDon-102600_M96_826", "MV…
#> $ trip_id                       <chr> "MV_A6-Weekday-SDon-102600_M96_826", "MV…
#> $ route_id                      <chr> "M96", "M96", "M96", "M96", "M96", "M96"…
#> $ direction_id                  <dbl> 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1…
#> $ start_time                    <chr> NA, NA, NA, NA, NA, NA, NA, NA, NA, NA, …
#> $ start_date                    <chr> "20260128", "20260128", "20260128", "202…
#> $ trip_schedule_relationship    <fct> NA, NA, NA, NA, NA, NA, NA, NA, NA, NA, …
#> $ modifications_id              <chr> NA, NA, NA, NA, NA, NA, NA, NA, NA, NA, …
#> $ vehicle_id                    <chr> "MTA NYCT_9771", "MTA NYCT_9771", "MTA N…
#> $ vehicle_label                 <chr> NA, NA, NA, NA, NA, NA, NA, NA, NA, NA, …
#> $ vehicle_license_plate         <chr> NA, NA, NA, NA, NA, NA, NA, NA, NA, NA, …
#> $ vehicle_wheelchair_accessible <fct> NA, NA, NA, NA, NA, NA, NA, NA, NA, NA, …
#> $ stop_sequence                 <dbl> 1, 3, 4, 5, 6, 7, 8, 9, 10, 11, 1, 2, 3,…
#> $ stop_id                       <chr> "401933", "401935", "401936", "401937", …
#> $ arrival_delay                 <int> NA, NA, NA, NA, NA, NA, NA, NA, NA, NA, …
#> $ arrival_time                  <dttm> 2026-01-28 17:13:20, 2026-01-28 17:14:1…
#> $ arrival_scheduled_time        <dttm> NA, NA, NA, NA, NA, NA, NA, NA, NA, NA,…
#> $ arrival_uncertainty           <int> NA, NA, NA, NA, NA, NA, NA, NA, NA, NA, …
#> $ departure_delay               <int> NA, NA, NA, NA, NA, NA, NA, NA, NA, NA, …
#> $ departure_time                <dttm> 2026-01-28 17:13:20, 2026-01-28 17:14:1…
#> $ departure_scheduled_time      <dttm> NA, NA, NA, NA, NA, NA, NA, NA, NA, NA,…
#> $ departure_uncertainty         <int> NA, NA, NA, NA, NA, NA, NA, NA, NA, NA, …
#> $ departure_occupancy_status    <fct> NA, NA, NA, NA, NA, NA, NA, NA, NA, NA, …
#> $ stop_schedule_relationship    <fct> NA, NA, NA, NA, NA, NA, NA, NA, NA, NA, …
#> $ file_timestamp                <dttm> 2026-01-28 17:13:34, 2026-01-28 17:13:3…
#> $ file_index                    <int> 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1…
```

## Inspecting one trip across its stops

Because a single trip can include predictions for multiple stops, it is
useful to inspect all rows associated with one `trip_id`. In the example
below, we select the first trip in the feed and display the route, stop
sequence, stop ID, and predicted arrival and departure times for each
stop. If a trip update has no stop time updates, it will appear as a
single row with all the `stop_*` fields NA. Documentation for all of the
columns is in the documentation for
[`read_gtfsrt_trip_updates()`](https://projects.indicatrix.org/gtfsrealtime-r/reference/read_gtfsrt_trip_updates.md).

``` r

updates |>
  filter(trip_id == first(trip_id)) |>
  select(
    trip_id,
    route_id,
    stop_id,
    stop_sequence,
    arrival_time,
    departure_time,
    arrival_delay,
    departure_delay
  )
#>                              trip_id route_id stop_id stop_sequence
#> 1  MV_A6-Weekday-SDon-102600_M96_826      M96  401933             1
#> 2  MV_A6-Weekday-SDon-102600_M96_826      M96  401935             3
#> 3  MV_A6-Weekday-SDon-102600_M96_826      M96  401936             4
#> 4  MV_A6-Weekday-SDon-102600_M96_826      M96  401937             5
#> 5  MV_A6-Weekday-SDon-102600_M96_826      M96  404087             6
#> 6  MV_A6-Weekday-SDon-102600_M96_826      M96  401939             7
#> 7  MV_A6-Weekday-SDon-102600_M96_826      M96  401941             8
#> 8  MV_A6-Weekday-SDon-102600_M96_826      M96  401942             9
#> 9  MV_A6-Weekday-SDon-102600_M96_826      M96  401943            10
#> 10 MV_A6-Weekday-SDon-102600_M96_826      M96  903003            11
#>           arrival_time      departure_time arrival_delay departure_delay
#> 1  2026-01-28 17:13:20 2026-01-28 17:13:20            NA              NA
#> 2  2026-01-28 17:14:15 2026-01-28 17:14:15            NA              NA
#> 3  2026-01-28 17:15:58 2026-01-28 17:15:58            NA              NA
#> 4  2026-01-28 17:17:42 2026-01-28 17:17:42            NA              NA
#> 5  2026-01-28 17:21:41 2026-01-28 17:21:41            NA              NA
#> 6  2026-01-28 17:24:30 2026-01-28 17:24:30            NA              NA
#> 7  2026-01-28 17:27:43 2026-01-28 17:27:43            NA              NA
#> 8  2026-01-28 17:30:05 2026-01-28 17:30:05            NA              NA
#> 9  2026-01-28 17:32:03 2026-01-28 17:32:03            NA              NA
#> 10 2026-01-28 17:33:38 2026-01-28 17:33:38            NA              NA
```
