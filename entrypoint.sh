#!/bin/sh
set -e
REPORT_TMP_DIR=/tmp/results
REPORT_DEST_DIR=${REPORT_DEST_DIR:-/tmp/reports_dest}

rm $REPORT_DEST_DIR/*.xml || true
mkdir $REPORT_TMP_DIR

REPORTS_DIR=$REPORT_TMP_DIR cargo test --test xchange
cp -r $REPORT_TMP_DIR/* $REPORT_DEST_DIR
