OS := $(shell uname)
ifeq ($(OS), Darwin)
	OPEN := open
else
	OPEN := xdg-open
endif

.PHONY: test
test:
	cargo test --all-features --verbose

.PHONY: lint
lint:
	cargo check --verbose --workspace --all-targets

.PHONY: fmt
fmt:
	cargo fmt --all

.PHONY: check-fmt
check-fmt:
	cargo fmt --all -- --check

.PHONY: coverage
coverage:
	cargo tarpaulin --out Html

	@echo "\nMoving coverage report to ./coverage"
	@mkdir -p coverage
	@mv tarpaulin-report.html ./coverage/index.html

	$(OPEN) coverage/index.html

