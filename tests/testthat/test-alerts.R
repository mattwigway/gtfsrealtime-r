test_that("can read alerts", {
  file = system.file("nyc-service-alerts.pb.bz2", package = "gtfsrealtime")

  alerts = read_gtfsrt_alerts(file, "America/New_York")
  expect_s3_class(alerts, "data.frame")
  expect_snapshot(head(alerts))
})

test_that("alerts give useful errors", {
  expect_error(
    read_gtfsrt_alerts("foo.pb", "America/New_York"),
    regexp = "No such file or directory|The system cannot find the file specified"
  )
})


test_that("invalid timezone leads to failure", {
  file = system.file("nyc-service-alerts.pb.bz2", package = "gtfsrealtime")

  # no such timezone
  expect_error(
    read_gtfsrt_alerts(file, "America/Chapel_Hill"),
    regexp = "Invalid time zone"
  )
})

test_that("timezones work", {
  file = system.file("nyc-service-alerts.pb.bz2", package = "gtfsrealtime")

  local_time = read_gtfsrt_alerts(file, "America/New_York")
  utc_time = read_gtfsrt_alerts(file, "Etc/UTC")

  # If you just subtract the times from each other you get zero, because R
  # correctly handles the time zone difference and sees that they are the
  # same time. But they're not consistently a five hour offset, because
  # NYC has alerts starting at a bunch of different times, and some are
  # during DST.
  expect_equal(is.na(utc_time$start), is.na(local_time$start))

  expect_equal(
    lubridate::hour(lubridate::with_tz(utc_time$start, "America/New_York")),
    lubridate::hour(local_time$start)
  )

  expect_equal(is.na(utc_time$end), is.na(local_time$end))

  expect_equal(
    lubridate::hour(lubridate::with_tz(utc_time$end, "America/New_York")),
    lubridate::hour(local_time$end)
  )

  # but the times should in fact be equivalent, just in different time zones
  expect_all_equal(
    (utc_time$start - local_time$start)[!is.na(utc_time$start)],
    as.difftime(0, units = "secs")
  )

  expect_all_equal(
    (utc_time$end - local_time$end)[!is.na(utc_time$end)],
    as.difftime(0, units = "secs")
  )
})

# This has Rust write out a feed that has every value of every enum, and then
# also return their expected order to make sure they match.
test_that("enums are correctly specified", {
  feed = tempfile()
  expected = test_data_enum_roundtrip_alerts(feed)$ok
  actual = read_gtfsrt_alerts(feed, "America/New_York")
  unlink(feed)

  expect_equal(as.character(actual$trip_schedule_relationship), expected$trip_schedule_relationship)
  expect_equal(as.character(actual$cause), expected$cause)
  expect_equal(as.character(actual$effect), expected$effect)
  expect_equal(as.character(actual$severity_level), expected$severity_level)
  # make sure there are no more enums we missed
  expect_equal(sum(sapply(actual, class) == "factor"), 4)
})

test_that("unwrapping works", {
  feed = tempfile()
  test_data_alert_unwrapping(feed)$ok
  actual = read_gtfsrt_alerts(feed, "America/New_York") |>
    # convert factors to char for comparisons
    dplyr::mutate(dplyr::across(dplyr::where(is.factor), as.character)) |>
    dplyr::arrange(id, start, end, agency_id, route_id, trip_trip_id, language)
  unlink(feed)

  # four alerts get expanded to 8 + 8 + 4 + 1 + 1 = 22
  expect_equal(nrow(actual), 22)

  # There are too many fields to hard code them all here, so I read it once
  # and validated it manually. Read that validated version.
  expected = read.csv(system.file("testdata/alerts_decoded.csv", package = "gtfsrealtime")) |>
    dplyr::arrange(id, start, end, agency_id, route_id, trip_trip_id, language) |>
    dplyr::mutate(
      id = as.character(id),
      start = as.POSIXct(start, tz = "America/New_York"),
      end = as.POSIXct(end, tz = "America/New_York"),
      trip_start_date = as.character(trip_start_date),
      trip_modification_id = as.character(trip_modification_id)
    )

  expect_equal(actual, expected)
})


test_that("id deduplication works", {
  # expect_warning doesn't capture warnings issued in R! macros in Rust code.
  # so we mock cli_warn and capture the results
  warnings = list(warnings = list())
  local_mocked_bindings(cli_warn = function(x) warnings$warnings <<- append(warnings$warnings, x), .package = "cli")

  file = tempfile()
  test_data_duplicate_ids_alerts(file)
  upd = read_gtfsrt_alerts(file, "America/New_York")
  unlink(file)

  expect_equal(
    warnings$warnings,
    list(
      "!" = "ID id is duplicated. Replacing with id_duplicated_1"
    )
  )

  # The second trip update (with the duplicated ID) has two stop time updates
  expect_equal(upd$id, c("id", "id_duplicated_1", "id_duplicated_1", "id2"))
})
