#!/bin/bash

if [ ! -f .init ]; then
  echo "running migrations"
  diesel migration run
  touch .init
fi

postman
