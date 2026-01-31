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

