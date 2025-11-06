.PHONY: help build run test clean fmt lint init-dev frontend-dev docker-up docker-down deploy

help:
	@echo "ReputeChain - Makefile Commands"
	@echo "================================"
	@echo "make build           - Build the blockchain node"
	@echo "make run             - Run the blockchain node in dev mode"
	@echo "make test            - Run tests"
	@echo "make clean           - Clean build artifacts"
	@echo "make fmt             - Format code"
	@echo "make lint            - Run clippy linter"
	@echo "make init-dev        - Initialize dev environment"
	@echo "make frontend-dev    - Start frontend dev server"
	@echo "make docker-up       - Start full stack with Docker"
	@echo "make docker-down     - Stop Docker containers"
	@echo "make deploy          - Deploy to production"

build:
	cargo build --release

run:
	cargo run --release -- --dev --tmp

test:
	cargo test

clean:
	cargo clean
	rm -rf frontend/dist frontend/node_modules

fmt:
	cargo fmt --all

lint:
	cargo clippy --all-targets --all-features -- -D warnings

init-dev:
	rustup default stable
	rustup update
	rustup target add wasm32-unknown-unknown
	rustup component add rust-src
	cd frontend && npm install

frontend-dev:
	cd frontend && npm run dev

frontend-build:
	cd frontend && npm run build

docker-build:
	docker build -t reputechain:latest .

docker-up: docker-build
	docker-compose up -d

docker-down:
	docker-compose down

deploy: build frontend-build
	@echo "Deployment ready. Push to your hosting provider."

.DEFAULT_GOAL := help
