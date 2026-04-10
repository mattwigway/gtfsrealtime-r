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
    as.difftime(0, units = "secs")
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
  expect_equal(as.character(actual$vehicle_wheelchair_accessible), expected$vehicle_wheelchair_accessible)
  expect_equal(as.character(actual$current_status), expected$current_status)
  expect_equal(as.character(actual$congestion_level), expected$congestion_level)
  expect_equal(as.character(actual$occupancy_status), expected$occupancy_status)
  # make sure there are no more enums we missed
  expect_equal(sum(sapply(actual, class) == "factor"), 5)
})

test_that("Louisville debug JSON matches read_gtfsrt_positions", {
  raw_expected = jsonlite::parse_json(gzfile(system.file(
    "testdata/louisville-positions.json.gz",
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

  expected = purrr::map(raw_expected$Entities, function(entity) {
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
      vehicle_wheelchair_accessible = null_to_na(p$Vehicle$WheelchairAccessible)
    )
  }) |>
    purrr::list_rbind()

  # don't label values, numeric labels used in JSON. Correct enum mapping tested above.
  actual = read_gtfsrt_positions(
    system.file("testdata/louisville-positions.pb.bz2", package = "gtfsrealtime"),
    "America/New_York",
    label_values = FALSE
  ) |>
    # null_to_na makes logical vectors. so for columns where everything is NA, convert to logical
    dplyr::mutate(dplyr::across(dplyr::where(\(col) all(is.na(col))), \(col) as.logical(col))) |>
    tibble::as_tibble()

  expect_true(nrow(actual) > 0)
  expect_equal(actual, expected, tolerance = 1e-6)
})

# The other types have unwrapping tests that incidentally test that fields are getting read properly.
# Since there's no unwrapping in positions, test separately.
# This tests a three item feed, one where everything is filled out, one where all optional fields
# are missing but the overall structure is there, and one where all optional structural elements
# (e.g. Vehicle) are missing
test_that("all columns read correctly", {
  expected = rbind(
    tibble::tibble_row(
      id = "1",
      latitude = 37.363,
      longitude = -122.123,
      bearing = 78,
      odometer = 8675809,
      speed = 45,
      trip_id = "trip",
      route_id = "route",
      direction_id = 1,
      start_time = "07:00:00",
      start_date = "20260401",
      schedule_relationship = "ADDED",
      stop_id = "stop",
      current_stop_sequence = 10,
      current_status = "STOPPED_AT",
      timestamp = lubridate::ymd_hms("2026-04-01 16:48:04", tz = "America/New_York"),
      congestion_level = "SEVERE_CONGESTION",
      occupancy_status = "CRUSHED_STANDING_ROOM_ONLY",
      occupancy_percentage = 15,
      vehicle_id = "42",
      vehicle_label = "label",
      vehicle_license_plate = "LIC-4242",
      vehicle_wheelchair_accessible = "WHEELCHAIR_ACCESSIBLE"
    ),

    # NAs because individual items are missing
    tibble::tibble_row(
      id = "2",
      latitude = 37.363,
      longitude = -122.123,
      bearing = NA,
      odometer = NA,
      speed = NA,
      trip_id = NA,
      route_id = NA,
      direction_id = NA,
      start_time = NA,
      start_date = NA,
      schedule_relationship = NA,
      stop_id = NA,
      current_stop_sequence = NA,
      current_status = NA,
      timestamp = NA,
      congestion_level = NA,
      occupancy_status = NA,
      occupancy_percentage = NA,
      vehicle_id = NA,
      vehicle_label = NA,
      vehicle_license_plate = NA,
      vehicle_wheelchair_accessible = NA
    ),

    # NAs because structure is missing
    tibble::tibble_row(
      id = "3",
      latitude = NA,
      longitude = NA,
      bearing = NA,
      odometer = NA,
      speed = NA,
      trip_id = NA,
      route_id = NA,
      direction_id = NA,
      start_time = NA,
      start_date = NA,
      schedule_relationship = NA,
      stop_id = NA,
      current_stop_sequence = NA,
      current_status = NA,
      timestamp = NA,
      congestion_level = NA,
      occupancy_status = NA,
      occupancy_percentage = NA,
      vehicle_id = NA,
      vehicle_label = NA,
      vehicle_license_plate = NA,
      vehicle_wheelchair_accessible = NA
    )
  )

  file = tempfile()
  test_data_positions_all_values(file)
  actual = read_gtfsrt_positions(file, "America/New_York") |>
    tibble::as_tibble() |>
    dplyr::mutate(dplyr::across(dplyr::where(is.factor), as.character))
  unlink(file)

  expect_equal(actual, expected, tolerance = 1e-4)
})

test_that("duplicate ids are deduplicated", {
  # expect_warning doesn't capture warnings issued in R! macros in Rust code.
  # so we mock cli_warn and capture the results
  warnings = list(warnings = list())
  local_mocked_bindings(cli_warn = function(x) warnings$warnings <<- append(warnings$warnings, x), .package = "cli")

  file = tempfile()
  test_data_duplicate_ids_positions(file)
  pos = read_gtfsrt_positions(file, "America/New_York")
  unlink(file)

  expect_equal(
    warnings$warnings,
    # the c("!" = ... gets unwrapped when appended to a list, and then the list has two duplicate elements,
    # which somehow R is okay with (?)
    list(
      "!" = 'ID )); stop("identifier with r code executed!")# is duplicated. Replacing with )); stop("identifier with r code executed!")#_duplicated_1',
      "!" = 'ID )); stop("identifier with r code executed!")# is duplicated. Replacing with )); stop("identifier with r code executed!")#_duplicated_2'
    )
  )

  # it should have been deduplicate
  expect_equal(
    pos$id,
    c(
      ")); stop(\"identifier with r code executed!\")#",
      ")); stop(\"identifier with r code executed!\")#_duplicated_1",
      ")); stop(\"identifier with r code executed!\")#_duplicated_2"
    )
  )
})
