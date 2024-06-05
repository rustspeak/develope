use serde::{Serialize, Deserialize};
use reqwest;
use serde_json::Result;
use reqwest::Error;

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u8,
    city: String,
    school: String,
}

#[tokio::main]
async fn main() {
    let response = reqwest::get("https://www.rust-lang.org").await;

    match response {
        Ok(res) => {
            let body = res.text().await;

            match body {
                Ok(text) => println!("Response body: {:?}", text),
                Err(err) => eprintln!("Error while reading response body: {}", err),
            }
        }
        Err(err) => eprintln!("Error while making request: {}", err),
    }

    let person = Person {
        name: String::from("Fossouo Wato"),
        age: 18,
        city: String::from("Douala"),
        school: String::from("Polytechnique School of Douala"),
    };

    let json_data = serde_json::to_string(&person).unwrap();
    println!("Serialized JSON: {:?}", json_data);

    let deserialized_person: Person = serde_json::from_str(&json_data).unwrap();
    println!("Deserialized Person: {:?}", deserialized_person);
}