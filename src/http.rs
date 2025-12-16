use reqwest::Client;
use crate::error::Error;
use reqwest::multipart::Form;

pub struct GoogleHttp {
    client: Client,
    token: String,
}


impl GoogleHttp {
    pub fn new(token: String) -> Self {
        Self {
            client: Client::new(),
            token
        }
    }

    pub async fn post_json<T: serde::Serialize>(
        &self,
        url: &str,
        body: &T,
    ) -> Result<reqwest::Response, Error> {
        Ok(self.client
            .post(url)
            .bearer_auth(&self.token)
            .json(body)
            .send().await?)
    }

    pub async fn get(
        &self, 
        url: &str,
    ) -> Result<reqwest::Response, Error> {
        Ok(self.client
            .get(url)
            .bearer_auth(&self.token)
            .send().await?)
    }

    pub async fn post_multipart(
        &self,
        url: &str,
        form: Form,
    ) -> Result<reqwest::Response, Error> {
        Ok(self.client
            .post(url)
            .bearer_auth(&self.token)
            .multipart(form)
            .send().await?)
    }
}