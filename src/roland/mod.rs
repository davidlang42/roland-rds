pub mod rd300nx;
pub mod live_set;
pub mod layers;
pub mod tones;
pub mod system;
pub mod types;

#[cfg(test)]
mod tests;

fn sum_to_zero(sum: u16) -> u8 {
    (u8::MAX - sum.to_be_bytes()[1]).wrapping_add(1)
}