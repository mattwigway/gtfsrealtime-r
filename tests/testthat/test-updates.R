test_that("can read updates", {
  file = system.file("nyc-trip-updates.pb.bz2", package = "gtfsrealtime")

  updates = read_gtfsrt_trip_updates(file)
  expect_s3_class(updates, "data.frame")
  expect_snapshot(head(updates))
})

test_that("updates give useful errors", {
  expect_error(
    read_gtfsrt_trip_updates("foo.pb"),
    regexp = "No such file or directory"
  )
})