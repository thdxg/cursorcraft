use rand::seq::index;

const BRAILLE_MAX_DOTS: usize = 8;
const BRAILLE_START_UNICODE: u32 = 0x2800;

pub fn braille_with_count(count: usize) -> char {
    assert!(count < BRAILLE_MAX_DOTS);
    let mask = index::sample(&mut rand::rng(), 8, count)
        .into_iter()
        .fold(0x0, |mask, bit| mask | 1 << bit);
    char::from_u32(BRAILLE_START_UNICODE + mask).unwrap()
}
