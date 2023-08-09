#!/usr/bin/env python3
# A script to convert SSH URLs to HTTP URLs from git remote -v output

import subprocess
import re

# Get the output of git remote -v and store it in a variable
output = subprocess.check_output(["git", "remote", "-v"]).decode()

# Loop through each line of the output
for line in output.splitlines():
  # Check if the line contains an SSH URL
  if "git@" in line:
    # Replace the git@ part with https:// and the : part with /
    http_url = line.replace("git@", "https://")
    http_url = http_url.replace(":", "/")
    # Print the converted URL
    print(http_url)
  else:
    # If the line does not contain an SSH URL, print it as it is
    print(line)
