#!/bin/bash

# Get the directory of the root package.json
ROOT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && cd .. && pwd )"

# Change to the root directory
cd "$ROOT_DIR" || exit

# Check if package.json exists
if [ ! -f package.json ]; then
  echo "Error: package.json not found in the root directory."
  exit 1
fi

# Get the list of workspaces from package.json
WORKSPACES=$(jq -r '.workspaces | .[]' < package.json)

# Iterate over each workspace and run npm test
for workspace in $WORKSPACES; do
  echo "Running tests in workspace: $workspace"
  (cd "$workspace" && npm test)
done
