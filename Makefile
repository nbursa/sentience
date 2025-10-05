# Makefile for Sentience Core

.PHONY: all build test clean install dev

# Default target
all: build

# Build the Rust library and Python extension
build:
	@echo "Building Sentience Core..."
	cargo build --release
	@echo "Building Python extension..."
	python setup.py build_ext --inplace
	@echo "Build complete!"

# Install in development mode
dev:
	@echo "Installing in development mode..."
	pip install -e .
	@echo "Development installation complete!"

# Install the package
install:
	@echo "Installing Sentience Core..."
	pip install .
	@echo "Installation complete!"

# Run tests
test:
	@echo "Running tests..."
	cargo test
	python -m pytest tests/ -v
	@echo "Tests complete!"

# Run the Rust demo
demo-rust:
	@echo "Running Rust demo..."
	cargo run --example sentience_core_demo

# Run the Python integration demo
demo-python:
	@echo "Running Python integration demo..."
	python examples/srai_sentience_integration.py

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	cargo clean
	rm -rf build/
	rm -rf *.so
	rm -rf *.dylib
	rm -rf *.dll
	find . -name "*.pyc" -delete
	find . -name "__pycache__" -delete
	@echo "Clean complete!"

# Format code
format:
	@echo "Formatting code..."
	cargo fmt
	black python/
	@echo "Formatting complete!"

# Lint code
lint:
	@echo "Linting code..."
	cargo clippy
	mypy python/
	@echo "Linting complete!"

# Check code quality
check: format lint test
	@echo "All checks passed!"

# Build documentation
docs:
	@echo "Building documentation..."
	cargo doc --no-deps --open
	@echo "Documentation complete!"

# Help
help:
	@echo "Available targets:"
	@echo "  build      - Build the Rust library and Python extension"
	@echo "  dev        - Install in development mode"
	@echo "  install    - Install the package"
	@echo "  test       - Run all tests"
	@echo "  demo-rust  - Run the Rust demo"
	@echo "  demo-python - Run the Python integration demo"
	@echo "  clean      - Clean build artifacts"
	@echo "  format     - Format code"
	@echo "  lint       - Lint code"
	@echo "  check      - Run all checks (format, lint, test)"
	@echo "  docs       - Build documentation"
	@echo "  help       - Show this help"
