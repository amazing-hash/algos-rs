#[allow(unused)]
fn xorshift_32(seed: u32) -> impl Iterator<Item = u32> {
    let mut value = seed;
    std::iter::repeat_with(move || {
        value ^= value << 13;
        value ^= value >> 17;
        value ^= value << 5;
        value
    })
}
