pub mod login {
    use thirtyfour::{ prelude::ElementWaitable, By, WebDriver, WebElement };
    use crate::click::click::{ find_element_with_retries, scroll_and_click };

    /// Logs in to the website using provided credentials.

    /// This function attempts to log in to the specified website using the provided WebDriver instance.
    /// It retrieves the username from an environment variable and attempts to log in using
    /// the provided username and a hardcoded password.

    /// # Arguments
    ///
    /// * `driver`: A reference to the WebDriver instance.
    /// * `site_url`: The URL of the website to log in to.

    /// # Errors
    ///
    /// This function does not currently return any specific errors.
    /// It may panic if the username environment variable is not found.
    pub async fn log_in(driver: &WebDriver, site_url: &str) {
        driver.goto(site_url).await.unwrap();

        dotenv::dotenv().ok();

        log::info!("Attempting to get secret info...");
        let hvwd_username: String = dotenv::var("HVWD_USERNAME").unwrap_or_else(|err| {
            log::error!("Failed to read HVWD_USERNAME: {}", err);
            panic!("Missing required environment variable: HVWD_USERNAME");
        });
        let hvwd_rec_password: String = dotenv::var("HVWD_REC_PASSWORD").unwrap_or_else(|err| {
            log::error!("Failed to read HVWD_REC_PASSWORD: {}", err);
            panic!("Missing required environment variable: HVWD_REC_PASSWORD");
        });

        match driver.find(By::Css("button[aria-label='Sign Up or Log In']".to_string())).await {
            Ok(login_button) => {
                login_button.wait_until().clickable().await.unwrap();
                login_button.click().await.unwrap();

                let email_input: WebElement = driver
                    .find(By::Css("input[id='email']".to_string())).await
                    .unwrap();
                email_input.send_keys(hvwd_username).await.unwrap();

                let password_input: WebElement = driver
                    .find(By::Css("input[id='rec-acct-sign-in-password']".to_string())).await
                    .unwrap();
                password_input.send_keys(hvwd_rec_password).await.unwrap();

                scroll_and_click(&driver, "button.rec-acct-sign-in-btn").await;
                match find_element_with_retries("button[aria-label='My Account']", &driver).await {
                    Ok(_) => {
                        log::info!("Successfully logged in.");
                    }
                    Err(e) => {
                        log::error!("Log in not successful, {}", e);
                    }
                };
            }
            Err(_) => {
                log::warn!("Login button not found, skipping login process");
            }
        }
    }
}
