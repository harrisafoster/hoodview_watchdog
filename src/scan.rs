pub mod scan {
    use crate::click::click::scroll_and_click;
    use crate::constants::constants::ALL_SITE_URLS;
    use crate::reserve::reserve::reserve_sites;
    use crate::structs::structs::CalendarAvailabilityDate;
    use std::error::Error;
    use chrono::prelude::*;
    use thirtyfour::{ By, WebDriver };

    /// Scrapes availability data for all sites.
    ///
    /// This function iterates through a list of site URLs and calls the `scrape` function
    /// for each site to retrieve its availability data.
    ///
    /// # Arguments
    ///
    /// * `driver`: A reference to the WebDriver instance.
    ///
    /// # Returns
    ///
    /// A `Result` containing a vector of tuples. Each tuple represents a site and its availability data:
    ///
    /// - `i32`: The site number.
    /// - `(Vec<CalendarAvailabilityDate>, bool)`:
    ///     - `Vec<CalendarAvailabilityDate>`: A vector containing the available dates for the site.
    ///     - `bool`: A boolean indicating whether the site is currently reserved.
    ///
    /// # Errors
    ///
    /// Returns an error if any of the `scrape` calls fail.
    pub async fn scrape_all(
        driver: &WebDriver
    ) -> Result<Vec<(i32, (Vec<CalendarAvailabilityDate>, bool))>, Box<dyn Error + Send + Sync>> {
        log::info!("Commencing scraping...");
        let mut all_site_availability: Vec<
            (i32, (Vec<CalendarAvailabilityDate>, bool))
        > = Vec::new();
        for site_url in ALL_SITE_URLS {
            log::info!("Attempting to scrape site {}", site_url.site_number);
            all_site_availability.push((
                site_url.site_number,
                scrape(site_url.url, &driver).await.unwrap(),
            ));
            log::info!("Site {} scraped.", site_url.site_number);
        }

        Ok(all_site_availability)
    }

    /// Scrapes availability data for a specific site.
    ///
    /// This function navigates to the given site URL, finds available dates on the calendar,
    /// and optionally reserves a block of consecutive dates if they meet certain criteria.
    ///
    /// # Arguments
    ///
    /// * `site_url`: The URL of the site to scrape.
    /// * `driver`: A reference to the WebDriver instance.
    ///
    /// # Returns
    ///
    /// A `Result` containing a tuple:
    ///
    /// - `Vec<CalendarAvailabilityDate>`: A vector containing the available dates for the site.
    /// - `bool`: A boolean indicating whether a reservation was made for the site.
    ///
    /// # Errors
    ///
    /// Returns an error if any of the WebDriver operations fail, such as navigation,
    /// finding elements, or executing JavaScript.
    pub async fn scrape(
        site_url: &str,
        driver: &WebDriver
    ) -> Result<(Vec<CalendarAvailabilityDate>, bool), Box<dyn Error + Send + Sync>> {
        driver.goto(site_url).await?;

        let mut calendar_availability_dates: Vec<CalendarAvailabilityDate> = Vec::new();

        for n in 0..12 {
            log::info!("Iteration: {}", n + 1);
            for available_date_element in driver.find_all(By::Css(".available")).await? {
                // possible classes: available, blocked, checkout, notYetReleased, currentReservation
                let label: String = available_date_element.attr("aria-label").await?.unwrap();
                let available_date: CalendarAvailabilityDate = extract_date_from_label(&label);
                if !calendar_availability_dates.contains(&available_date) {
                    calendar_availability_dates.push(available_date);
                }
            }

            scroll_and_click(&driver, "button[aria-label='Next']").await;
        }

        let mut reserved_dates: Vec<CalendarAvailabilityDate> = Vec::new();
        let mut reserved: bool = false;

        if
            let Some((start_date, end_date, days)) = find_longest_consecutive_dates(
                &calendar_availability_dates
            )
        {
            // Check if the found dates are within the desired month range
            if
                days >= 3 &&
                start_date.month() >= 7 &&
                start_date.month() <= 9 &&
                end_date.month() >= 7 &&
                end_date.month() <= 9
            {
                reserved_dates = reserve_sites(
                    driver,
                    site_url,
                    start_date,
                    end_date
                ).await.unwrap();
                reserved = true;
            } else {
                reserved_dates = calendar_availability_dates;
            }
        }

        Ok((reserved_dates, reserved))
    }

    /// Extracts the month from a given string.
    ///
    /// This function takes a string as input, splits it by spaces,
    /// and returns the second element of the resulting array,
    /// assuming the second element represents the month.
    ///
    /// # Arguments
    ///
    /// * `raw_string`: The input string containing the month.
    ///
    /// # Returns
    ///
    /// A string slice representing the extracted month.
    ///
    /// # Example
    ///
    /// ```rust
    /// let month = determine_month("Tuesday, January 15, 2024");
    /// assert_eq!(month, "January");
    /// ```
    fn determine_month(raw_string: &str) -> &str {
        let split_string: Vec<&str> = raw_string.split(" ").collect();
        split_string[1]
    }

    /// Extracts the day from a given string.
    ///
    /// This function takes a string as input, splits it by spaces,
    /// and extracts the third element, assuming it represents the day of the month.
    /// It then removes the comma from the extracted day.
    ///
    /// # Arguments
    ///
    /// * `raw_string`: The input string containing the day.
    ///
    /// # Returns
    ///
    /// A String representing the extracted day of the month without the comma.
    ///
    /// # Example
    ///
    /// ```rust
    /// let day = determine_day("Tuesday, January 15, 2024");
    /// assert_eq!(day, "15");
    /// ```
    fn determine_day(raw_string: &str) -> String {
        let split_string: Vec<&str> = raw_string.split(" ").collect();
        let day: &str = split_string[2];
        day.replace(",", "")
    }

    /// Extracts the day of the week from a given string.

    /// This function takes a string as input, splits it by spaces,
    /// and extracts the first element, assuming it represents the day of the week.
    ///
    /// # Arguments
    ///
    /// * `raw_string`: The input string containing the day of the week.
    ///
    /// # Returns
    ///
    /// A String representing the extracted day of the week.
    ///
    /// # Example
    ///
    /// ```rust
    /// let day_of_week = determine_day_of_week("Tuesday, January 15, 2024");
    /// assert_eq!(day_of_week, "Tuesday");
    /// ```
    fn determine_day_of_week(raw_string: &str) -> String {
        let split_string: Vec<&str> = raw_string.split(" ").collect();
        let day: &str = split_string[0];
        day.replace(",", "")
    }

    /// Extracts the year from a given string.

    /// This function takes a string as input, splits it by spaces,
    /// and extracts the fourth element, assuming it represents the year.
    ///
    /// # Arguments
    ///
    /// * `raw_string`: The input string containing the year.
    ///
    /// # Returns
    ///
    /// A string slice representing the extracted year.
    ///
    /// # Example
    ///
    /// ```rust
    /// let year = determine_year("Tuesday, January 15, 2024");
    /// assert_eq!(year, "2024");
    /// ```
    fn determine_year(raw_string: &str) -> &str {
        let split_string: Vec<&str> = raw_string.split(" ").collect();
        split_string[3]
    }

    /// Finds the longest consecutive sequence of available dates.
    ///
    /// This function takes a slice of `CalendarAvailabilityDate` as input and
    /// identifies the longest consecutive sequence of available dates within the given slice.
    ///
    /// # Arguments
    ///
    /// * `dates`: A slice of `CalendarAvailabilityDate` representing the available dates.
    ///
    /// # Returns
    ///
    /// An `Option` containing a tuple:
    ///
    /// - `NaiveDate`: The start date of the longest consecutive sequence.
    /// - `NaiveDate`: The end date of the longest consecutive sequence.
    /// - `i64`: The number of days in the longest consecutive sequence.
    ///
    /// Returns `None` if the input slice is empty.
    fn find_longest_consecutive_dates(
        dates: &[CalendarAvailabilityDate]
    ) -> Option<(NaiveDate, NaiveDate, i64)> {
        if dates.is_empty() {
            return None;
        }

        let maximum_duration: i64 = 9;
        let mut sorted_dates: Vec<_> = dates
            .iter()
            .map(|d| d.to_naive_date())
            .collect();
        sorted_dates.sort();

        let mut longest_start = sorted_dates[0];
        let mut longest_end = sorted_dates[0];
        let mut current_start = sorted_dates[0];
        let mut current_end = sorted_dates[0];

        for i in 1..sorted_dates.len() {
            if sorted_dates[i] - sorted_dates[i - 1] == chrono::Duration::days(1) {
                current_end = sorted_dates[i];
            } else {
                let duration: i64 = (current_end - current_start).num_days() + 1;
                if
                    duration > (longest_end - longest_start).num_days() &&
                    duration <= maximum_duration
                {
                    longest_start = current_start;
                    longest_end = current_end;
                } else if duration > maximum_duration {
                    // If the current duration exceeds the maximum, adjust the end date to fit the maximum
                    longest_end = current_start + chrono::Duration::days(maximum_duration - 1);
                }
                current_start = sorted_dates[i];
                current_end = sorted_dates[i];
            }
        }

        let duration: i64 = (current_end - current_start).num_days() + 1;
        if duration > (longest_end - longest_start).num_days() && duration <= maximum_duration {
            longest_start = current_start;
            longest_end = current_end;
        } else if duration > maximum_duration {
            // Adjust the end date to fit the maximum if the final duration exceeds it
            longest_end = current_start + chrono::Duration::days(maximum_duration - 1);
        }

        Some((longest_start, longest_end, (longest_end - longest_start).num_days() + 1))
    }

    /// Extracts date information from a given label string.

    /// This function parses a given string (typically a label from a web element)
    /// to extract the weekday, day, month, and year.
    ///
    /// # Arguments
    ///
    /// * `label`: The input string containing the date information.
    ///
    /// # Returns
    ///
    /// A `CalendarAvailabilityDate` struct containing the extracted weekday, day, month, and year.
    pub fn extract_date_from_label(label: &str) -> CalendarAvailabilityDate {
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

        available_date
    }
}
