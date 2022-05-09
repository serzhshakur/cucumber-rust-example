#!/bin/sh
set -e

var=false

exists() {
  if ls /app/allure-results/*.xml &> /dev/null; then
    var=true
  fi
}

until test $var = true ; do
  >&2 echo "no report file is available - waiting..."
  sleep 1
  exists
done

allure serve -p 8080
