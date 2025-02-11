use rand::Rng;

pub fn string(length: u32) -> String {
    let mut rng = rand::rng();
    const CHARSET: &[u8] = "abcdefghijklmnopqrstuvwxyz".as_bytes();
    let mut arr = Vec::new();
    for _ in 0..=length {
        let n = rng.random_range(0..(CHARSET.len()));
        let char = CHARSET[n];
        arr.push(char);
    }

    std::str::from_utf8(&arr[..]).unwrap().to_string()
}
