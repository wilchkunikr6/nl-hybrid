use std::io::{self, Write};
use crate::api::{ApiClient, CosmeticItem};

pub struct UserInterface {
    api_client: ApiClient,
}

impl UserInterface {
    pub fn new(api_client: ApiClient) -> Self {
        UserInterface { api_client }
    }

    pub fn display_menu(&self) {
        println!("Welcome to NL Hybrid");
        println!("1. Unlock Cosmetic Items");
        println!("2. Exit");
    }

    pub fn handle_input(&self) {
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        match choice.trim() {
            "1" => self.unlock_cosmetics(),
            "2" => std::process::exit(0),
            _ => println!("Invalid choice. Please try again."),
        }
    }

    fn unlock_cosmetics(&self) {
        let items = vec![
            CosmeticItem { name: "Legendary Outfit".to_string(), item_type: "outfit".to_string() },
            CosmeticItem { name: "Epic Emote".to_string(), item_type: "emote".to_string() },
        ];

        for item in items {
            let response = tokio::runtime::Runtime::new().unwrap().block_on(self.api_client.unlock_item(&item));
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