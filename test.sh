#!/bin/bash

GITHUB_OUTPUT="./errors.txt" cargo run -- \
		"${SPLUNK_HEC_HOSTNAME}" \
		"${SPLUNK_HEC_TOKEN}" \
		"{\"test\" : \"woo\", \"test_time\" : \"$(date +%Y-%m-%d-%H:%M:%S)\"}" \
		'' '' '' "${SPLUNK_HEC_PORT}"

cat errors.txt
