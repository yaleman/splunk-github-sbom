#!/bin/bash
if [ -f 'errors.txt' ]; then
	rm errors.txt
fi

# find the logs with source=splunk-github-sbom-test

INPUT_GITHUB_TOKEN="${GITHUB_TOKEN}" GITHUB_OUTPUT="./errors.txt" cargo run -- \
		"${GITHUB_TOKEN}" \
		"${SPLUNK_HEC_HOSTNAME}" \
		"${SPLUNK_HEC_TOKEN}" \
		'' \
		'' \
		'splunk-github-sbom-test' \
		"${SPLUNK_HEC_PORT}"  \
		"yaleman/$(basename "$(pwd)")"
if [ -f 'errors.txt' ]; then
	cat errors.txt
fi