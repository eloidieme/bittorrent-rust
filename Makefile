# BitTorrent Rust Client Makefile
# Provides convenient commands for development, testing, and building

.PHONY: help build test clean run-example check fmt clippy docs install uninstall

help: ## Show this help message
	@echo "BitTorrent Rust Client - Available Commands:"
	@echo "============================================="
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

build: ## Build the project in debug mode
	cargo build

build-release: ## Build the project in release mode
	cargo build --release

test: ## Run all tests
	cargo test

test-verbose: ## Run tests with verbose output
	cargo test -- --nocapture

test-integration: ## Run integration tests
	cargo test --test integration

check: ## Check code without building
	cargo check

fmt: ## Format code with rustfmt
	cargo fmt

fmt-check: ## Check if code is properly formatted
	cargo fmt -- --check

clippy: ## Run clippy linter
	cargo clippy

clippy-pedantic: ## Run clippy with pedantic warnings
	cargo clippy -- -W clippy::pedantic

docs: ## Generate documentation
	cargo doc --no-deps --open

docs-build: ## Build documentation without opening
	cargo doc --no-deps

run-example: ## Run the basic client example with debian torrent
	cargo run --example basic-client -- examples/torrents/debian.iso.torrent

run-example-custom: ## Run basic client with custom torrent file (usage: make run-example-custom TORRENT=path/to/file.torrent)
	cargo run --example basic-client -- $(TORRENT)

dev: ## Run in development mode (build + test + run example)
	$(MAKE) build
	$(MAKE) test
	$(MAKE) run-example

clean: ## Clean build artifacts
	cargo clean

clean-all: clean ## Clean everything including target and debug files
	rm -rf target/
	rm -rf examples/debug/*.json
	find . -name "*.orig" -delete
	find . -name "*.rej" -delete

install: build-release ## Install the binary to ~/.cargo/bin
	cargo install --path .

uninstall: ## Uninstall the binary
	cargo uninstall bittorrent-rust

ci: fmt-check clippy test ## Run CI pipeline (format check + clippy + tests)
	@echo "CI pipeline completed successfully"

ci-full: clean fmt-check clippy-pedantic test build-release ## Run full CI pipeline
	@echo "Full CI pipeline completed successfully"

bench: ## Run benchmarks
	cargo bench

coverage: ## Generate test coverage report
	cargo tarpaulin --out Html

audit: ## Run security audit
	cargo audit

update: ## Update dependencies
	cargo update

info: ## Show project information
	@echo "Project: BitTorrent Rust Client"
	@echo "Version: $(shell grep '^version' Cargo.toml | cut -d'"' -f2)"
	@echo "Rust version: $(shell rustc --version)"
	@echo "Cargo version: $(shell cargo --version)"
	@echo "Target directory: $(shell cargo metadata --format-version 1 | jq -r '.target_directory')"

debug-build: ## Build with debug symbols and info
	RUSTFLAGS="-g" cargo build

debug-run: ## Run example with debug output
	RUST_LOG=debug cargo run --example basic-client -- examples/torrents/debian.iso.torrent

torrent-info: ## Show information about the debian torrent
	@echo "Torrent file: examples/torrents/debian.iso.torrent"
	@echo "File size: $(shell ls -lh examples/torrents/debian.iso.torrent | awk '{print $$5}')"
	@echo "Last modified: $(shell ls -l examples/torrents/debian.iso.torrent | awk '{print $$6, $$7, $$8}')"

quick: fmt clippy test ## Quick development check (format + lint + test)

full: clean fmt clippy-pedantic test build-release run-example ## Full development workflow

tree: ## Show project structure (requires tree command)
	tree -I 'target|.git' -a

sizes: ## Show sizes of important files
	@echo "Project file sizes:"
	@du -h examples/torrents/*.torrent 2>/dev/null || echo "No torrent files found"
	@du -h target/debug/bittorrent-rust 2>/dev/null || echo "Debug binary not built"
	@du -h target/release/bittorrent-rust 2>/dev/null || echo "Release binary not built"

setup: ## Setup development environment
	@echo "Setting up development environment..."
	rustup component add rustfmt clippy
	@echo "Development environment ready!"

# Show this help by default
.DEFAULT_GOAL := help
