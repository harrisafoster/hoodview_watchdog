pub mod scan {
    use crate::constants::constants::ALL_SITE_URLS;
    use crate::structs::structs::CalendarAvailabilityDate;
    use std::error::Error;
    use chrono::{Datelike, Local};
    use thirtyfour::prelude::*;
    use webdriver_client::{ chrome::ChromeDriver, Driver };
    use std::time::Duration;
    use std::thread;

    pub async fn scrape_all() -> Result<
        Vec<(i32, Vec<CalendarAvailabilityDate>)>,
        Box<dyn Error + Send + Sync>
    > {
        log::info!("Commencing scraping...");
        let mut all_site_availability: Vec<(i32, Vec<CalendarAvailabilityDate>)> = Vec::new();
        let chromedriver: ChromeDriver = ChromeDriver::spawn().unwrap();
        let session_url: String = chromedriver.url().to_owned();
        let mut caps: thirtyfour::ChromeCapabilities = DesiredCapabilities::chrome();
        caps.add_arg("--headless=new")?;
        caps.add_arg("--start-maximized")?;
        let driver: WebDriver = WebDriver::new(session_url, caps).await?;
        for site_url in ALL_SITE_URLS {
            log::info!("Attempting to scrape site {}", site_url.site_number);
            all_site_availability.push((
                site_url.site_number,
                scrape(site_url.url.to_string(), &driver).await.unwrap(),
            ));
            log::info!("Site {} scraped.", site_url.site_number);
        }

        // close the browser and release its resourcesS
        driver.quit().await?;

        Ok(all_site_availability)
    }

    pub async fn scrape(
        site_url: String,
        driver: &WebDriver
    ) -> Result<Vec<CalendarAvailabilityDate>, Box<dyn Error + Send + Sync>> {
        driver.goto(site_url).await?;

        let mut calendar_availability_dates: Vec<CalendarAvailabilityDate> = Vec::new();

        for n in 0..12 {
            let today: chrono::DateTime<Local> = Local::now();
            let current_month: chrono::DateTime<Local> = today.with_month0((today.month0() + n) % 12).unwrap();
            let month_name: String = current_month.format("%B").to_string();
            log::info!("Current month: {}, {}", month_name, current_month.year());
            for available_date_element in driver.find_all(By::Css(".available")).await? {
                // possible classes: available, blocked, checkout, notYetReleased, currentReservation
                let label: String = available_date_element.attr("aria-label").await?.unwrap();
                let month: String = determine_month(&label).to_string();
                let day: String = determine_day(&label);
                let weekday: String = determine_day_of_week(&label);
                let year: String = determine_year(&label).to_string();
                let available_date: CalendarAvailabilityDate = CalendarAvailabilityDate {
                    weekday,
                    day,
                    month,
                    year,
                };
                if !calendar_availability_dates.contains(&available_date) {
                    calendar_availability_dates.push(available_date);
                }
            }

            let next_button: WebElement = find_element_with_retries(
                "button[aria-label='Next']",
                &driver
            ).await;
            next_button.wait_until().displayed().await?;
            next_button.wait_until().clickable().await?;
            next_button.click().await?;
        }

        Ok(calendar_availability_dates)
    }

    fn determine_month(raw_string: &str) -> &str {
        let split_string: Vec<&str> = raw_string.split(" ").collect();
        split_string[1]
    }

    fn determine_day(raw_string: &str) -> String {
        let split_string: Vec<&str> = raw_string.split(" ").collect();
        let day: &str = split_string[2];
        day.replace(",", "")
    }

    fn determine_day_of_week(raw_string: &str) -> String {
        let split_string: Vec<&str> = raw_string.split(" ").collect();
        let day: &str = split_string[0];
        day.replace(",", "")
    }

    fn determine_year(raw_string: &str) -> &str {
        let split_string: Vec<&str> = raw_string.split(" ").collect();
        split_string[3]
    }

    async fn find_element_with_retries(css_query: &str, driver: &WebDriver) -> WebElement {
        log::info!("Attempting to get WebElement by CSS query: {}", css_query);
        let elem: WebElement = {
            let mut retries: i32 = 3;
            let mut delay: Duration = Duration::from_secs(3);

            loop {
                match driver.find(By::Css(css_query)).await {
                    Ok(button) => {
                        log::info!("WebElement retrieved.");
                        break button;
                    }
                    Err(e) => {
                        if retries == 0 {
                            log::error!("Maximum number of retries exceeded.");
                            println!("{:?}", e);
                        }
                        log::warn!("Retries remaining: {}", retries);
                        retries -= 1;
                        thread::sleep(delay);
                        delay *= 3; // Exponential backoff
                    }
                }
            }
        };

        elem
    }
}
