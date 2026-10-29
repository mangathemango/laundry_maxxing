use std::time::Duration;

use laundry_maxxing::laundry::LaundryMachine;
use chrono::Local;
use colored_text::{self, Colorize};
use laundry_maxxing::laundry::LaundryState;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "Welcone to laundry maxxer polling thing".yellow());
    let client = reqwest::Client::builder().no_proxy().build()?;
    let mut current_laundry_states = LaundryState::all();
    let mut last_laundry_states = LaundryState::all();
    loop {
        for state in current_laundry_states.iter_mut() {
            if state.try_poll(&client).await.is_err() {
                continue;
            }
        }

        for i in 0..6 {
            if current_laundry_states[i].is_available && !last_laundry_states[i].is_available {
                let unavailable_duration = last_laundry_states[i].last_available.unwrap().elapsed();
                let mins = unavailable_duration.as_secs() / 60;
                let secs = unavailable_duration.as_secs() % 60;
                println!("{:?} was used for {}m {}s", current_laundry_states[i].machine, mins, secs);
            }
        }

        last_laundry_states = current_laundry_states.clone();
        std::thread::sleep(Duration::from_secs(10));
    }
}
