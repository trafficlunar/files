use rand::{thread_rng, Rng};

pub fn generate_password() -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";
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
}