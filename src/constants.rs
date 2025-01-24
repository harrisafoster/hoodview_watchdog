pub mod constants {
    use crate::structs::structs::SiteURL;
    
    /// Constant representing the URL for campsite number 4 on recreation.gov.
    pub const SITE4: SiteURL = SiteURL { site_number: 4, url: "https://www.recreation.gov/camping/campsites/82694" };

    /// Constant representing the URL for campsite number 5 on recreation.gov.
    pub const SITE5: SiteURL = SiteURL { site_number: 5, url: "https://www.recreation.gov/camping/campsites/82039" };

    /// Constant representing the URL for campsite number 6 on recreation.gov.
    pub const SITE6: SiteURL = SiteURL { site_number: 6, url: "https://www.recreation.gov/camping/campsites/82684" };

    /// Constant representing the URL for campsite number 23 on recreation.gov.
    pub const SITE23: SiteURL = SiteURL { site_number: 23, url: "https://www.recreation.gov/camping/campsites/82818" };

    /// Constant representing the URL for campsite number 24 on recreation.gov.
    pub const SITE24: SiteURL = SiteURL { site_number: 24, url: "https://www.recreation.gov/camping/campsites/82979" };

    /// Constant representing the URL for campsite number 26 on recreation.gov.
    pub const SITE26: SiteURL = SiteURL { site_number: 26, url: "https://www.recreation.gov/camping/campsites/82154" };

    /// Constant representing an empty SiteURL object
    pub const SITE_EMPTY: SiteURL = SiteURL { site_number: 0, url: ""};

    /// Array containing constants for all campsite URLs (excluding SITE_EMPTY).
    pub const ALL_SITE_URLS: [SiteURL<'_>; 6] = [SITE4, SITE6, SITE23, SITE24, SITE26, SITE5];
}