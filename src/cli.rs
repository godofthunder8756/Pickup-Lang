use clap::Parser;

/// CLI arguments for Pickup
#[derive(Parser)]
#[command(author, version, about = "Pickup: A modern Lua-inspired scripting language")]
pub struct Args {
    /// Optional path to a Pickup script
    pub script: Option<String>,
    
    /// Enable verbose debug output
    #[arg(long = "noise", default_value_t = false)]
    pub verbose_output: bool,
}

pub fn parse_args() -> Args {
    Args::parse()
}