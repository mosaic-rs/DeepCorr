# Loading library
# this will be gone when code is on CRAN

setwd("/Users/harrywoodhouse/CODE/deepcorr/bindings/R") # r bindings
#rextendr::register_()
rextendr::document() # creates R documentation for Rust code and R bindings
devtools::load_all() # loads functions i think?

hello()

Name <- "Harry"

echo(Name)

add(3, 4)
subtract(10, 3)
multiply(2, 100)
divide(1000, 2)
exponent(3, 3)

my_matrix <- matrix(rnorm(100), nrow = 10)
normalized <- normalize(my_matrix, method = "zscore", epsilon = 1e-6)
print(normalized)

mean(normalized)
sd(normalized)


# testing real formant data

library(dplyr)


df <- readxl::read_excel('/Users/harrywoodhouse/CODE/Linguistics as Data Science Code/Summative/sociophonetics_FACE_data.xlsx') # Outputs dataframe called df


df <- df %>% 
  rename(
    speaker = Speaker,
    # col 2 - 4 are good
    text = Text,
    target_transcript = `Target transcript`,
    segments_before = `Segment Before`,
    segments_after = `Segment After`,
    # F for formant can stay capitalized as that is how it is written 
    `F1_time_0.2` = `F1-time_0.2`,
    `F2_time_0.2` = `F2-time_0.2`
  )

df <- df %>%
  group_by(speaker) %>%
  mutate( # making columns where we get the mean of F1/F2 for that specific speaker
    F1_mean = mean(F1_time_0.2),
    F1_sd = sd(F1_time_0.2),
    F2_mean = mean(F2_time_0.2),
    F2_sd = sd(F2_time_0.2)
  ) %>%
  
  filter(
    # filtering out any values that are not within 2.5 SD
    
    F1_time_0.2 > (F1_mean - 2.5 * F1_sd) & F1_time_0.2 < (F1_mean + 2.5 * F1_sd),
    
    F2_time_0.2 > (F2_mean - 2.5 * F2_sd) & F2_time_0.2 < (F2_mean + 2.5 * F2_sd)
  )



# NORMALISATION TESTING
library(microbenchmark)

df <- df %>%
  group_by(speaker) %>%
  mutate(
    F1_normalized = (F1_time_0.2 - F1_mean) / F1_sd,
    F2_normalized = (F2_time_0.2 - F2_mean) / F2_sd
  )

df_normalized <- df %>%
  group_by(speaker) %>%
  mutate(
    # because R is the stupidest language ever, we have to convert the values to a Nx1 matrix 
    F1_normalized = normalize(as.matrix(F1_time_0.2), "minmax"),
    F2_normalized = normalize(as.matrix(F2_time_0.2), "minmax")
  ) %>%
  ungroup()

# TRIP N101
int_matrix <- matrix(1:4, nrow = 2)
result <- try(normalize(int_matrix, method = "zscore", epsilon = NULL))
print(result)

# TRIP N102
zero_row_matrix <- matrix(c(1, 1, 0, 1, 1, 0), nrow = 3)
result <- try(normalize(zero_row_matrix, method = "cosine", epsilon = NULL))

# TRIP N103 (Invalid Method)
try(normalize(matrix(as.numeric(1:4), 2), "garbage", NULL))






results_full <- microbenchmark(
  Manual_R_Full = {
    (df$F1_time_0.2 - mean(df$F1_time_0.2)) / sd(df$F1_time_0.2)
  },
  DeepCorr_Rust_Full = {
    normalize(as.matrix(df$F1_time_0.2), method = "zscore", epsilon = 1e-10)
  },
  times = 100
)

print(results)


zero_row_matrix <- matrix(c(1, 0, 3, 0), nrow = 2)

result <- try(normalize(zero_row_matrix, method = "cosine", epsilon = NULL))
