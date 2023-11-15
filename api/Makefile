CARGO := cargo

all:
	$(CARGO) run

build:
	$(CARGO) build

test:
	$(CARGO) test -- --test-threads=1

fmt:
	$(CARGO) fmt

.phony:
	all build test fmt