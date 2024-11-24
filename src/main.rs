use scan::scan::scrape_all;
use structs::structs::CalendarAvailabilityDate;
use tokio;
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
    let available_dates: Vec<(i32, Vec<CalendarAvailabilityDate>)> = scrape_all().await.unwrap();
    let message: String = generate_message(available_dates);
    let mut subject: String = "Hoodview campsites bruh! Notified: ".to_string();
    let now: String = Local::now().format("%Y-%m-%d %H:%M:%S GMT:%Z").to_string();
    subject.push_str(&now);
    if message != "None" {
        send_notification(subject, message);
    } else {
        log::warn!("No availability found :(");
    }
}
