setwd(here::here("vignettes"))
knitr::knit(
    here::here("vignette-prerender/archived.Rmd"),
    output=here::here("vignettes/archived.Rmd")
)
