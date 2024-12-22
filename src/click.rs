pub mod click {
    use std::{ thread, time::Duration };

    use thirtyfour::{ error::WebDriverError, prelude::ElementWaitable, By, WebDriver, WebElement };

    /// Attempts to find a WebElement with retries.

    /// This function repeatedly attempts to find a WebElement using the given CSS query
    /// and the provided WebDriver instance.
    /// It includes retry logic with exponential backoff to handle transient errors.

    /// # Arguments
    ///
    /// * `css_query`: The CSS query to locate the WebElement.
    /// * `driver`: A reference to the WebDriver instance.
    ///
    /// # Returns
    ///
    /// A `Result` containing the found WebElement if successful,
    /// or a `WebDriverError` if the retries are exhausted.
    pub async fn find_element_with_retries(
        css_query: &str,
        driver: &WebDriver
    ) -> Result<WebElement, WebDriverError> {
        log::info!("Attempting to get WebElement by CSS query: {}", css_query);

        let mut retries: i32 = 3;
        let mut delay: Duration = Duration::from_secs(3);

        loop {
            match driver.find(By::Css(css_query)).await {
                Ok(button) => {
                    log::info!("WebElement retrieved.");
                    return Ok(button);
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
    }

    /// Scrolls to a WebElement and clicks it.

    /// This function attempts to find a WebElement using the given CSS query,
    /// scrolls the element into view if necessary, waits for it to be clickable,
    /// and then clicks it. It incorporates error handling for cases where the
    /// element cannot be found.

    /// # Arguments
    ///
    /// * `driver`: A reference to the WebDriver instance.
    /// * `css_query`: The CSS query to locate the WebElement.
    pub async fn scroll_and_click(driver: &WebDriver, css_query: &str) {
        match find_element_with_retries(css_query, &driver).await {
            Ok(button) => {
                button.wait_until().displayed().await.unwrap();
                button.scroll_into_view().await.unwrap();
                button.wait_until().clickable().await.unwrap();
                button.click().await.unwrap();
            }
            Err(e) => {
                log::warn!("Could not find button: {:?}", e);
            }
        }
    }
}
