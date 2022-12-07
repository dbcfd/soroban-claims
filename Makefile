SHELL := /bin/bash

.PHONY:
test: build
	cargo test --release


build:
	cargo build --target wasm32-unknown-unknown --release

.PHONY:
deploy-patient-visit:
	{ \
		soroban invoke \
      		--wasm target/wasm32-unknown-unknown/release/soroban-claims.wasm \
      		--id 1 \
      		--fn hello \
			--arg friend \
	}

