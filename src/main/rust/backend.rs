use reqwest::Client;
use crate::types::{Inputs, Outputs};

const URL: &str = "http://127.0.0.1:6060/calculator";

pub struct CalculatorClient(Client);

impl CalculatorClient {
    pub fn new() -> Self {
        Self(Client::new())
    }

    pub(crate) async fn calculate(&self, inputs: Inputs) -> Result<Outputs, reqwest::Error> {
        let client = &self.0;

        Ok(client.post(URL)
            .json(&inputs)
            .send()
            .await?
            .json::<Outputs>()
            .await?)

    }
}

