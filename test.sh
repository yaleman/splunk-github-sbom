#!/bin/bash
if [ -f 'errors.txt' ]; then
	rm errors.txt
fi

if [ -z "${GITHUB_TOKEN}" ]; then
	echo "Please set the GITHUB_TOKEN env var"
	exit 1
fi
if [ -z "${SPLUNK_HEC_HOSTNAME}" ]; then
	echo "Please set the SPLUNK_HEC_HOSTNAME env var"
	exit 1
fi
if [ -z "${SPLUNK_HEC_TOKEN}" ]; then
	echo "Please set the SPLUNK_HEC_TOKEN env var"
	exit 1
fi
if [ -z "${SPLUNK_HEC_PORT}" ]; then
	echo "SPLUNK_HEC_PORT env var unset, defaulting to 8088"
	SPLUNK_HEC_PORT="8088"
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