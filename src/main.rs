//! Read and write PADMet files

#![warn(missing_docs)]

/* std use */

/* crate use */
use anyhow::Context as _;
use clap::Parser as _;

/* project use */
use padmet::cli;
use padmet::error;

#[hotpath::main]
fn main() -> error::Result<()> {
    // Parse argument
    let arguments = cli::Arguments::parse();

    // Setup logger
    stderrlog::new()
        .module(module_path!())
        .quiet(arguments.quiet())
        .verbosity(arguments.verbosity())
        .timestamp(arguments.timestamp())
        .init()
        .context("stderrlog already create a logger")?;

    Ok(())
}
