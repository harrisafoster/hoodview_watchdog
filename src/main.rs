use thirtyfour::{ DesiredCapabilities, WebDriver };
use tokio;
use webdriver_client::firefox::GeckoDriver;
use webdriver_client::Driver;
use chrono::{ prelude::*, Duration };
use std::thread;
pub mod notify;
pub mod scan;
pub mod structs;
pub mod message;
pub mod constants;
pub mod reserve;
pub mod login;
pub mod click;
use crate::notify::notify::send_notification;
use crate::message::message::generate_message;
use constants::constants::SITE23;
use login::login::log_in;
use scan::scan::scrape_all;
use structs::structs::CalendarAvailabilityDate;

#[tokio::main]
async fn main() {
    // Set up logging
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();
    log::info!("Starting main thread...");

    // Set up gecko driver
    let gecko_driver: GeckoDriver = GeckoDriver::spawn().unwrap();
    let session_url: String = gecko_driver.url().to_owned();
    let mut caps: thirtyfour::FirefoxCapabilities = DesiredCapabilities::firefox();
    caps.add_arg("--headless").unwrap();
    let driver: WebDriver = WebDriver::new(&session_url, caps).await.unwrap();

    loop {
        driver.goto(SITE23.url).await.unwrap();
        log_in(&driver, SITE23.url).await;
        let available_dates: Vec<(i32, (Vec<CalendarAvailabilityDate>, bool))> = scrape_all(
            &driver
        ).await.unwrap();
        let message: String = generate_message(available_dates);

        let pdt_offset: FixedOffset = FixedOffset::west_opt(8 * 3600).unwrap();
        let pdt_time: DateTime<FixedOffset> = Utc::now().with_timezone(&pdt_offset);
        let formatted_time: String = pdt_time.format("%Y-%m-%d %H:%M:%S GMT:%Z").to_string();
        let mut subject: String = "Hoodview campsites bruh! Notified: ".to_string();
        subject.push_str(&formatted_time);

        if message != "None" {
            send_notification(subject, message);
        } else {
            log::warn!("No available dates found/reserved within desired range.");
        }

        // Calculate sleep duration to run at the next hour on the hour.
        let now: DateTime<Local> = Local::now();
        let next_hour: DateTime<Local> = (now + Duration::hours(1)).with_minute(0).unwrap().with_second(0).unwrap().with_nanosecond(0).unwrap();
        let duration: std::time::Duration = next_hour.signed_duration_since(now).to_std().unwrap();

        log::warn!("Sleeping for {} seconds", duration.as_secs());
        thread::sleep(duration);
    }
}
