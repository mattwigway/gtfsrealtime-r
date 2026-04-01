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
    regexp = "No such file or directory|The system cannot find the file specified"
  )
})

# This has Rust write out a feed that has every value of every enum, and then
# also return their expected order to make sure they match.
test_that("enums are correctly specified", {
  feed = tempfile()
  expected = test_data_enum_roundtrip_positions(feed)$ok
  actual = read_gtfsrt_positions(feed, "Etc/UTC")
  unlink(feed)

  expect_equal(as.character(actual$schedule_relationship), expected$schedule_relationship)
  expect_equal(as.character(actual$wheelchair_accessible), expected$wheelchair_accessible)
  expect_equal(as.character(actual$current_status), expected$current_status)
  expect_equal(as.character(actual$congestion_level), expected$congestion_level)
  expect_equal(as.character(actual$occupancy_status), expected$occupancy_status)
  # make sure there are no more enums we missed
  expect_equal(sum(sapply(actual, class) == "factor"), 5)
})