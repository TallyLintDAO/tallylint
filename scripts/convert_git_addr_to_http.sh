#!/bin/bash
# A script to convert SSH URLs to HTTP URLs from git remote -v output

# Get the output of git remote -v and store it in a variable
output=$(git remote -v)

# Loop through each line of the output
while read -r line; do
  # Check if the line contains an SSH URL
  if [[ $line == *"git@"* ]]; then
    # Replace the git@ part with https:// and the : part with /
    http_url=${line/git@/https:\/\/}
    http_url=${http_url/:/\/}
    # Replace the // part with a single /
    http_url=${http_url/\/\//\/}
    # Replace the https:/ part with https://
    http_url=${http_url/https:\//https:\/\/}
    # Print the converted URL
    echo "$http_url"
  else
    # If the line does not contain an SSH URL, print it as it is
    echo "$line"
  fi
done <<< "$output"
