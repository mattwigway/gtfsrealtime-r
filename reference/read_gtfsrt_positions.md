# Read GTFS-realtime vehicle positions into a data frame

Read GTFS-realtime vehicle positions into a data frame

## Usage

``` r
read_gtfsrt_positions(filename, as_sf = FALSE)
```

## Arguments

- filename:

  filename to read. Can be uncompressed or compressed with gzip or
  bzip2. Can also be an http:// or https:// URL.

- as_sf:

  return an sf (spatial) object rather than a data frame.
