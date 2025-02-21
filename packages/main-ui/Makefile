SERVICE ?= main-ui
VERSION ?= $(shell toml get Cargo.toml package.version | tr -d \")
COMMIT ?= $(shell git rev-parse --short HEAD)
ENV ?= local

ACCESS_KEY_ID ?= $(shell aws configure get aws_access_key_id $(AWS_FLAG))
SECRET_ACCESS_KEY ?= $(shell aws configure get aws_secret_access_key $(AWS_FLAG))
REGION ?= $(shell aws configure get region)

RUST_LOG ?= debug
WORKSPACE_ROOT ?= ../..

BUILD_ENV ?= ENV=$(ENV) VERSION=$(VERSION) COMMIT=$(COMMIT) RUST_LOG=$(RUST_LOG) ENV=$(ENV) AWS_REGION=${REGION} AWS_ACCESS_KEY_ID=${ACCESS_KEY_ID} AWS_SECRET_ACCESS_KEY=${SECRET_ACCESS_KEY} SERVICE=$(SERVICE) WORKSPACE_ROOT=$(WORKSPACE_ROOT)

setup.tool:
	cargo binstall dioxus-cli
	cargo binstall toml-cli
	npm i -g @tailwindcss/cli

run: clean public
	$(BUILD_ENV) dx serve --fullstack

build: clean public
	$(BUILD_ENV) dx build --release --fullstack --server-features lambda
	cp -r $(WORKSPACE_ROOT)/target/dx/$(SERVICE)/release/web $(ARTIFACT_DIR)
	mv $(ARTIFACT_DIR)/server $(ARTIFACT_DIR)/bootstrap

.PHONY: public
public: public/tailwind.css

public/tailwind.css:
	tailwindcss -i ./public/input.css -o ./public/tailwind.css --minify

clean:
	rm -rf public/tailwind.css public/dep.js

build-docker: clean public/tailwind.css
	docker run -it --rm --name $(SERVICE) -v $(PWD)/../..:/app -w /app/packages/$(SERVICE) biyard/dioxus-docker bash -c 'source ~/.cargo/env && $(BUILD_ENV) dx build --release --fullstack --server-features lambda && cp -r /app/target/dx/$(SERVICE)/release/web /app/.build/$(SERVICE) && mv /app/.build/$(SERVICE)/server /app/.build/$(SERVICE)/bootstrap'
