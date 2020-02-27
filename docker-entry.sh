#!/bin/bash

./wait-for-it.sh database:5432

if [ $? -ne 0 ]; then
  echo "database wait error"
  exit
fi

if [ ! -f .init ]; then
  echo "running migrations"
  diesel migration run
  touch .init
fi

postman
