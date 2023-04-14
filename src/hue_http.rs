
// A wrapper for reqwest::blocking::get that can be mocked.
pub mod hue_http {
    use mockall::automock;
    #[automock]
    pub mod get_request {
        use crate::HueError;

        pub fn get(url: &str) -> Result<String, HueError> {
            let response = reqwest::blocking::get(url.to_string());
            match response {
                Ok(response) => Ok(response.text()?),
                Err(e) => Err(e.into())
            }
        }

    }
}
