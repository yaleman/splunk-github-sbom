.PHONY: test
test:
	./test.sh

.PHONY: build
build: ## Build the docker image
	docker build -t splunk-github-sbom .
