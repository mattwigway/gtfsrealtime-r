test_that("can read updates", {
  file = system.file("nyc-trip-updates.pb.bz2", package = "gtfsrealtime")

  updates = read_gtfsrt_trip_updates(file, "America/New_York")
  expect_s3_class(updates, "data.frame")
  expect_snapshot(head(updates))
})

test_that("updates give useful errors", {
  expect_error(
    read_gtfsrt_trip_updates("foo.pb", "America/New_York"),
    regexp = "No such file or directory|The system cannot find the file specified"
  )
})


test_that("invalid timezone leads to failure", {
  file = system.file("nyc-trip-updates.pb.bz2", package = "gtfsrealtime")

  # no such timezone
  expect_error(
    read_gtfsrt_trip_updates(file, "America/Chapel_Hill"),
    regexp = "Invalid time zone"
  )
})

test_that("timezones work", {
  file = tempfile()
  # not using the NYC test data here b/c it does not have any scheduled_times
  test_data_update_unwrapping(file)

  local_time = read_gtfsrt_trip_updates(file, "Europe/Paris")
  utc_time = read_gtfsrt_trip_updates(file, "Etc/UTC")
  unlink(file)

  # If you just subtract the times from each other you get zero, because R
  # correctly handles the time zone difference and sees that they are the
  # same time. So extract the hour component and make sure that it is off
  # by -1 hours (Paris -> UTC)
  for (col in c("arrival_time", "arrival_scheduled_time", "departure_time", "departure_scheduled_time")) {
    expect_equal(is.na(utc_time[[col]]), is.na(local_time[[col]]))

    expect_all_equal(
      (lubridate::hour(utc_time[[col]]) - lubridate::hour(local_time[[col]]))[!is.na(utc_time[[col]])],
      -2
    )

    # but the times should in fact be equivalent, just in different time zones
    expect_all_equal(
      (utc_time[[col]] - local_time[[col]])[!is.na(utc_time[[col]])],
      as.difftime(0, units = "secs")
    )
  }
})

# This has Rust write out a feed that has every value of every enum, and then
# also return their expected order to make sure they match.
test_that("enum roundtrip is correct", {
  file = tempfile()
  expected = test_data_enum_roundtrip_updates(file)$ok
  actual = read_gtfsrt_trip_updates(file, "Australia/Sydney")
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
  rt = read_gtfsrt_trip_updates(file, "Australia/Sydney")
  unlink(file)

  # id 1 should become two rows, so all update-level things should have been duplicated
  expect_equal(nrow(rt), 6)
  expect_equal(rt$id, c("1", "1", "2", "3", "4", "5"))
  expect_equal(rt$trip_id, c("one", "one", "two", "three", NA, NA))
  expect_equal(rt$route_id, c("rte1", "rte1", "rte2", "rte3", NA, NA))
  expect_equal(rt$start_time, c("06:00:00", "06:00:00", "06:00:02", "06:00:03", NA, NA))
  expect_equal(rt$start_date, c("20260401", "20260401", "20260402", "20260403", NA, NA))
  expect_equal(as.character(rt$trip_schedule_relationship), c("SCHEDULED", "SCHEDULED", "ADDED", "SCHEDULED", NA, NA))
  expect_equal(rt$vehicle_id, c("veh1", "veh1", "veh2", NA, NA, NA))
  expect_equal(rt$vehicle_label, c("lab1", "lab1", "lab2", NA, NA, NA))
  expect_equal(rt$license_plate, c("PLA-0001", "PLA-0001", "PLA-0002", NA, NA, NA))
  expect_equal(
    as.character(rt$wheelchair_accessible),
    c("WHEELCHAIR_ACCESSIBLE", "WHEELCHAIR_ACCESSIBLE", "NO_VALUE", NA, NA, NA)
  )
  expect_equal(rt$stop_sequence, c(2, 4, 1, NA, NA, NA))
  expect_equal(rt$stop_id, c("stop1", "stop2", "stop1_2", NA, NA, NA))
  expect_equal(rt$arrival_delay, c(5, 10, 12, NA, NA, NA))
  expect_equal(rt$arrival_time, as.POSIXct(c(1775059604, 1775059704, 1775058604, NA, NA, NA), "Australia/Sydney"))
  expect_equal(rt$arrival_scheduled_time, as.POSIXct(c(1775059504, NA, NA, NA, NA, NA), "Australia/Sydney"))
  expect_equal(rt$arrival_uncertainty, c(35, 37, 30, NA, NA, NA))
  expect_equal(rt$departure_delay, c(25, 30, 20, NA, NA, NA))
  expect_equal(rt$departure_time, as.POSIXct(c(1775059624, 1775059724, 1775058624, NA, NA, NA), "Australia/Sydney"))
  expect_equal(rt$departure_scheduled_time, as.POSIXct(c(1775059524, NA, NA, NA, NA, NA), "Australia/Sydney"))
  expect_equal(rt$departure_uncertainty, c(25, 27, 24, NA, NA, NA))
})

test_that("updates match debug json", {
  # A number of agencies, for example Louisville, provide GTFS-realtime in both
  # protocol buffers and the much less efficient JSON format for debugging. Here
  # we make sure what we read from the PB matches the JSON.
  raw_expected = jsonlite::parse_json(gzfile(system.file(
    "testdata/louisville-updates.json.gz",
    package = "gtfsrealtime"
  )))

  # something missing from the JSON will be NULL, but then the column will be missing in the output
  null_to_na = function(x) {
    if (is.null(x)) {
      NA
    } else {
      x
    }
  }

  # flatten to a table, one row per trip update
  expected_trip_updates = purrr::map(raw_expected$Entities, function(entity) {
    u = entity$TripUpdate
    tibble::tibble_row(
      id = null_to_na(entity$Id),
      trip_id = null_to_na(u$Trip$TripId),
      route_id = null_to_na(u$Trip$RouteId),
      direction_id = null_to_na(u$Trip$DirectionId),
      start_time = null_to_na(u$Trip$StartTime),
      start_date = null_to_na(u$Trip$StartDate),
      trip_schedule_relationship = null_to_na(u$Trip$schedule_relationship), # who know why this is different case in JSON...
      modifications_id = null_to_na(u$Trip$ModificationsId), # TODO why are we even reading this? It's experimental
      vehicle_id = null_to_na(u$Vehicle$Id),
      vehicle_label = null_to_na(u$Vehicle$Label),
      license_plate = null_to_na(u$Vehicle$LicensePlate),
      wheelchair_accessible = null_to_na(u$Vehicle$WheelchairAccessible)
    )
  }) |>
    purrr::list_rbind()

  # make stop time updates, one row per stoptimeupdate
  expected_stop_time_updates = purrr::map(raw_expected$Entities, function(entity) {
    purrr::map(entity$TripUpdate$StopTimeUpdates, function(s) {
      tibble::tibble_row(
        id = null_to_na(entity$Id),
        stop_sequence = null_to_na(s$StopSequence),
        stop_id = null_to_na(s$StopId),
        arrival_delay = null_to_na(s$Arrival$Delay),
        arrival_time = null_to_na(s$Arrival$Time),
        arrival_scheduled_time = null_to_na(s$Arrival$ScheduledTime),
        arrival_uncertainty = null_to_na(s$Arrival$Uncertainty),
        departure_delay = null_to_na(s$Departure$Delay),
        departure_time = null_to_na(s$Departure$Time),
        departure_scheduled_time = null_to_na(s$Departure$ScheduledTime),
        departure_uncertainty = null_to_na(s$Departure$Uncertainty),
        departure_occupancy_status = null_to_na(s$DepartureOccupancyStatus),
        stop_schedule_relationship = null_to_na(s$schedule_relationship)
      )
    }) |>
      purrr::list_rbind()
  }) |>
    purrr::list_rbind() |>
    dplyr::mutate(dplyr::across(tidyselect::ends_with("time"), function(x) {
      if (!all(is.na(x))) {
        as.POSIXct(x, "America/New_York")
      } else {
        # if they are all NAs, don't change the column type, because all of the all NA
        # columns are converted to logical when loading the data.
        x
      }
    }))

  # left join will duplicate each trip update for all of its stop time updates, and leave trip updates without stop time updates
  # in with all NAs in stop time fields
  expected = dplyr::left_join(expected_trip_updates, expected_stop_time_updates, by = "id")

  # In the Louisville JSON, the enums are represented as their underlying numbers. Correct enum
  # mapping is ensured by the roundtrip tests
  actual = read_gtfsrt_trip_updates(
    system.file("testdata/louisville-updates.pb.bz2", package = "gtfsrealtime"),
    "America/New_York",
    label_values = FALSE
  ) |>
    # null_to_na makes logical vectors. so for columns where everything is NA, convert to logical
    dplyr::mutate(dplyr::across(dplyr::where(\(col) all(is.na(col))), \(col) as.logical(col))) |>
    tibble::as_tibble()

  expect_equal(nrow(actual), 15216)
  expect_equal(actual, expected)
})
