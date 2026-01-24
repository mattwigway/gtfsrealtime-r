enum_schedule_relationship = function (x) {
    no_coerce_factor(
        x,
        # note intentionally no 4
        levels = c(0, 1, 2, 3, 5, 6, 7, 8),
        labels = c(
            "SCHEDULED", # 0;
            "ADDED", # 1 [deprecated", # true];
            "UNSCHEDULED", # 2;
            "CANCELED", # 3;
            "REPLACEMENT", # 5;
            "DUPLICATED", # 6;
            "DELETED", # 7;
            "NEW" # 8;
        )
    )
}

enum_cause = function (x) {
    no_coerce_factor(
        x,
        levels = c(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12),
        labels = c(
            "UNKNOWN_CAUSE",
            "OTHER_CAUSE",
            "TECHNICAL_PROBLEM",
            "STRIKE",
            "DEMONSTRATION",
            "ACCIDENT",
            "HOLIDAY",
            "WEATHER",
            "MAINTENANCE",
            "CONSTRUCTION",
            "POLICE_ACTIVITY",
            "MEDICAL_EMERGENCY"
        )
    )
}

enum_severity_level = function (x) {
    no_coerce_factor(
        x,
        levels = c(1, 2, 3, 4),
        labels = c(
            "UNKNOWN_SEVERITY",
            "INFO",
            "WARNING",
            "SEVERE"
        )
    )
  }
