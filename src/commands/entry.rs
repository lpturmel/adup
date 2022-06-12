use clap::{Parser, Subcommand};

use super::config::command::Config;
use super::install::command::Install;
use super::login::command::Login;

#[derive(Debug, Parser)]
#[clap(name = "Adup")]
#[clap(about = "A World of Warcraft addon manager", long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub commands: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Install an addon
    #[clap(arg_required_else_help = true)]
    Install(Install),

    /// Link to your gitub account
    #[clap(arg_required_else_help = true)]
    Login(Login),

    /// Get info about the logged in account
    Account,
    Config(Config),
}
