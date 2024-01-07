PROJECT_NAME=$(notdir $(CURDIR))
CONTAINER_PATH=$(CURDIR)/build/docker-compose.yml

workspace: exit
	docker-compose -f $(CONTAINER_PATH) up -d workspace
	docker-compose -f $(CONTAINER_PATH) exec workspace bash

exit:
	docker-compose -f $(CONTAINER_PATH) down --remove-orphans --volumes
	docker rmi -f workspace

clean:
	cargo clean

format:
	cargo fmt

# run clean before test, build or run
# may cause linking with `cc` failed error

test: format
	cargo test

build: format
	cargo build -vvv

run: format
	cargo run

release: clean format
	cargo build --release

.PHONY: build