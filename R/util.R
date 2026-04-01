# create a factor without introducing NAs by coercion
enum_to_factor = function (values, enum) {
    if (!is.null(enum$err)) { # this should not happen
        cli_abort(enum$err)
    } else {
        enum = enum$ok
    }

    levels = enum$levels
    labels = enum$labels

    if (!all(is.na(values) | values %in% levels)) {
        unknown_vals = unique(values[!(values %in% levels)])
        typ = sub("^gtfsrealtime::transit_realtime::", "", enum$typ)
        cli_warn(c(
            "Unknown values of {.val {typ}}: {.val {unknown_vals}}",
            ">" = "known values: {.val {enum$levels}} ({.val {enum$labels}})",
            "i" = "These likely represent newer or local GTFS-rt extensions, and will be presented as stringified numbers.",
            "v" = "If these values are standard or widely-used, report an issue at {.url https://github.com/mattwigway/gtfsrealtime-r/issues}"
        ))

        # add them, without names
        levels = c(levels, unknown_vals)
        labels = c(labels, as.character(unknown_vals))
    }

    factor(
        values,
        levels=levels,
        labels=labels
    )
}