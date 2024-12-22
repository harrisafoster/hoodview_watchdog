pub mod structs {
    use chrono::NaiveDate;

    #[derive(Debug, Clone, PartialEq)]
    pub struct CalendarAvailabilityDate {
        pub weekday: String,
        pub day: String,
        pub month: String,
        pub year: String,
    }

    impl CalendarAvailabilityDate {
        /// Converts the `CalendarAvailabilityDate` to a `NaiveDate` struct.

        /// This function converts the calendar availability date information stored in the struct
        /// (year, month, day) to a `NaiveDate` struct, which represents a date without time zone information.

        /// # Returns
        ///
        /// A `NaiveDate` struct representing the date.
        pub fn to_naive_date(&self) -> NaiveDate {
            let month_str: &str = match self.month.as_str() {
                "January" => "01",
                "February" => "02",
                "March" => "03",
                "April" => "04",
                "May" => "05",
                "June" => "06",
                "July" => "07",
                "August" => "08",
                "September" => "09",
                "October" => "10",
                "November" => "11",
                "December" => "12",
                _ => unreachable!(), // Month is guaranteed to be in the list
            };

            let date_str: String = format!("{}-{}-{}", self.year, month_str, self.day);
            NaiveDate::parse_from_str(&date_str, "%Y-%m-%d").unwrap()
        }
    }

    pub struct SiteURL<'a> {
        pub site_number: i32,
        pub url: &'a str,
    }
}
