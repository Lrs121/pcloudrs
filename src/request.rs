use super::PCloudApi;

const USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);
pub const ROOT_FOLDER: usize = 0;

#[derive(Debug)]
pub enum Error {
    Payload(u16, String),
    Reqwest(reqwest::Error),
    ResponseFormat,
    Download(std::io::Error),
    Upload(std::io::Error),
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Self::Reqwest(err)
    }
}

#[derive(Debug, serde::Deserialize)]
#[serde(untagged)]
pub enum Response<T> {
    Error {
        result: u16,
        error: String,
    },
    Success {
        result: u16,
        #[serde(flatten)]
        payload: T,
    },
}

impl<T> Response<T> {
    pub fn payload(self) -> Result<T, Error> {
        match self {
            Self::Error { result, error } => Err(Error::Payload(result, error)),
            Self::Success { payload, .. } => Ok(payload),
        }
    }
}

impl PCloudApi {
    pub(crate) fn create_client() -> reqwest::Client {
        reqwest::ClientBuilder::new()
            .user_agent(USER_AGENT)
            .build()
            .expect("couldn't create reqwest client")
    }

    fn build_url(&self, method: &str) -> String {
        format!("{}/{}", self.data_center.base_url(), method)
    }

    pub(crate) async fn get_request<T: serde::de::DeserializeOwned>(
        &self,
        method: &str,
        params: &[(&str, &str)],
    ) -> Result<T, Error> {
        let mut local_params = self.credentials.to_vec();
        local_params.extend_from_slice(params);
        let uri = self.build_url(method);
        let req = self.client.get(uri).query(&local_params).send().await?;
        // req.json::<T>().await.map_err(Error::from)
        let body = req.text().await?;
        println!("GET {}: {}", method, body);
        Ok(serde_json::from_str(&body).unwrap())
    }

    pub(crate) async fn put_request_data<T: serde::de::DeserializeOwned>(
        &self,
        method: &str,
        params: &[(&str, &str)],
        payload: Vec<u8>,
    ) -> Result<T, Error> {
        let mut local_params = self.credentials.to_vec();
        local_params.extend_from_slice(params);
        let uri = self.build_url(method);
        let req = self
            .client
            .put(uri)
            .query(&local_params)
            .body(payload)
            .send()
            .await?;
        // req.json::<T>().await.map_err(Error::from)
        let body = req.text().await?;
        println!("PUT {}: {}", method, body);
        Ok(serde_json::from_str(&body).unwrap())
    }
}
