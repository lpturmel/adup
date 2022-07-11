pub struct Version;

impl Version {
    fn parse(version_nb: String) -> anyhow::Result<u32, anyhow::Error> {
        // v3234235
        let filtered_version: String = version_nb
            .to_owned()
            .chars()
            .filter(|c| c.is_numeric())
            .collect();

        let nb = filtered_version.parse::<u32>()?;
        return Ok(nb);
    }
}
#[cfg(test)]
mod tests {
    use super::Version;
    #[test]
    fn it_can_remove_non_numeric_chars() {
        let version = "v323232";

        assert_eq!(Version::parse(version.to_string()).unwrap(), 323232);
    }
}
