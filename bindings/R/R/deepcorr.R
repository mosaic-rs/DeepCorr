#' Normalize Data
#' @export
normalize <- function(data, method, epsilon = NULL) {
  res <- normalize_native(data, method, epsilon)
    if (is.character(res) && length(res) == 1 && grepl("^\\[N\\d{3}\\]", res)) {
    
    red_bold <- "\033[1;31m"
    blue_under <- "\033[4;34m"
    reset <- "\033[0m"
    
    parts <- strsplit(res, "\nWiki: ") [[1]]
    header_msg <- parts[1]
    wiki_url <- parts[2]
    
    formatted_msg <- paste0(
      "\n", red_bold, header_msg, reset, 
      "\nWiki: ", blue_under, wiki_url, reset, "\n"
    )
    stop(formatted_msg, call. = FALSE)
  } 
  return(res)
}