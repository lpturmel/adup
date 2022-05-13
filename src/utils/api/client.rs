use super::responses::Profile;
use reqwest::Client;

pub struct Api {
    base_url: &'static str,
    pat: String,
    client: Client,
}

impl Api {
    pub fn new(pat_token: String) -> Self {
        let base_url = "https://api.github.com";
        let client = Client::new();
        Self {
            base_url,
            pat: pat_token,
            client,
        }
    }

    pub async fn get_profile(&self) -> Result<Profile, reqwest::Error> {
        let profile_url = format!("{}{}", self.base_url, "/user");
        let response = self
            .client
            .get(profile_url)
            .header("Content-Type", "application/json")
            .header("User-Agent", "AdUp/0.1")
            .header("Authorization", format!("token {}", self.pat))
            .send()
            .await?;
        let profile = response.json::<Profile>().await?.clone();

        Ok(profile)
    }

    pub async fn get_repo(&self, repo_name: String) {
        todo!()
    }
}
