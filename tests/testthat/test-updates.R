test_that("can read updates", {
  file = system.file("nyc-trip-updates.pb.bz2", package = "gtfsrealtime")

  updates = read_gtfsrt_trip_updates(file)
  expect_s3_class(updates, "data.frame")
  expect_snapshot(head(updates))
})

test_that("updates give useful errors", {
  expect_error(
    read_gtfsrt_trip_updates("foo.pb"),
    regexp = "No such file or directory|The system cannot find the file specified"
  )
})

# This has Rust write out a feed that has every value of every enum, and then
# also return their expected order to make sure they match.
test_that("enum roundtrip is correct", {
  file = tempfile()
  expected = test_data_enum_roundtrip_updates(file)$ok
  actual = read_gtfsrt_trip_updates(file)
  unlink(file)

  expect_equal(as.character(actual$trip_schedule_relationship), expected$trip_schedule_relationship)
  expect_equal(as.character(actual$wheelchair_accessible), expected$wheelchair_accessible)
  expect_equal(as.character(actual$departure_occupancy_status), expected$departure_occupancy_status)
  expect_equal(as.character(actual$stop_schedule_relationship), expected$stop_schedule_relationship)
   # make sure there are no more enums we missed
  expect_equal(sum(sapply(actual, class) == "factor"), 4)
})