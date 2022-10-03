#!/usr/bin/env bash

set -x
set -o pipefail

while true; do
  echo "Running init_db.sh ...";

  ./scripts/init_db.sh;
  if [ $? -eq 0 ]; then
    echo "Done"
    break;
  fi;

  echo "An error occurred; retrying in 5 seconds...";
  sleep 5;
done
