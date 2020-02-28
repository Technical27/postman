#!/bin/bash

./wait-for-it.sh database:5432
waitStatus=$?

if [[ $waitStatus -ne 0 ]]; then
  echo "database wait error: $waitStatus"
  exit
fi

if [[ -f backup.sql ]]; then
  echo "restoring database"
  cat backup.sql | psql -U postman
  backupStatus=$?
  rm backup.sql
  if [[ $backupStatus -ne 0 ]]; then
    echo "error while restoring database: $backupStatus"
    exit
  fi
fi

if [ ! -f .init ]; then
  echo "running migrations"
  diesel migration run
  touch .init
fi

postman
