pub mod state {
    use std::fs;
    use std::io::Error;

    pub fn message_changed(new_message: &str) -> Result<bool, Error> {
        let file_path: &str = "last_message_sent.txt";
        if !fs::metadata(file_path).is_ok() {
            // File doesn't exist, create it
            fs::write(file_path, new_message)?;
            log::info!("File created with new state.");
            return Ok(true);
        }

        // File exists, read its content
        let current_state: String = fs::read_to_string(file_path)?;

        // Compare the current state with the new state
        if current_state != new_message {
            // Overwrite the file with the new state
            fs::write(file_path, new_message)?;
            log::info!("File overwritten with new state.");
            return Ok(true);
        }

        log::info!("File content is already up-to-date.");
        Ok(false)
    }
}
