use clap::Args;

#[derive(Debug, Args)]
pub struct Delete {
    // The name of the addon to delete
    pub name: String,
}
