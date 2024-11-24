pub mod structs {
    #[derive(Debug)]
    pub struct CalendarAvailabilityDate {
        pub weekday: String,
        pub day: String,
        pub month: String,
        pub year: String
    }

    impl PartialEq for CalendarAvailabilityDate {
        fn eq(&self, other: &Self) -> bool {
            self.weekday == other.weekday
            && self.day == other.day 
            && self.month == other.month
            && self.year == other.year 
        }
    }

    pub struct SiteURL<'a> {
        pub site_number: i32,
        pub url: &'a str
    }
}