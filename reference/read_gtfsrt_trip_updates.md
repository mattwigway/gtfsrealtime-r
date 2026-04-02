# Read GTFS-realtime trip updates

Note that each trip update becomes multiple rows, one per stop time
update, with the trip_id, etc., duplicated in each row. The column id
can be used to identify which rows came from the same trip update.

## Usage

``` r
read_gtfsrt_trip_updates(filename, timezone, label_values = TRUE)
```

## Arguments

- filename:

  filename to read. Can be uncompressed or compressed with gzip or
  bzip2. Can also be an http:// or https:// URL.

- timezone:

  timezone of feed, in Olson format. Times in GTFS-realtime are stored
  as Unix time in UTC; this option will convert to local times. If you
  want to read times in UTC, specify "Etc/UTC"

- label_values:

  should enum types in GTFS-realtime (i.e. categorical variables) be
  converted to factors with their English labels. If false, they will be
  left as numeric codes. Default true.
