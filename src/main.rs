use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::io::{self, Write};

#[derive(Serialize, Deserialize)]
struct CosmeticItem {
    name: String,
    item_type: String,
}

#[derive(Serialize, Deserialize)]
struct UnlockResponse {
    success: bool,
    message: String,
}

struct App {
    client: Client,
}

impl App {
    async fn unlock_item(&self, item: &CosmeticItem) -> Result<UnlockResponse, Box<dyn Error>> {
        let response = self.client.post("https://api.fortnite.com/unlock")
            .json(item)
            .send()
            .await?
            .json::<UnlockResponse>()
            .await?;
        Ok(response)
    }

    fn run(&self) {
        let items = vec![
            CosmeticItem { name: "Legendary Outfit".to_string(), item_type: "outfit".to_string() },
            CosmeticItem { name: "Epic Emote".to_string(), item_type: "emote".to_string() },
        ];

        for item in items {
            let response = tokio::runtime::Runtime::new().unwrap().block_on(self.unlock_item(&item));
            match response {
                Ok(res) => {
                    println!("Unlocking {}: {}", item.name, res.message);
                }
                Err(e) => {
                    eprintln!("Error unlocking {}: {}", item.name, e);
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let client = Client::new();
    let app = App { client };
    app.run();
}