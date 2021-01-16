use rand::seq::SliceRandom;

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789)(*&^%$#@!~";
const PASSWORD_LEN: usize = 16;

pub fn random_pass() -> String {
    let mut rng = rand::thread_rng();

    let password: String = (0..PASSWORD_LEN)
        .map(|_| char::from(*CHARSET.choose(&mut rng).unwrap()))
        .collect();

    password
}

fn main() {
    let password = random_pass();
    println!("{:?}", password);
}
