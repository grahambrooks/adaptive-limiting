.DEFAULT_GOAL := help

.PHONY: help check fmt fmt-check lint test hooks

help: ## List targets
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "  \033[36m%-12s\033[0m %s\n", $$1, $$2}' $(MAKEFILE_LIST)

check: fmt-check lint test ## The pre-push gate: the same commands as CI's check job

fmt: ## Format all Rust sources
	cargo fmt --all

fmt-check: ## Verify formatting
	cargo fmt --all --check

lint: ## Clippy, warnings denied (as CI)
	cargo clippy --workspace --all-targets --all-features --locked -- -D warnings

test: ## Run the tests (as CI)
	cargo test --workspace --all-features --locked

hooks: ## Install the git hooks (pre-commit and pre-push) from prek.toml
	prek install
