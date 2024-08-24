use rand::{distributions::Alphanumeric, thread_rng, Rng};

pub async fn route() -> String {
    let rand_string = thread_rng()
        .sample_iter(&Alphanumeric)
        .map(char::from)
        .take(8)
        .collect();

    rand_string
}