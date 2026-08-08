## {gtfsrealtime}

  [![R-CMD-check](https://github.com/mattwigway/gtfsrealtime-r/actions/workflows/R-CMD-check.yaml/badge.svg)](https://github.com/mattwigway/gtfsrealtime-r/actions/workflows/R-CMD-check.yaml)

<img src="https://img.shields.io/badge/maintenance-actively_developed-blue.svg" alt="maintenance-status: actively-developed" />

Fast library to read GTFS-realtime files into R data frames.

## Installation

The package is installable from CRAN, and can be installed in the usual way:

```r
install.packages('gtfsrealtime')
```

It requires the current or previous release of R (currently 4.6 or 4.5); older versions are likely to work as well but will require building code from scratch which requires a Rust development environment. If you get errors about `rustc` not being found, you likely need to upgrade your version of R.

If you want to build from source, this package contains compiled [extendr](https://extendr.rs) Rust code to efficiently read GTFS-realtime. You will need a Rust development environment; you can build the Rust code by running `rextendr::document()`.

## Usage

GTFS-realtime feeds come in three flavors: vehicle positions, trip updates, and service alerts. This package exposes three functions, one for each type of file: [`read_gtfsrt_positions()`](https://projects.indicatrix.org/gtfsrealtime-r/reference/read_gtfsrt_positions.html), [`read_gtfsrt_trip_updates()`](https://projects.indicatrix.org/gtfsrealtime-r/reference/read_gtfsrt_trip_updates.html), and [`read_gtfsrt_alerts()`](https://projects.indicatrix.org/gtfsrealtime-r/reference/read_gtfsrt_alerts.html). We also have vignettes of working with each type of file: [vehicle positions](https://projects.indicatrix.org/gtfsrealtime-r/articles/positions.html), [trip updates](https://projects.indicatrix.org/gtfsrealtime-r/articles/trip_updates.html), and [service alerts](https://projects.indicatrix.org/gtfsrealtime-r/articles/service_alerts.html).

For most analytical applications of GTFS-realtime, you will want to work with archived data. GTFS-realtime feeds can be quite large, so the package supports reading feeds compressed with ZIP, `gzip`, or `bzip2` (anecdotally, `bzip2` seems to provide slightly better compression than `gzip`). For zip files, it is also possible to have multiple GTFS-realtime feeds in a single file; in this case, the functions above will read all of the files in the ZIP file. You can differentiate records from different files with the `file_index` column. We also have [an article demonstrating working with a day of archived data](https://projects.indicatrix.org/gtfsrealtime-r/articles/archived.html).

GTFS-realtime is a hierarchical format, and R data frames are flat tables. Thus, a single trip update or alert will become multiple rows in the output, with a common `id`. See the individual function documentation for details.

GTFS-realtime has several experimental extensions, which are not currently supported. Furthermore, GTFS-realtime differential updates (e.g. just providing some new vehicle positions) are considered unsupported by the current GTFS-realtime specification, and thus we do not support them. If you have a feed where you encounter any of these situations, please [open an issue](https://github.com/mattwigway/gtfsrealtime-r/issues/new) (and attach the feed if you can)!

### Error handling

Like most large datasets, GTFS-realtime data can have problems. Truncated or corrupted files may or may not be read successfully. If the truncation or corruption affects the protocol buffers structure an error will generally be emitted. If the corruption has instead corrupted a specific value (e.g. replaced a route ID of "16_10" with "1>_10"), the incorrect value will be read. If a truncation has happened to truncate the data exactly at the boundary between two messages, only the messages before the truncation will be read (this also means that if you have a truncated file that you cannot read, removing bytes from the end one at a time will likely eventually yield a file you can read, containing all records before the truncation). It is not possible to detect truncation or corruption without an external checksum, but it is also exceedingly rare. This same limitation is shared by other data formats, such as CSV.

Lastly, it is possible for feed providers to use values for categorical variables that are not defined in the GTFS-realtime specification—for instance, a cause for an alert of `POWER_OUTAGE`. Because categorical variables are stored as numbers in GTFS-realtime feeds, the package has no way to know what text to associate with any values that are not in the specification. In this case, the data will just contain a number for that value, rather than the text description. If you are finding numbers interspersed in columns that otherwise contain text, this is likely the cause.


## Development versions and contributions

Development versions of the package are available from [mattwigway.r-universe.dev](https://mattwigway.r-universe.dev/gtfsrealtime). To install the latest development version, run:

```{r}
install.packages('gtfsrealtime', repos = c('https://mattwigway.r-universe.dev', 'https://cloud.r-project.org'))
```

If you want to make contributions to the package, you'll need to build from source. This package contains compiled [extendr](https://extendr.rs) Rust code to efficiently read GTFS-realtime. You will need a Rust development environment; you can build the Rust code by running `rextendr::document()`. You will also need to install [`protoc`](https://protobuf.dev/installation/) if you are working with the Git source (the `.tar.gz` source builds from CRAN compile the protobuf files as part of the package build process, and do not require `protoc`).