# Read GTFS-realtime vehicle positions into a data frame

Read GTFS-realtime vehicle positions into a data frame

## Usage

``` r
read_gtfsrt_positions(filename, timezone, as_sf = FALSE)
```

## Arguments

- filename:

  filename to read. Can be uncompressed or compressed with gzip or
  bzip2. Can also be an http:// or https:// URL.

- timezone:

  timezone of feed, in Olson format. Times in GTFS-realtime are stored
  as Unix time in UTC; this option will convert to local times. If you
  want to read times in UTC, specify "Etc/UTC"

- as_sf:

  return an sf (spatial) object rather than a data frame.
