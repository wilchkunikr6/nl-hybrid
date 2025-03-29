pub mod api {
    use reqwest::Client;
    use serde::{Deserialize, Serialize};
    use std::error::Error;

    #[derive(Serialize, Deserialize)]
    pub struct CosmeticItem {
        pub name: String,
        pub item_type: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct UnlockResponse {
        pub success: bool,
        pub message: String,
    }

    pub struct ApiClient {
        client: Client,
    }

    impl ApiClient {
        pub fn new() -> Self {
            let client = Client::new();
            ApiClient { client }
        }

        pub async fn unlock_item(&self, item: &CosmeticItem) -> Result<UnlockResponse, Box<dyn Error>> {
            let response = self.client.post("https://api.fortnite.com/unlock")
                .json(item)
                .send()
                .await?
                .json::<UnlockResponse>()
                .await?;
            Ok(response)
        }
    }
}