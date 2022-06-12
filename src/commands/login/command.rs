use clap::Args;

#[derive(Debug, Args)]
pub struct Login {
    // Your Github Public Access Token
    pub pat: String,
}
