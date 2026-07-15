//! Thin entry point for the `gor` binary.
//!
//! Parses CLI arguments, initializes tracing, and dispatches to the library.
//! All business logic lives in [`gor`] (the library crate).

use tracing_subscriber::{EnvFilter, fmt};

fn main() -> anyhow::Result<()> {
    // Initialize tracing with a reasonable default filter.
    // Set RUST_LOG to override (e.g. RUST_LOG=debug).
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("gor=info"));
    fmt().with_env_filter(filter).init();

    gor::Gor::new().run()
}
