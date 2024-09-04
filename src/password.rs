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

pub fn init_password() {
    PASSWORD.get_or_init(|| {
        let generate_password =
            std::env::var("GENERATE_PASSWORD").unwrap_or_else(|_| "false".to_string()) == "true";

        if generate_password {
            let password_length: usize = std::env::var("GENERATE_PASSWORD_LENGTH")
                .unwrap_or_else(|_| "16".to_string())
                .parse()
                .unwrap_or(16);

            let mut rng = thread_rng();
            let generated_password = (0..password_length)
                .map(|_| {
                    let idx = rng.gen_range(0..CHARSET.len());
                    CHARSET[idx] as char
                })
                .collect();

            tracing::info!("password is {}", generated_password);
            generated_password
        } else {
            std::env::var("PASSWORD")
                .expect("PASSWORD must be set when GENERATE_PASSWORD is set to false")
        }
    });
}
