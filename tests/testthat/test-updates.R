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

# update with multiple stop times become multiple rows, those with 1 or none do not
test_that("updates are unwrapped correctly", {
  file = tempfile()
  test_data_update_unwrapping(file)
  rt = read_gtfsrt_trip_updates(file)
  unlink(file)

  # id 1 should become two rows, so all update-level things should have been duplicated
  expect_equal(nrow(rt), 4)
  expect_equal(rt$id, c("1", "1", "2", "3"))
  expect_equal(rt$trip_id, c("one", "one", "two", "three"))
  expect_equal(rt$route_id, c("rte1", "rte1", "rte2", "rte3"))
  expect_equal(rt$start_time, c("06:00:00", "06:00:00", "06:00:02", "06:00:03"))
  expect_equal(rt$start_date, c("20260401", "20260401", "20260402", "20260403"))
  expect_equal(as.character(rt$trip_schedule_relationship), c("SCHEDULED", "SCHEDULED", "ADDED", "SCHEDULED"))
  expect_equal(rt$vehicle_id, c("veh1", "veh1", "veh2", NA))
  expect_equal(rt$vehicle_label, c("lab1", "lab1", "lab2", NA))
  expect_equal(rt$license_plate, c("PLA-0001", "PLA-0001", "PLA-0002", NA))
  expect_equal(as.character(rt$wheelchair_accessible), c("WHEELCHAIR_ACCESSIBLE", "WHEELCHAIR_ACCESSIBLE", "NO_VALUE", NA))
  expect_equal(rt$stop_sequence, c(2, 4, 1, NA))
  expect_equal(rt$stop_id, c("stop1", "stop2", "stop1_2", NA))
  expect_equal(rt$arrival_delay, c(5, 10, 12, NA))
  expect_equal(rt$arrival_time, c(1775059604, 1775059704, 1775058604, NA))
  expect_equal(rt$arrival_uncertainty, c(35, 37, 30, NA))
  expect_equal(rt$departure_delay, c(25, 30, 20, NA))
  expect_equal(rt$departure_time, c(1775059624, 1775059724, 1775058624, NA))
  expect_equal(rt$departure_uncertainty, c(25, 27, 24, NA))
})