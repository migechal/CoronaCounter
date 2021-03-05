data <- read.csv("cases.csv", header = TRUE)
positives <- data$positive[1]
cat(positives)