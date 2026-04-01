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

test_that("Louisville debug JSON matches read_gtfsrt_positions", {
  raw_expected = jsonlite::parse_json(gzfile(system.file("testdata/louisville-positions.json.gz", package = "gtfsrealtime")))

  # something missing from the JSON will be NULL, but then the column will be missing in the output
  null_to_na = function (x) {
    if (is.null(x)) {
      NA
    } else {
      x
    }
  }

  expected = purrr::map(raw_expected$Entities, function (entity) {
    p = entity$Vehicle
    tibble::tibble_row(
      id = null_to_na(entity$Id),
      latitude = null_to_na(p$Position$Latitude),
      longitude = null_to_na(p$Position$Longitude),
      bearing = null_to_na(p$Position$Bearing),
      odometer = null_to_na(p$Position$Odometer),
      speed = null_to_na(p$Position$Speed),
      trip_id = null_to_na(p$Trip$TripId),
      route_id = null_to_na(p$Trip$RouteId),
      direction_id = null_to_na(p$Trip$DirectionId),
      start_time = null_to_na(p$Trip$StartTime),
      start_date = null_to_na(p$Trip$StartDate),
      # this one is snake case for some reason
      schedule_relationship = null_to_na(p$Trip$schedule_relationship),
      stop_id = null_to_na(p$StopId),
      current_stop_sequence = null_to_na(p$CurrentStopSequence),
      current_status = null_to_na(p$CurrentStatus),
      timestamp = null_to_na(as.POSIXct(p$Timestamp, tz = "America/New_York")),
      congestion_level = null_to_na(p$congestion_level),
      occupancy_status = null_to_na(p$occupancy_status),
      occupancy_percentage = null_to_na(p$OccupancyPercentage),
      vehicle_id = null_to_na(p$Vehicle$Id),
      vehicle_label = null_to_na(p$Vehicle$Label),
      vehicle_license_plate = null_to_na(p$Vehicle$LicensePlate),
      wheelchair_accessible = null_to_na(p$Vehicle$WheelchairAccessible)
    )
  }) |> purrr::list_rbind()

  # don't label values, numeric labels used in JSON. Correct enum mapping tested above.
  actual = read_gtfsrt_positions(system.file("testdata/louisville-positions.pb.bz2", package = "gtfsrealtime"), "America/New_York", label_values = FALSE) |>
    # null_to_na makes logical vectors. so for columns where everything is NA, convert to logical
    dplyr::mutate(dplyr::across(dplyr::where(\(col) all(is.na(col))), \(col) as.logical(col)))|>
    tibble::as_tibble()

  expect_true(nrow(actual) > 0)
  expect_equal(actual, expected, tolerance = 1e-6)
})