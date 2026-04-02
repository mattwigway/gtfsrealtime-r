# Alerts are hierarchical: a single alert can have multiple applicability periods, multiple affected entities, and translations to multiple languages. The read function flattens all of that to a tabular format, with one row for every combination of applicability, entity, and language.

This has four alerts. The first one has all fields filled out, and has
two each of applicability periods, entities informed, and language - so
it should become 2*2*2 = 8 rows. The second one is identical but is
missing some (but not all) Spanish translations The third one does not
have Spanish translations, so it should become 2\*2 = 4 rows The fourth
one has no time ranges, entities, or languages so should just be one row
the fifth one is all NA

## Usage

``` r
test_data_alert_unwrapping(filename)
```
