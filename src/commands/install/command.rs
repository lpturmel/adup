use clap::Args;

#[derive(Debug, Args)]
pub struct Install {
    // The name of the addon to insall
    pub name: String,
}
