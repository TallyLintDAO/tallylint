#!/bin/bash
path=$(dirname "$0")
cd -P "${path}/../.." # P: physical directory structure and not to follow symbolic links.
path=$(pwd)
echo "${path}"
