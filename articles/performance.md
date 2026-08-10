# Performance

This article documents the performance of the `gtfsrealtime` package,
using one day of bus position data (~2.8 million observations) in New
York City. To provide a baseline, it also reads the same data with
`RProtoBuf`, the only other actively-maintained R package we are aware
of that can read GTFS-realtime.

``` r

library(gtfsrealtime)
library(RProtoBuf)
```

## Getting data

First, we download the data, as it is too large to ship with the
package.

``` r

suppressMessages({
  if (!file.exists("nyc-bus-demo.zip")) {
    download.file(
      "https://files.indicatrix.org/gtfsrealtime-r/nyc-bus-demo.zip",
      "nyc-bus-demo.zip"
    )
  }
})
```

## Reading with `gtfsrealtime`

`gtfsrealtime` can read directly from the ZIP archive. We use `as_sf`
here, since that is used in the paper, but when we go head to head with
RProtoBuf below we don’t to make a fair comparison.

``` r

system.time({data_gtfsrealtime = read_gtfsrt_positions("nyc-bus-demo.zip", "America/New_York", as_sf=T)})
#>    user  system elapsed 
#>  11.063   3.002  15.444
```

Confirm we read all observations:

``` r

nrow(data_gtfsrealtime)
#> [1] 2825902
```

## Comparison with RProtoBuf

### Proto loading

First, we need to read the `.proto` file.

``` r

readProtoFiles2(here::here("src/rust/src/gtfs-realtime.proto"))
```

### Data preparation

The ZIP file contains bzipped protobuf files. `gtfsrealtime` supports
this natively, but RProtoBuf does not. So we unzip and uncompress the
files first.

``` r

pbdir = tempfile()
dir.create(pbdir)

unzip("nyc-bus-demo.zip", exdir=pbdir)



for (file in list.files(file.path(pbdir, "nyc-bus-demo"), full.names=T)) {
  system2("bunzip2", c(file))
}
```

### The parameters of the test

- Read all of the files in the directory into a list of data frames, one
  for each file

``` r

input_files = list.files(file.path(pbdir, "nyc-bus-demo"), full.names=T)
stopifnot(length(input_files) == 1440) # minutely for one day
```

### Re-reading with `gtfsrealtime`

We will read the uncompressed files with `gtfsrealtime` to get a fair
comparison, without decompression time included, but including the
additional file operations RProtoBuf will do. If anything this tilts the
field towards RProtoBuf, as the `gtfsrealtime` version is loaded first
so files may be e.g. cached by the operating system or hardware.

``` r

system.time({data_gtfsrealtime_uncompressed = lapply(input_files, \(f) read_gtfsrt_positions(f, "America/New_York"))})
#>    user  system elapsed 
#>   6.551   4.601  11.811
```

### Reading with RProtoBuf

Next, we will read them with RProtoBuf:

``` r

system.time({
  data_rpbf = lapply(input_files, \(f) RProtoBuf::read(transit_realtime.FeedMessage, f))
})
#>    user  system elapsed 
#>   2.398   0.425   3.681
```

This is actually significantly faster, but is not a fair comparison. The
output of `RProtoBuf::read` is a hierarchical object, which is
notoriously slow and difficult to work with in R, and which will need to
be converted to tabular to work with with common, performant libraries
in R. The output of `gtfsrealtime` is already in tabular format.

### Reading and converting to tabular format with RProtoBuf

For a truly fair test, we need to also convert the RProtoBuf output to
data frames like the GTFS-rt output. We do this two ways, once with a
“row-wise” approach where each vehicle position is converted to a data
frame and then row-bound, and oncw with a “column-wise” where a single
data frame is constructed from columns that are constructed from the
hierarchical data with `vapply`. This is still not a completely fair
comparison, as it doesn’t have some of the features `gtfsrealtime` has,
such as ID deduplication or human readable enum columns, but should be
close in terms of functionality.

#### Column-wise approach

``` r

flatten_position_message = function(message) {
  data.frame(
    id = vapply(message$entity, \(x) x$id, ""),
    latitude = vapply(message$entity, \(e) e$vehicle$position$latitude, 42.0),
    longitude = vapply(message$entity, \(e) e$vehicle$position$longitude, 42.0),
    bearing = vapply(message$entity, \(e) e$vehicle$position$bearing, 42.0),
    odometer = vapply(message$entity, \(e) e$vehicle$position$odometer, 42.0),
    speed = vapply(message$entity, \(e) e$vehicle$position$speed, 42.0),
    trip_id = vapply(message$entity, \(e) e$vehicle$trip$trip_id, ""),
    route_id = vapply(message$entity, \(e) e$vehicle$trip$route_id, ""),
    direction_id = vapply(message$entity, \(e) e$vehicle$trip$direction_id, 0),
    start_time = vapply(message$entity, \(e) e$vehicle$trip$start_time, ""),
    start_date = vapply(message$entity, \(e) e$vehicle$trip$start_date, ""),
    schedule_relationship = vapply(message$entity, \(e) e$vehicle$trip$schedule_relationship, 0L),
    stop_id = vapply(message$entity, \(e) e$vehicle$stop_id, ""),
    current_stop_sequence = vapply(message$entity, \(e) e$vehicle$current_stop_sequence, 42.0),
    current_status = vapply(message$entity, \(e) e$vehicle$current_status, 0L),
    timestamp = as.POSIXct(vapply(message$entity, \(e) e$vehicle$timestamp, 42.0), "America/New_York"),
    congestion_level = vapply(message$entity, \(e) e$vehicle$congestion_level, 0L),
    occupancy_status = vapply(message$entity, \(e) e$vehicle$occupancy_status, 0L),
    occupancy_percentage = vapply(message$entity, \(e) e$vehicle$occupancy_percentage, 42.0),
    vehicle_id = vapply(message$entity, \(e) e$vehicle$vehicle$id, ""),
    vehicle_label = vapply(message$entity, \(e) e$vehicle$vehicle$label, ""),
    vehicle_license_plate = vapply(message$entity, \(e) e$vehicle$vehicle$license_plate, ""),
    wheelchair_accessible = vapply(message$entity, \(e) e$vehicle$vehicle$wheelchair_accessible, 0L)
  )
}


system.time({
  data_rpbf_columnwise = lapply(input_files, \(f) flatten_position_message(RProtoBuf::read(transit_realtime.FeedMessage, f)))
})
#>     user   system  elapsed 
#> 1403.794   29.809 1457.412
```

#### Row-wise approach

We define the function to process a single row of the data, and then
apply it as part of the reading process

``` r

flatten_position_update = function(entity) {
  pos = entity$vehicle

  data.frame(
    id = entity$id,
    latitude = pos$position$latitude,
    longitude = pos$position$longitude,
    bearing = pos$position$bearing,
    odometer = pos$position$odometer,
    speed = pos$position$speed,
    trip_id = pos$trip$trip_id,
    route_id = pos$trip$route_id,
    direction_id = pos$trip$direction_id,
    start_time = pos$trip$start_time,
    start_date = pos$trip$start_date,
    schedule_relationship = pos$trip$schedule_relationship,
    stop_id = pos$stop_id,
    current_stop_sequence = pos$current_stop_sequence,
    current_status = pos$current_status,
    timestamp = as.POSIXct(pos$timestamp, "America/New_York"),
    congestion_level = pos$congestion_level,
    occupancy_status = pos$occupancy_status,
    occupancy_percentage = pos$occupancy_percentage,
    vehicle_id = pos$vehicle$id,
    vehicle_label = pos$vehicle$label,
    vehicle_license_plate = pos$vehicle$license_plate,
    wheelchair_accessible = pos$vehicle$wheelchair_accessible
  )
}

system.time({
  data_rpbf_rowwise = lapply(input_files, function(f) {
    msg = RProtoBuf::read(transit_realtime.FeedMessage, f)
    lapply(msg$entity, flatten_position_update) |> rbind()
  })
})
#>     user   system  elapsed 
#> 2278.537 2246.347 5154.453
```

### Conclusion

`gtfsrealtime` is more than two orders of magnitude faster than an
RProtoBuf workflow using standard R data processing tools.
`gtfsrealtime` is also more user-friendly, as a single line of code
reads a whole day of data and formats it as a data frame.
