use std::time::Duration;

use colored_text::{self, Colorize};
use laundry_maxxing::laundry::LaundryMachine;
use chrono::Local;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "Welcone to laundry maxxer".yellow());
    println!("{}: machine is free to use", "O".green());
    println!("{}: machine is in use (not available)", "X".red());
    println!("?: an error happened (check internet connection/turn off VPN)");
    let client = reqwest::Client::builder().no_proxy().build()?;
    loop {
        let local_time = Local::now().format("%T");
        print!("[{}] ", local_time.blue());
        print!("{} ", "(Washers)".yellow());
        for &machine in LaundryMachine::all().iter() {
            // Add space before dryer
            if machine == LaundryMachine::Dryer {
                print!("{}", "  (Dryer) ".yellow());
            }

            match machine.is_free(&client).await {
                Ok(is_free) => {
                    if is_free { 
                        print!("{}", "O".green()) 
                    } else { 
                        print!("{}", "X".red())
                    }
                },
                Err(_) => print!("?"),
            }
        }
        println!();
        std::thread::sleep(Duration::from_secs(10));
    }
}
