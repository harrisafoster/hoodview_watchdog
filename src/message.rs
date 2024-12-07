pub mod message {
    use crate::{
        constants::constants::{ ALL_SITE_URLS, SITE_EMPTY },
        structs::structs::{ CalendarAvailabilityDate, SiteURL },
    };

    pub fn generate_message(
        all_site_availability: Vec<(i32, Vec<CalendarAvailabilityDate>)>
    ) -> String {
        log::info!("Building message...");
        let mut message: String =
            "<section style=\"box-sizing: border-box; max-width: fit-content;\"><p>Hey there potential campsite holder, </p>".to_string();

        if has_non_empty_vector(&all_site_availability) {
            for site in all_site_availability {
                if !site.1.is_empty() {
                    let site_info: String = format!(
                        "<article style=\"max-width: fit-content; margin-bottom: 16px; padding: 8px; border: 1px solid slategray; border-radius: 6px;\"><p style=\"width: 100%; padding: 0px 0px 6px 0px; text-align: center;\"><b>Site {}</b> is available on: </p>",
                        site.0
                    );
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
                    for date in &site.1 {
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
                    let url_tip: String = format!(
                        "<br><a href=\"{}\" style=\"color: green; text-decoration: underline;\">Head here and pick up site {}!</a></article>",
                        site_url,
                        site.0
                    );
                    message.push_str(&url_tip);
                } else {
                    let site_info: String = format!(
                        "<article style=\"max-width: 100%; margin-bottom: 16px; padding: 8px 0px 8px 8px; border: 1px solid slategray; border-radius: 6px;\"><p style=\"color: red;\"><b>Site {} is not available <br>for the next 12 months :( </b></p></article>",
                        site.0
                    );
                    message.push_str(&site_info);
                }
            }
        } else {
            return "None".to_string();
        }

        message.push_str(
            "<br>May the odds be ever in your favor...<br><br> -Ya main man Harris</section>"
        );
        let version_message: String = format!("Brought to you by the HoodviewWatchdog version: {}", env!("CARGO_PKG_VERSION"));
        message.push_str(&version_message);
        log::info!("Message built.");
        message
    }

    fn has_non_empty_vector(parent_vector: &Vec<(i32, Vec<CalendarAvailabilityDate>)>) -> bool {
        for (_, v) in parent_vector.iter() {
            if !v.is_empty() {
                log::info!(
                    "At least one site contained at least one vector containing valid availability."
                );
                return true;
            }
        }
        log::warn!("All sites contained only empty availability vectors.");
        false
    }

    fn get_site_const(site_number: i32) -> SiteURL<'static> {
        for site_url in ALL_SITE_URLS {
            if site_number == site_url.site_number {
                return site_url;
            }
        }
        SITE_EMPTY
    }
}
