use rand::distributions::Alphanumeric;
use rand::Rng;

pub fn generate(len: usize) -> String {
    let mut rng = rand::thread_rng();
    let random_string: String = rng
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect();
    random_string
}