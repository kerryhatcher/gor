# gor development tasks — run `just` to list recipes, `just <recipe>` to run one.
# https://github.com/casey/just

# List all available recipes.
default:
    @just --list

# Build with locked Cargo.lock.
build:
    cargo build --locked

# Build an optimized release binary with locked Cargo.lock.
release:
    cargo build --locked --release

# Run the full test suite (nextest + doc tests).
test:
    cargo nextest run --workspace --all-features
    cargo test --doc

# Run library unit tests only.
test-unit:
    cargo nextest run --lib

# Run integration tests only.
test-integration:
    cargo nextest run --test integration

# Lint: check formatting and run clippy with warnings denied.
lint:
    cargo fmt --all --check
    cargo clippy --all-targets --all-features -- -D warnings

# Lint and auto-fix formatting and clippy issues.
lint-fix:
    cargo fmt --all
    cargo clippy --all-targets --all-features --fix --allow-dirty

# Build documentation with broken links / warnings denied.
doc:
    RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --open

# Produce an lcov coverage report via llvm-cov.
coverage:
    cargo llvm-cov nextest --workspace --all-features --lcov --output-path lcov.info
    cargo llvm-cov report --lcov --output-path lcov.info

# Supply-chain checks: cargo-deny and cargo-audit.
audit:
    cargo deny check
    cargo audit

# Spellcheck the tree.
typos:
    typos

# Detect unused dependencies.
shear:
    cargo shear

# Remove build artifacts.
clean:
    cargo clean

# Run the same gates as CI: lint, test, audit, typos, shear.
ci: lint test audit typos shear

# Install the binary locally from this checkout.
install:
    cargo install --path . --locked
