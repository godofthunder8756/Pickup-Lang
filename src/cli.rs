use clap::Parser;

/// CLI arguments for Pickup
#[derive(Parser)]
#[command(author, version, about = "Pickup: A modern Lua-inspired scripting language")]
pub struct Args {
    /// Optional path to a Pickup script
    pub script: Option<String>,
}

pub fn parse_args() -> Args {
    Args::parse()
}