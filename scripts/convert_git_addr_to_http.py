#!/usr/bin/env python3
# A script to convert SSH URLs to HTTP URLs from git remote -v output

import subprocess
import re

# Define a regular expression pattern to match SSH URLs
pattern = r"git@([^:]+):(.+)\.git"

# Define a function to convert SSH URLs to HTTP URLs
def convert(url):
  # Use the re.search method to find the matching groups in the URL
  match = re.search(pattern, url)
  # If there is a match, return the converted URL with https:// and /
  if match:
    return f"https://{match.group(1)}/{match.group(2)}"
  # Otherwise, return the original URL
  else:
    return url

# Get the output of git remote -v and store it in a variable
output = subprocess.check_output(["git", "remote", "-v"]).decode()

# Loop through each line of the output
for line in output.splitlines():
  # Split the line by whitespace and get the first and second elements
  name, url = line.split()[:2]
  # Print the name and the converted URL
  print(name, convert(url))
