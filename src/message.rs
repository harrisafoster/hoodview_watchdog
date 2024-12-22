pub mod message {
    use crate::{
        constants::constants::{ ALL_SITE_URLS, SITE_EMPTY },
        structs::structs::{ CalendarAvailabilityDate, SiteURL },
    };

    /// Generates a formatted message based on campsite availabilities.

    /// This function takes a vector of campsite availabilities as input and generates
    /// a formatted HTML message string. The message includes:
    ///
    /// - A greeting.
    /// - Information about available campsites:
    ///     - Site number.
    ///     - Whether the site is available or in the cart.
    ///     - A table displaying available dates (weekday, day, month, year).
    ///     - A link to the campsite details or reservation confirmation page.
    /// - A closing message.
    /// - Version information.
    ///
    /// If no campsites are available or have reservations, the function returns "None".
    ///
    /// # Arguments
    ///
    /// * `all_site_availability`: A vector containing information about campsite availabilities.
    ///   Each element is a tuple containing:
    ///     - Site number (i32)
    ///     - A tuple containing:
    ///         - Vector of `CalendarAvailabilityDate` representing available dates for the site.
    ///         - A boolean flag indicating whether the site is in the cart (True) or available (False).
    ///
    /// # Returns
    ///
    /// A formatted HTML message string containing campsite availability information.
    pub fn generate_message(
        all_site_availability: Vec<(i32, (Vec<CalendarAvailabilityDate>, bool))>
    ) -> String {
        log::info!("Building message...");
        let mut message: String =
            "<section style=\"box-sizing: border-box; max-width: fit-content;\"><p>Hey there campsite holder, </p>".to_string();

        if has_at_least_one_reserved_vector(&all_site_availability) {
            for site in all_site_availability {
                if !site.1.0.is_empty() {
                    let site_info: String;
                    if site.1.1 {
                        site_info = format!(
                            "<article style=\"max-width: fit-content; margin-bottom: 16px; padding: 8px; border: 1px solid slategray; border-radius: 6px;\"><p style=\"width: 100%; padding: 0px 0px 6px 0px; text-align: center;\"><b>Site {}</b> is in your cart: </p>",
                            site.0
                        );
                    } else {
                        site_info = format!(
                            "<article style=\"max-width: fit-content; margin-bottom: 16px; padding: 8px; border: 1px solid slategray; border-radius: 6px;\"><p style=\"width: 100%; padding: 0px 0px 6px 0px; text-align: center;\"><b>Site {}</b> is available: </p>",
                            site.0
                        );
                    }

                    message.push_str(&site_info);

                    // Headers
                    let table_header_names: [&str; 4] = ["Weekday", "Day", "Month", "Year"];
                    let mut table_headers: String = "".to_string();
                    for name in table_header_names {
                        let formatted_header: String =
                            format!("<th scope=\"col\" style=\"font-weight: bold;\">{}</th>", name);
                        table_headers.push_str(&formatted_header);
                    }

                    // Rows
                    let mut table_rows: String = "".to_string();
                    for date in &site.1.0 {
                        let formatted_row = format!(
                            "<tr><td>{}</td><td style=\"font-weight: bold;\">{}</td><td>{}</td><td>{}</td></tr>",
                            date.weekday,
                            date.day,
                            date.month,
                            date.year
                        );
                        table_rows.push_str(&formatted_row);
                    }

                    let formatted_final_table: String = format!(
                        "<table style=\"table-layout: auto; border: 1px solid black; padding: 4px; border-radius: 6px;\"><thead style=\"background: darkolivegreen; color: white;\"><tr>{}</tr></thead><tbody>{}</tbody></table>",
                        table_headers,
                        table_rows
                    );
                    message.push_str(&formatted_final_table);
                    let site_url: String = get_site_const(site.0).url.to_string();
                    let url_tip: String;
                    if site.1.1 {
                        url_tip = format!(
                            "<br><a href=\"{}\" style=\"color: green; text-decoration: underline;\">Confirm site {} reservation here!</a><br><p style=\"color: red; font-weight: bold;\">15 minutes left to confirm!</p></article>",
                            "https://www.recreation.gov/cart",
                            site.0
                        );
                    } else {
                        url_tip = format!(
                            "<br><a href=\"{}\" style=\"color: blue; text-decoration: underline;\">Check out site {} availability here!</a></article>",
                            site_url,
                            site.0
                        );
                    }

                    message.push_str(&url_tip);
                }
            }
        } else {
            return "None".to_string();
        }

        message.push_str(
            "<br>May the odds be ever in your favor...<br><br> -Ya main man Harris</section>"
        );
        let version_message: String = format!(
            "<br><br>Brought to you by the HoodviewWatchdog version: {}",
            env!("CARGO_PKG_VERSION")
        );
        message.push_str(&version_message);
        log::info!("Message built.");
        message
    }

    /// Checks if any site in the given vector has been reserved.

    /// This function iterates through a vector of tuples, where each tuple represents a site
    /// and its availability data. It checks the boolean flag in each tuple to determine
    /// if the corresponding site has been reserved.

    /// # Arguments
    ///
    /// * `parent_vector`: A vector of tuples, where each tuple contains:
    ///     - Site number (i32)
    ///     - A tuple containing:
    ///         - Vector of `CalendarAvailabilityDate` representing available dates for the site.
    ///         - A boolean flag indicating whether the site is in the cart (True) or available (False).
    ///
    /// # Returns
    ///
    /// `true` if at least one site in the vector has been reserved, `false` otherwise.
    fn has_at_least_one_reserved_vector(
        parent_vector: &Vec<(i32, (Vec<CalendarAvailabilityDate>, bool))>
    ) -> bool {
        for (_, (_, reserved)) in parent_vector.iter() {
            if *reserved {
                log::info!("At least one site has been added to cart.");
                return true;
            }
        }
        log::warn!("No sites have been reserved.");
        false
    }

    /// Retrieves a SiteURL struct based on the given site number.

    /// This function iterates through a list of `SiteURL` structs and returns the struct
    /// that corresponds to the provided `site_number`.

    /// # Arguments
    ///
    /// * `site_number`: The integer representing the site number.
    ///
    /// # Returns
    ///
    /// The `SiteURL` struct corresponding to the given `site_number`,
    /// or `SITE_EMPTY` if no matching site is found.
    fn get_site_const(site_number: i32) -> SiteURL<'static> {
        for site_url in ALL_SITE_URLS {
            if site_number == site_url.site_number {
                return site_url;
            }
        }
        SITE_EMPTY
    }
}
