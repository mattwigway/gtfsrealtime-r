# This prepares the package for CRAN submission by:
# - Vendoring Rust dependencies
# - Creating inst/AUTHORS.md
# - Creating LICENSE.note.md

if (!require(rextendr) | !require(RcppTOML)) {
  stop("rextendr and RcppTOML are needed at build time to prepare for CRAN submission")
}

extract_info = function(file) {
  toml = RcppTOML::parseTOML(file)
  result = list(
    package = toml$package$name,
    authors = toml$package$authors,
    url = toml$package$homepage,
    version = toml$package$version,
    license = toml$package$license
  )

  if (is.null(result$url)) {
    result$url = toml$package$repository
  }

  if (is.null(result$url)) {
    result$url = sprintf("https://docs.rs/", result$package)
  }

  return(result)
}

extract_licenses = function(licenses) {
  lics = regmatches(licenses, gregexpr("[^ \\(\\)/]+", licenses))[[1]]
  lics[!(lics %in% c("AND", "OR", "WITH"))]
}

# vendor Rust packages
rextendr::vendor_pkgs()

# extract author and license information
vendor_files = untar("src/rust/vendor.tar.xz", list = TRUE)
cargo_files = vendor_files[grepl("Cargo.toml$", vendor_files)]

dir = tempfile()
dir.create(dir)

untar("src/rust/vendor.tar.xz", files = cargo_files, exdir = dir)

crate_info = lapply(file.path(dir, cargo_files), extract_info)

unlink(dir)

# create inst/AUTHORS.md
authfile = file("inst/AUTHORS.md", "wt")
cat(
  "# Authors

`gtfsrealtime` is written by Matt Bhagat-Conway <mwbc@unc.edu>. It includes
public-domain and Apache-licensed code and documentation from the
[GTFS-realtime specification](https://gtfs.org/documentation/realtime), written
and copyrighted by the collective entity \"The GTFS specification authors\".
Additionally, it includes code from a number of Rust crates. These crates and
their authors are listed below.

",
  file = authfile
)

for (crate in crate_info) {
  authors = paste(crate$authors, collapse = ", ")
  if (authors == "") {
    authors = sprintf("%s contributors (see <%s> for details)", crate$package, crate$url)
  }

  cat(
    sprintf(
      "
## `%s`

- Version: %s
- Authors: %s
- License: %s
- Homepage: <%s>
",
      crate$package,
      crate$version,
      authors,
      crate$license,
      crate$url
    ),
    file = authfile
  )
}


close(authfile)

licenses = unique(unlist(lapply(crate_info, \(x) extract_licenses(x$license))))

for (lic in licenses) {
  cat(lic)
  download.file(
    sprintf("https://raw.githubusercontent.com/spdx/license-list-data/refs/heads/main/text/%s.txt", lic),
    sprintf("LICENSE.d/%s.txt", lic),
    "auto"
  )
}
