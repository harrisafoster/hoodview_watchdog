use scan::scan::scrape_all;
use structs::structs::CalendarAvailabilityDate;
use thirtyfour::{ DesiredCapabilities, WebDriver };
use tokio;
use webdriver_client::firefox::GeckoDriver;
use webdriver_client::Driver;
pub mod notify;
pub mod scan;
pub mod structs;
pub mod message;
pub mod constants;
pub mod state;
use crate::notify::notify::send_notification;
use crate::message::message::generate_message;
use crate::state::state::message_changed;
use chrono::prelude::*;
use std::{ thread, time };

#[tokio::main]
async fn main() {
    // Get sleep interval
    dotenv::dotenv().ok();
    let interval: String = dotenv::var("INTERVAL").unwrap_or_else(|err| {
        log::error!("Failed to read INTERVAL: {}", err);
        panic!("Missing required environment variable: INTERVAL");
    });
    let converted_interval: u64 = interval.parse().unwrap();

    // Set up logging
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();
    log::info!("Starting main thread...");

    // Set up gecko driver
    let gecko_driver: GeckoDriver = GeckoDriver::spawn().unwrap();
    let session_url: String = gecko_driver.url().to_owned();

    loop {
        let mut caps: thirtyfour::FirefoxCapabilities = DesiredCapabilities::firefox();
        caps.add_arg("--headless").unwrap();
        let driver: WebDriver = WebDriver::new(&session_url, caps).await.unwrap();
        let available_dates: Vec<(i32, Vec<CalendarAvailabilityDate>)> = scrape_all(
            &driver
        ).await.unwrap();
        let message: String = generate_message(available_dates);
        let mut subject: String = "Hoodview campsites bruh! Notified: ".to_string();
        let now: String = Local::now().format("%Y-%m-%d %H:%M:%S GMT:%Z").to_string();
        subject.push_str(&now);

        let message_is_new: bool = message_changed(&message).unwrap();
        if message != "None" && message_is_new == true {
            send_notification(subject, message);
        } else {
            log::warn!("No change in availability since the last run.");
        }
        driver.quit().await.unwrap();

        let sleep_interval: time::Duration = time::Duration::from_secs(converted_interval);
        log::warn!("Sleeping for {} seconds", converted_interval);
        thread::sleep(sleep_interval);
    }
}
