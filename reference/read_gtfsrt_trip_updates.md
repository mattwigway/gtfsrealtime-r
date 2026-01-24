# Read GTFS-realtime trip updates

Note that each trip update becomes multiple rows, one per stop time
update, with the trip_id, etc., duplicated in each row. The column id
can be used to identify which rows came from the same trip update.

## Usage

``` r
read_gtfsrt_trip_updates(filename)
```

## Arguments

- filename:

  filename to read. Can be uncompressed or compressed with gzip or
  bzip2. Can also be an http:// or https:// URL.
