use anyhow::{Ok, Result};

use clap::Parser;
use grep::*;

fn main() -> Result<()> {
    let config = GrepConfig::parse();

    config.match_with_default_strategy()?;

    Ok(())
}
