test_that("can read positions", {
  file = system.file("nyc-vehicle-positions.pb.bz2", package = "gtfsrealtime")

  positions = read_gtfsrt_positions(file, "America/New_York")
  expect_s3_class(positions, "data.frame")
  expect_snapshot(head(positions))
})

test_that("invalid timezone leads to failure", {
  file = system.file("nyc-vehicle-positions.pb.bz2", package = "gtfsrealtime")

  # no such timezone
  expect_error(
    read_gtfsrt_positions(file, "America/Chapel_Hill"),
    regexp = "Invalid time zone"
  )
})

test_that("timezones work", {
  file = system.file("nyc-vehicle-positions.pb.bz2", package = "gtfsrealtime")

  local_time = read_gtfsrt_positions(file, "America/New_York")
  utc_time = read_gtfsrt_positions(file, "Etc/UTC")

  # If you just subtract the times from each other you get zero, because R
  # correctly handles the time zone difference and sees that they are the
  # same time. So extract the hour component and make sure that it is off
  # by five hours (NYC -> UTC)
  expect_all_equal(
    lubridate::hour(utc_time$timestamp) - lubridate::hour(local_time$timestamp),
    5
  )

  # but the times should in fact be equivalent, just in different time zones
  expect_all_equal(
    utc_time$timestamp - local_time$timestamp,
    as.difftime(0, units="secs")
  )
})

test_that("error handling works", {
  expect_error(
    read_gtfsrt_positions("foo.pb", "America/New_York"),
    regexp = "No such file or directory"
  )
})