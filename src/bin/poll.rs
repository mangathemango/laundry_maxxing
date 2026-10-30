use std::time::Duration;

use colored_text::{self, Colorize};
use laundry_maxxing::{
    laundry::{LaundryMachine, LaundryStore},
    storage::json::JsonStorage,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "Welcone to laundry maxxer polling thing".yellow());
    let client = reqwest::Client::builder().no_proxy().build()?;
    let storage: JsonStorage = JsonStorage {};

    loop {
        for machine in LaundryMachine::all() {
            let poll_event = machine.poll(&client).await;
            if let Err(e) = storage.save_poll_event(poll_event) {
                eprintln!("{e}")
            };
        }
        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}
