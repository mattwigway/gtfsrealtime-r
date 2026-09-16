# differential feeds are officially unsupported but will give unexpected results if they exist. throw an error
test_that("differential feed generates error", {
  # if we encounter one.
  feed = tempfile()
  test_data_differential_feed(feed)

  expect_error(
    {
      read_gtfsrt_alerts(feed, "America/New_York")
    },
    regexp = "Differential GTFS-realtime feeds are not supported"
  )

  expect_error(
    {
      read_gtfsrt_positions(feed, "America/New_York")
    },
    regexp = "Differential GTFS-realtime feeds are not supported"
  )

  expect_error(
    {
      read_gtfsrt_trip_updates(feed, "America/New_York")
    },
    regexp = "Differential GTFS-realtime feeds are not supported"
  )

  unlink(feed)
})
