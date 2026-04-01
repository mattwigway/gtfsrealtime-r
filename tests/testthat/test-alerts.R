test_that("can read alerts", {
  file = system.file("nyc-service-alerts.pb.bz2", package = "gtfsrealtime")

  alerts = read_gtfsrt_alerts(file)
  expect_s3_class(alerts, "data.frame")
  expect_snapshot(head(alerts))
})

test_that("alerts give useful errors", {
  expect_error(
    read_gtfsrt_alerts("foo.pb"),
    regexp = "No such file or directory|The system cannot find the file specified"
  )
})


# This has Rust write out a feed that has every value of every enum, and then
# also return their expected order to make sure they match.
test_that("enums are correctly specified", {
  feed = tempfile()
  expected = test_data_enum_roundtrip_alerts(feed)$ok
  actual = read_gtfsrt_alerts(feed)
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
  actual = read_gtfsrt_alerts(feed)
  unlink(feed)

  # four alerts get expanded to 8 + 8 + 4 + 1 = 21
  expect_equal(nrow(actual), 21)
})
