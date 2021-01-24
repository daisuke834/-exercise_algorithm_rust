CARGO_CMD=cargo

.PHONY: all
all: lint build test run

.PHONY: run
run: build
	cat input.txt | $(CARGO_CMD) run

.PHONY: build
build:
	$(CARGO_CMD) build

.PHONY: test
test:
	$(CARGO_CMD) test

.PHONY: format
format:
	$(CARGO_CMD) fmt

.PHONY: lint
lint:
	$(CARGO_CMD) fmt -- --check
	$(CARGO_CMD) clippy

.PHONY: clean
clean:
	rm -rf target
