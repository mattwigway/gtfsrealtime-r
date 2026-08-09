setwd(here::here("vignettes"))
knitr::knit(
  here::here("vignette-prerender/performance.Rmd"),
  output = here::here("vignettes/articles/performance.Rmd")
)


knitr::knit(
  here::here("vignette-prerender/archived.Rmd"),
  output = here::here("vignettes/articles/archived.Rmd")
)
