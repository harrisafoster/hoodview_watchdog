use scan::scan::scrape_all;
use structs::structs::CalendarAvailabilityDate;
use thirtyfour::{ChromiumLikeCapabilities, DesiredCapabilities, WebDriver};
use tokio;
use webdriver_client::chrome::ChromeDriver;
use webdriver_client::Driver;
pub mod notify;
pub mod scan;
pub mod structs;
pub mod message;
pub mod constants;
use crate::notify::notify::send_notification;
use crate::message::message::generate_message;
use chrono::prelude::*;

#[tokio::main]
async fn main() {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();
    log::info!("Starting main thread...");
    let chromedriver: ChromeDriver = ChromeDriver::spawn().unwrap();
    let session_url: String = chromedriver.url().to_owned();
    let mut caps: thirtyfour::ChromeCapabilities = DesiredCapabilities::chrome();
    caps.add_arg("--headless=new").unwrap();
    caps.add_arg("--start-maximized").unwrap();
    let driver: WebDriver = WebDriver::new(session_url, caps).await.unwrap();
    let available_dates: Vec<(i32, Vec<CalendarAvailabilityDate>)> = scrape_all(&driver).await.unwrap();
    let message: String = generate_message(available_dates);
    let mut subject: String = "Hoodview campsites bruh! Notified: ".to_string();
    let now: String = Local::now().format("%Y-%m-%d %H:%M:%S GMT:%Z").to_string();
    subject.push_str(&now);
    if message != "None" {
        send_notification(subject, message);
    } else {
        log::warn!("No availability found :(");
    }
    driver.quit().await.unwrap();
}
