test_that("Invalid enum values work", {
  tmp = tempfile()
  # This has two single vehicle position entries, with a schedule relationship of 
  test_data_invalid_enum_positions(tmp)

  expect_warning(
    {data = read_gtfsrt_positions(tmp, "America/New_York")},
    regex = "Unknown values of.*trip_descriptor::ScheduleRelationship.*256"
  )

  expect_true(all(data$schedule_relationship == c("256", "SCHEDULED")))

  unlink(tmp)
})
