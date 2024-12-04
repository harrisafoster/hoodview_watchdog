pub mod notify {
    use lettre::message::header::ContentType;
    use lettre::transport::smtp::authentication::Credentials;
    use lettre::{ Message, SmtpTransport, Transport };
    use dotenv;

    pub fn send_notification(subject: String, message: String) {
        dotenv::dotenv().ok();

        log::info!("Attempting to get secret info...");
        let hvwd_username: String = dotenv::var("HVWD_USERNAME").unwrap_or_else(|err| {
            log::error!("Failed to read HVWD_USERNAME: {}", err);
            panic!("Missing required environment variable: HVWD_USERNAME");
        });
        let hvwd_app_password: String = dotenv::var("HVWD_APP_PASSWORD").unwrap_or_else(|err| {
            log::error!("Failed to read HVWD_APP_PASSWORD: {}", err);
            panic!("Missing required environment variable: HVWD_APP_PASSWORD");
        });
        let hvwd_recipients: String = dotenv::var("HVWD_RECIPIENTS").unwrap_or_else(|err| {
            log::error!("Failed to read HVWD_RECIPIENTS: {}", err);
            panic!("Missing required environment variable: HVWD_RECIPIENTS");
        });
        log::info!("Secret info successfully retrieved.");
        
        hvwd_recipients.split(",").for_each(|hvwd_recipient| {
            let recipient: String = format!("Hopeful Campsite Reserver <{}>", hvwd_recipient);
            let sender: String = format!("The Hoodview Watchdog <{}>", &hvwd_username);
            let creds: Credentials = Credentials::new(hvwd_username.to_string(), hvwd_app_password.to_string());
        
            log::info!("Attempting to send email notification to {}", hvwd_recipient);
            let email: Message = Message::builder()
                .from(sender.parse().unwrap())
                .to(recipient.parse().unwrap())
                .subject(&subject)
                .header(ContentType::TEXT_HTML)
                .body(String::from(&message))
                .unwrap();

            // Open a remote connection to gmail
            let mailer: SmtpTransport = SmtpTransport::relay("smtp.gmail.com")
                .unwrap()
                .credentials(creds)
                .build();

            // Send the email
            match mailer.send(&email) {
                Ok(_) => log::info!("Email sent successfully to {}", hvwd_recipient),
                Err(e) => log::error!("Email not sent successfully due to error: {}", e),
            }
        });
    }
}
