## {gtfsrealtime}

[![R-CMD-check](https://github.com/mattwigway/gtfsrealtime-r/actions/workflows/R-CMD-check.yaml/badge.svg)](https://github.com/mattwigway/gtfsrealtime-r/actions/workflows/R-CMD-check.yaml)
[![CRAN
status](https://www.r-pkg.org/badges/version/gtfsrealtime)](https://CRAN.R-project.org/package=gtfsrealtime)
![maintenance-status:
actively-developed](https://img.shields.io/badge/maintenance-actively_developed-blue.svg)

Fast library to read GTFS-realtime files into R data frames.

## Installation

The package is installable from CRAN, and can be installed in the usual
way:

``` r

install.packages('gtfsrealtime')
```

It requires the current or previous release of R (currently 4.6 or 4.5);
older versions are likely to work as well but will require building code
from scratch which requires a Rust development environment. If you get
errors about `rustc` not being found, you likely need to upgrade your
version of R.

If you want to build from source, this package contains compiled
[extendr](https://extendr.rs) Rust code to efficiently read
GTFS-realtime. You will need a Rust development environment; you can
build the Rust code by running
[`rextendr::document()`](https://extendr.github.io/rextendr/reference/document.html).

## Usage

GTFS-realtime feeds come in three flavors: vehicle positions, trip
updates, and service alerts. This package exposes three functions, one
for each type of file:
[`read_gtfsrt_positions()`](https://projects.indicatrix.org/gtfsrealtime-r/reference/read_gtfsrt_positions.html),
[`read_gtfsrt_trip_updates()`](https://projects.indicatrix.org/gtfsrealtime-r/reference/read_gtfsrt_trip_updates.html),
and
[`read_gtfsrt_alerts()`](https://projects.indicatrix.org/gtfsrealtime-r/reference/read_gtfsrt_alerts.html).
We also have vignettes of working with each type of file: [vehicle
positions](https://projects.indicatrix.org/gtfsrealtime-r/articles/positions.html),
[trip
updates](https://projects.indicatrix.org/gtfsrealtime-r/articles/trip_updates.html),
and [service
alerts](https://projects.indicatrix.org/gtfsrealtime-r/articles/service_alerts.html).

For most analytical applications of GTFS-realtime, you will want to work
with archived data. GTFS-realtime feeds can be quite large, so the
package supports reading feeds compressed with ZIP, `gzip`, or `bzip2`
(anecdotally, `bzip2` seems to provide slightly better compression than
`gzip`). For zip files, it is also possible to have multiple
GTFS-realtime feeds in a single file; in this case, the functions above
will read all of the files in the ZIP file. You can differentiate
records from different files with the `file_index` column. We also have
[an article demonstrating working with a day of archived
data](https://projects.indicatrix.org/gtfsrealtime-r/articles/archived.html).

GTFS-realtime is a hierarchical format, and R data frames are flat
tables. Thus, a single trip update or alert will become multiple rows in
the output, with a common `id`. See the individual function
documentation for details.

## Development versions and contributions

Development versions of the package are available from
[mattwigway.r-universe.dev](https://mattwigway.r-universe.dev/gtfsrealtime).
To install the latest development version, run:

`{r} install.packages('gtfsrealtime', repos = c('https://mattwigway.r-universe.dev', 'https://cloud.r-project.org'))`

If you want to make contributions to the package, you’ll need to build
from source. This package contains compiled
[extendr](https://extendr.rs) Rust code to efficiently read
GTFS-realtime. You will need a Rust development environment; you can
build the Rust code by running
[`rextendr::document()`](https://extendr.github.io/rextendr/reference/document.html).
You will also need to install
[`protoc`](https://protobuf.dev/installation/) if you are working with
the Git source (the `.tar.gz` source builds from CRAN compile the
protobuf files as part of the package build process, and do not require
`protoc`).
