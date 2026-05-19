# Service alerts

This vignette shows how to read GTFS-realtime service alerts into R
using the {gtfsrealtime} package. Service alerts describe disruptions,
planned work, stop closures, detours, and other rider-facing
information. The {gtfsrealtime} package reads the nested GTFS-realtime
alert format and flattens it into a data frame that is easier to inspect
and analyze in R.

## Load libraries

First, we load {gtfsrealtime} to read GTFS-realtime files, {dplyr} to
inspect and summarize the resulting data frame, and {stringr} to shorten
long alert messages for display.

``` r

library(gtfsrealtime)
library(dplyr)
library(stringr)
```

## Load a GTFS-realtime service alerts feed

This example uses a New York City service alerts feed included with
{gtfsrealtime}. The file is compressed with bzip2 to save space.
{gtfsrealtime} can automatically detect and read uncompressed files as
well as files compressed with zip, gzip, or bzip2. Zip files can contain
multiple GTFS-realtime files, in which case {gtfsrealtime} will read all
of them. You can differentiate which file each update came from based on
the `file_index` field.

GTFS-realtime time values are stored as Unix timestamps, which are
interpreted relative to UTC. To convert to local time, we provide a
local time zone. Time zones are specified in standardized TZ database
format (generally `Continent/City`; for a list, [see
here](https://en.wikipedia.org/wiki/List_of_tz_database_time_zones)). If
you do not want to convert times, you can specify a time zone of
`Etc/UTC`.

``` r

alerts <- read_gtfsrt_alerts(
  system.file("nyc-service-alerts.pb.bz2", package = "gtfsrealtime"),
  "America/New_York"
)
```

## Explore the alerts

GTFS-realtime alerts are
[nested](https://gtfs.org/documentation/realtime/reference/#message-alert):
one alert can include multiple active time periods and multiple informed
entities, such as routes, stops, trips, or agencies affected by the
alert.
[`read_gtfsrt_alerts()`](https://projects.indicatrix.org/gtfsrealtime-r/reference/read_gtfsrt_alerts.md)
flattens this structure into a data frame. As a result, the same alert
`id` may appear in multiple rows when an alert applies to more than one
entity or time period.

``` r

glimpse(alerts)
#> Rows: 430
#> Columns: 28
#> $ id                         <chr> "MTA NYCT_lmm:planned_work:22245", "MTA NYC…
#> $ start                      <dttm> 2025-01-03 00:00:35, 2025-01-03 00:00:35, …
#> $ end                        <dttm> NA, NA, 2026-06-30 00:00:00, 2026-06-30 00…
#> $ agency_id                  <chr> "MTA NYCT", "MTA NYCT", "MTA NYCT", "MTA NY…
#> $ route_id                   <chr> "X38", "X28", NA, NA, NA, NA, "B9", "B63", …
#> $ route_type                 <int> NA, NA, NA, NA, NA, NA, NA, NA, NA, NA, NA,…
#> $ direction_id               <dbl> NA, NA, NA, NA, NA, NA, NA, NA, NA, NA, NA,…
#> $ trip_trip_id               <chr> NA, NA, NA, NA, NA, NA, NA, NA, NA, NA, NA,…
#> $ trip_route_id              <chr> NA, NA, "M15+", "M15+", "S66", "S66", NA, N…
#> $ trip_direction_id          <dbl> NA, NA, 0, 1, 1, 0, NA, NA, NA, 1, 0, 0, 1,…
#> $ trip_start_time            <chr> NA, NA, NA, NA, NA, NA, NA, NA, NA, NA, NA,…
#> $ trip_start_date            <chr> NA, NA, NA, NA, NA, NA, NA, NA, NA, NA, NA,…
#> $ trip_schedule_relationship <fct> NA, NA, NA, NA, NA, NA, NA, NA, NA, NA, NA,…
#> $ trip_modification_id       <chr> NA, NA, NA, NA, NA, NA, NA, NA, NA, NA, NA,…
#> $ stop_id                    <chr> NA, NA, NA, NA, NA, NA, NA, NA, NA, NA, NA,…
#> $ cause                      <fct> NA, NA, NA, NA, NA, NA, NA, NA, NA, NA, NA,…
#> $ effect                     <fct> NA, NA, NA, NA, NA, NA, NA, NA, NA, NA, NA,…
#> $ language                   <chr> "EN", "EN", "EN", "EN", "EN", "EN", "EN", "…
#> $ cause_detail               <chr> NA, NA, NA, NA, NA, NA, NA, NA, NA, NA, NA,…
#> $ effect_detail              <chr> NA, NA, NA, NA, NA, NA, NA, NA, NA, NA, NA,…
#> $ url                        <chr> NA, NA, NA, NA, NA, NA, NA, NA, NA, NA, NA,…
#> $ header_text                <chr> "X28  and X38  stops on Surf Ave at W 21st …
#> $ description_text           <chr> "X28  and X38  stops on Surf Ave at W 21st …
#> $ tts_header_text            <chr> NA, NA, NA, NA, NA, NA, NA, NA, NA, NA, NA,…
#> $ tts_description_text       <chr> NA, NA, NA, NA, NA, NA, NA, NA, NA, NA, NA,…
#> $ severity_level             <fct> NA, NA, NA, NA, NA, NA, NA, NA, NA, NA, NA,…
#> $ file_timestamp             <dttm> 2026-01-28 08:57:42, 2026-01-28 08:57:42, …
#> $ file_index                 <int> 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1…
```

## Inspecting a single alert

Because a single alert can apply to more than one route, stop, or trip,
it is useful to inspect all rows associated with one alert id. In the
example below, we select the first alert in the feed and display the
affected route or stop fields along with the rider-facing alert text.

``` r

alerts |>
  filter(id == first(id)) |>
  select(id, route_id, stop_id, header_text, description_text)
#>                                id route_id stop_id
#> 1 MTA NYCT_lmm:planned_work:22245      X38    <NA>
#> 2 MTA NYCT_lmm:planned_work:22245      X28    <NA>
#>                                                                  header_text
#> 1 X28  and X38  stops on Surf Ave at W 21st St are closed in both directions
#> 2 X28  and X38  stops on Surf Ave at W 21st St are closed in both directions
#>                                                                                                                                                                                                                                                                                                                                     description_text
#> 1 X28  and X38  stops on Surf Ave at W 21st St are closed in both directions\nFor northbound service, use the temporary stop on Surf Ave at W 22nd St.\nFor southbound service, use the stop on W 17th St at Mermaid Ave.\n\nWhat's happening?\nConstruction\n\nNote: Bus arrival information may not be available/accurate while buses are detoured
#> 2 X28  and X38  stops on Surf Ave at W 21st St are closed in both directions\nFor northbound service, use the temporary stop on Surf Ave at W 22nd St.\nFor southbound service, use the stop on W 17th St at Mermaid Ave.\n\nWhat's happening?\nConstruction\n\nNote: Bus arrival information may not be available/accurate while buses are detoured
```

## Summarizing affected routes

We can also summarize which routes appear most often in the alerts feed.
Route IDs appear in two places in the alerts feed: in the `route_id`
column and in the `trip_route_id` column (which correspond to alerts
that apply to an entire route, and those that correspond to a single
trip on a route). First, we put those together, and then we count the
number of routes. Note that `trip_route_id` is optional if the trip ID
itself is specified, so it might be necessary to refer to the static
GTFS to map trip IDs to route IDs with some feeds, though in the example
feed all updates have route IDs or trip route IDs. It is also possible
to have service alerts that apply to specific stops, modes
(bus/tram/etc), or agency, and do not have a route ID, though there are
none in the New York MTA example feed. If alerts have translated strings
or multiple time periods, they may be in multiple rows in the data
frame, so we make sure to group by the alert ID and select only a single
instance before counting the number of alerts by route.

``` r

# make sure trip_route_id and route_id always agree if both are specified
stopifnot(with(alerts, all(is.na(route_id) | is.na(trip_route_id) | route_id == trip_route_id)))

# make sure every update has a route id
stopifnot(with(alerts, all(!is.na(route_id) | !is.na(trip_route_id))))

alerts |>
  mutate(route_id = coalesce(route_id, trip_route_id)) |>
  group_by(id) |>
  slice_head(n=1) |>
  ungroup() |>
  count(route_id, sort = TRUE) |>
  head(10)
#> # A tibble: 10 × 2
#>    route_id     n
#>    <chr>    <int>
#>  1 M15          3
#>  2 M15+         3
#>  3 M66          3
#>  4 B44          2
#>  5 B65          2
#>  6 BX39         2
#>  7 Q18          2
#>  8 Q32          2
#>  9 Q47          2
#> 10 B12          1
```

## Working with alert text

The alert text fields contain the rider-facing message. `header_text`
usually provides a short summary, while `description_text` provides more
detail. All fields are described in the documentation for
\[[`read_gtfsrt_alerts()`](https://projects.indicatrix.org/gtfsrealtime-r/reference/read_gtfsrt_alerts.md)\].

``` r

 alerts |>
     distinct(id, header_text, description_text) |>
     mutate(
         header_text = str_trunc(header_text, 80),
         description_text = str_trunc(description_text, 120)
     ) |>
     head()
#>                                id
#> 1 MTA NYCT_lmm:planned_work:22245
#> 2 MTA NYCT_lmm:planned_work:29838
#> 3       MTA NYCT_lmm:alert:503501
#> 4       MTA NYCT_lmm:alert:503667
#> 5 MTA NYCT_lmm:planned_work:22403
#> 6 MTA NYCT_lmm:planned_work:29559
#>                                                                        header_text
#> 1       X28  and X38  stops on Surf Ave at W 21st St are closed in both directions
#> 2 Northbound M15-SBS stop on Water St at Pine St is closed, buses make a tempor...
#> 3 S66 buses are detoured in both directions due to icy roads at Arlo Rd/Stratfo...
#> 4                               You may wait longer for these buses:\nB9, B35, B63
#> 5 Southbound Bx39 stop on White Plains Rd at E 215th St is closed; please use t...
#> 6 M5 stop on W 31st St at 6th Ave is closed; for service, consider the southbou...
#>                                                                                                             description_text
#> 1  X28  and X38  stops on Surf Ave at W 21st St are closed in both directions\nFor northbound service, use the temporary ...
#> 2   Northbound M15-SBS stop on Water St at Pine St is closed, buses make a temporary stop across the intersection on Wate...
#> 3  S66 buses are detoured in both directions due to icy roads at Arlo Rd/Stratford Ave.\nS66 buses in both directions wil...
#> 4 You may wait longer for these buses:\nB9, B35, B63\nWe're running as much service as we can with the operators we have ...
#> 5   Southbound Bx39 stop on White Plains Rd at E 215th St is closed; please use the new stop on White Plains Rd at E 216t...
#> 6   M5 stop on W 31st St at 6th Ave is closed; for service, consider the southbound stop on 5th Ave at W 33rd St or the n...
```
