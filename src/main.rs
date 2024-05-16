use rand::Rng;
use chrono::prelude::*;
use serde::{Serialize, Deserialize};
use reqwest::get;

#[derive(Debug, Serialize, Deserialize)]
struct MyStruct {
    name: String,
    age: u32,
}

async fn fetch_data() -> Result<(), reqwest::Error> {
    // Generate a random number
    let random_number = rand::thread_rng().gen_range(1..=100);
    println!("Random Number: {}", random_number);

    // Get the current date and time
    let current_time = Local::now();
    println!("Current Time: {}", current_time);

    // Serialize and deserialize a struct
    let my_struct = MyStruct {
        name: String::from("Josh"),
        age: 30,
    };
    let serialized = serde_json::to_string(&my_struct).unwrap();
    println!("Serialized: {}", serialized);
    let deserialized: MyStruct = serde_json::from_str(&serialized).unwrap();
    println!("Deserialized: {:?}", deserialized);

    // Make an HTTP request
    let response = get("https://api.coindesk.com/v1/bpi/currentprice/BTC.json").await?;
    let body = response.text().await?;
    println!("Bitcoin Price: {}", body);

    Ok(())
}

fn main() {
    let result = tokio::runtime::Runtime::new().unwrap().block_on(fetch_data());
    if let Err(err) = result {
        eprintln!("Error: {:?}", err);
    }
}
