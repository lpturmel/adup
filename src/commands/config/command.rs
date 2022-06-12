use clap::{Args, Subcommand};

#[derive(Debug, Args)]
pub struct Config {
    #[clap(subcommand)]
    pub commands: Option<ConfigCommands>,
}

#[derive(Debug, Subcommand)]
pub enum ConfigCommands {
    #[clap(arg_required_else_help = true)]
    Location(Location),
}

#[derive(Debug, Args)]
pub struct Location {
    #[clap(subcommand)]
    pub commands: Option<LocationCommands>,
}

#[derive(Debug, Subcommand)]
pub enum LocationCommands {
    #[clap(arg_required_else_help = true)]
    Set(ConfigSetCommand),
    Get,
}

#[derive(Debug, Args)]
pub struct ConfigSetCommand {
    /// The path to the game location
    pub path: String,
}
