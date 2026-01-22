## {gtfsrealtime}

Fast library to read GTFS-realtime files into R data frames.

## Installation

The package is installable from
[mattwigway.r-universe.dev](https://mattwigway.r-universe.dev/gtfsrealtime).
Run the following at your R prompt:

``` r
install.packages('gtfsrealtime', repos = c('https://mattwigway.r-universe.dev', 'https://cloud.r-project.org'))
```

If you want to build from source, this package contains compiled
[extendr](https://extendr.rs) Rust code to efficiently read
GTFS-realtime. You will need a Rust development environment; you can
build the Rust code by running `rextendr::document()`.

## Usage

Currently, this library only supports reading vehicle position feeds.
See [this
vignette](https://projects.indicatrix.org/gtfsrealtime-r/articles/positions.html)
for a walkthrough.
