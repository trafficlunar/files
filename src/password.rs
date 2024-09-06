use std::sync::OnceLock;

use rand::{thread_rng, Rng};

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";

pub static PASSWORD: OnceLock<String> = OnceLock::new();

pub fn get_password() -> &'static str {
    PASSWORD
        .get()
        .expect("Password has not been initalized!")
        .as_str()
}

// Used for generating password on start-up
pub fn init_password() {
    PASSWORD.get_or_init(|| {
        // Get .env variables
        let generate_password =
            std::env::var("GENERATE_PASSWORD").unwrap_or_else(|_| "false".to_string()) == "true";

        if generate_password {
            // Get .env variables
            let password_length: usize = std::env::var("GENERATE_PASSWORD_LENGTH")
                .unwrap_or_else(|_| "16".to_string())
                .parse()
                .unwrap_or(16);

            // Start rng thread
            let mut rng = thread_rng();
            // Generate the password
            let generated_password = (0..password_length)
                .map(|_| {
                    let idx = rng.gen_range(0..CHARSET.len());
                    CHARSET[idx] as char
                })
                .collect();

            // Log the generated password
            tracing::info!("password is {}", generated_password);
            generated_password
        } else {
            tracing::warn!("password is not set! anybody can upload, delete, and rename files!");
            "".to_owned()
        }
    });
}
