# Read GTFS-realtime alerts

This function reads GTFS realtime alerts. Alerts are hierarchical: a
single alert can have multiple applicability periods, multiple affected
entities, and translations to multiple languages. This function flattens
all of that to a tabular format, with one row for every combination of
applicability, entity, and language. All rows from a single alert can be
identified through a common id.

## Usage

``` r
read_gtfsrt_alerts(filename)
```

## Arguments

- filename:

  filename to read. Can be uncompressed or compressed with gzip or
  bzip2. Can also be an http:// or https:// URL.
