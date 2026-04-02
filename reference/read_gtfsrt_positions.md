# Read GTFS-realtime vehicle positions into a data frame

Read GTFS-realtime vehicle positions into a data frame

## Usage

``` r
read_gtfsrt_positions(filename, timezone, as_sf = FALSE, label_values = TRUE)
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

- label_values:

  should enum types in GTFS-realtime (i.e. categorical variables) be
  converted to factors with their English labels. If false, they will be
  left as numeric codes. Default true.
