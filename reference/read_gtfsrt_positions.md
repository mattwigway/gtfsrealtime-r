# Read GTFS-realtime vehicle positions into a data frame

Read GTFS-realtime vehicle positions into a data frame

## Usage

``` r
read_gtfsrt_positions(filename, as_sf = FALSE)
```

## Arguments

- filename:

  filename to read. can be uncompressed or compressed with gzip or
  bzip2.

- as_sf:

  return an sf (spatial) object rather than a data frame.
