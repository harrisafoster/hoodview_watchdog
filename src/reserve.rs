pub mod reserve {
    use chrono::NaiveDate;
    use thirtyfour::{ By, WebDriver, WebElement };
    use std::error::Error;
    use crate::{
        click::click::{ find_element_with_retries, scroll_and_click },
        scan::scan::extract_date_from_label,
        structs::structs::CalendarAvailabilityDate,
    };

    /// Reserves a block of consecutive dates on the specified campsite.

    /// This function attempts to reserve a block of consecutive dates on the given campsite.
    /// It navigates to the campsite page, locates the start and end dates in the calendar,
    /// clicks on them to select them, adds the reservation to the cart,
    /// and then proceeds to the cart page.

    /// # Arguments
    ///
    /// * `driver`: A reference to the WebDriver instance.
    /// * `site_url`: The URL of the campsite to reserve.
    /// * `start_date`: The start date of the reservation period.
    /// * `end_date`: The end date of the reservation period.
    ///
    /// # Returns
    ///
    /// A `Result` containing a vector of `CalendarAvailabilityDate` representing the reserved dates,
    /// or an error if the reservation process fails.
    pub async fn reserve_sites(
        driver: &WebDriver,
        site_url: &str,
        start_date: NaiveDate,
        end_date: NaiveDate
    ) -> Result<Vec<CalendarAvailabilityDate>, Box<dyn Error + Send + Sync>> {
        log::info!("Attempting to reserve sites...");

        driver.goto(site_url).await.unwrap();

        let mut found_start: bool = false;
        let mut found_end: bool = false;
        let mut start_element: Option<thirtyfour::WebElement> = None;
        let mut end_element: Option<thirtyfour::WebElement> = None;
        let mut reserved_dates: Vec<CalendarAvailabilityDate> = Vec::new();

        for n in 0..12 {
            log::info!("Iteration: {}", n + 1);

            for available_date_element in driver.find_all(By::Css(".available")).await? {
                let label: String = available_date_element.attr("aria-label").await?.unwrap();
                let available_date: CalendarAvailabilityDate = extract_date_from_label(&label);

                if available_date.to_naive_date() == start_date {
                    log::info!("Found start date: {:?}", available_date);
                    start_element = Some(available_date_element);
                    found_start = true;
                } else if
                    available_date.to_naive_date() == end_date &&
                    available_date.to_naive_date() != start_date
                {
                    log::info!("Found end date: {:?}", available_date);
                    end_element = Some(available_date_element);
                    found_end = true;
                }

                if found_start && found_end {
                    break;
                }
            }

            if found_start && found_end {
                break;
            }

            scroll_and_click(&driver, "button[aria-label='Next']").await;
        }

        if found_start && found_end && start_element.is_some() && end_element.is_some() {
            let start_date_element: WebElement = start_element.unwrap();
            let end_date_element: WebElement = end_element.unwrap();

            reserved_dates.push(
                extract_date_from_label(&start_date_element.attr("aria-label").await?.unwrap())
            );
            reserved_dates.push(
                extract_date_from_label(&end_date_element.attr("aria-label").await?.unwrap())
            );

            scroll_and_click(
                &driver,
                &format!(
                    "div[aria-label='{}']",
                    start_date_element.attr("aria-label").await?.unwrap()
                )
            ).await;
            scroll_and_click(
                &driver,
                &format!(
                    "div[aria-label='{}']",
                    end_date_element.attr("aria-label").await?.unwrap()
                )
            ).await;

            scroll_and_click(&driver, "button[id='add-cart-campsite']").await;
            let campsite_id_query = format!(
                "button[id='campsite-id-{}-remove-item']",
                extract_campsite_id(site_url).unwrap()
            );

            match find_element_with_retries(&campsite_id_query, &driver).await {
                Ok(_) => {
                    driver.goto(site_url).await.unwrap();
                    scroll_and_click(
                        &driver,
                        "button[data-component='Button'][type='button'].sarsa-button.sarsa-button-link.sarsa-button-lg.pb-1.pb-lg-0"
                    ).await;

                    log::info!("Successfully reserved sites.");
                }
                Err(e) => {
                    log::error!("Reservation not successful, {}", e);
                }
            }

            return Ok(reserved_dates);
        } else {
            log::warn!("Could not find all desired dates.");
        }

        Ok(reserved_dates)
    }

    /// Extracts the campsite ID from a given URL.

    /// This function extracts the campsite ID from a URL string.
    /// It assumes the campsite ID is the last part of the URL and consists of digits.

    /// # Arguments
    ///
    /// * `url`: The URL string containing the campsite ID.
    ///
    /// # Returns
    ///
    /// An `Option<String>` containing the extracted campsite ID,
    /// or `None` if the URL does not contain a valid campsite ID.
    fn extract_campsite_id(url: &str) -> Option<String> {
        let url_parts = url.split('/');
        let last_part = url_parts.last()?;
        Some(
            last_part
                .chars()
                .filter(|c| c.is_ascii_digit())
                .collect::<String>()
                .trim()
                .to_string()
        )
    }
}
