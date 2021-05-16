use reqwest;
use serde_json;

pub struct Client {
    pub base_request: Request
}

impl Client {
    pub fn new(base_url: &str) -> Client {
        let base_request = Request::new(base_url);
        
        Client { base_request }
    }

    pub fn get_avg_price(&self, symbol: &str) -> serde_json::Value {
        let endpoint = "avgPrice";
        let query = [("symbol", symbol)];

        let request = self.base_request.make(endpoint, &query).unwrap();
        request
    }
}

pub struct Request {
    api_client: reqwest::Client,
    base_url: String
}

impl Request {
    pub fn new(base_url: &str) -> Request {
        let api_client = reqwest::Client::new();
        let base_url = String::from(base_url);
        
        Request { api_client, base_url }
    }

    #[tokio::main]
    async fn make(&self, endpoint: &str, query: &[(&str, &str)]) -> Result<serde_json::Value, reqwest::Error> {
        let url = format!("{base_url}/{endpoint}", base_url=self.base_url, endpoint=endpoint);
        let response = self.api_client.get(url)
            .query(query)
            .send()
            .await?;
        let content = response.text().await?;
        
        let response_data: serde_json::Value = serde_json::from_str(&content).unwrap();
        
        Ok(response_data)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
