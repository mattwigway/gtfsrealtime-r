# create a factor without introducing NAs by coercion
no_coerce_factor = function (values, levels, labels) {
    if (!all(is.na(values) | values %in% levels)) {
        stop("Not all values appear in levels")
    }

    factor(
        values,
        levels=levels,
        labels=labels
    )
}