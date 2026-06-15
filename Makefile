.PHONY: help dev test contract-test sse-replay security-test

help: ## Show this help
	@echo "Yume dev commands (Swing 0.1 — skeleton)"
	@echo ""
	@echo "Available targets:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | \
		awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-20s\033[0m %s\n", $$1, $$2}'

dev: ## Start local dev stack (docker compose)
	@echo "Starting local dev stack (docker compose)..."
	@echo "(Not yet implemented — see Swing 0.4)"

test: ## Run all tests
	cargo test --workspace

contract-test: ## Run contract tests
	cargo test -p yume-contracts

sse-replay: ## Run SSE replay harness
	@echo "Running SSE replay harness..."
	@echo "(Not yet implemented — see Swing 0.5)"

security-test: ## Run security tests
	@echo "Running security tests..."
	@echo "(Not yet implemented — see Swing 0.10)"
