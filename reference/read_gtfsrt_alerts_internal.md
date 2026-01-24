# Read a GTFS-rt service alerts feed. GTFS-rt alerts support translations. If there is more than one translation in an alert, there will be one row for that alert in each language. Alerts in all languages will share a feed_index.

Read a GTFS-rt service alerts feed. GTFS-rt alerts support translations.
If there is more than one translation in an alert, there will be one row
for that alert in each language. Alerts in all languages will share a
feed_index.

## Usage

``` r
read_gtfsrt_alerts_internal(file)
```
