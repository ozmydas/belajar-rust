#[path = "conversion.rs"]
pub mod conversion_utility;

pub fn is_odd_number(number: i32) -> bool {
    return number % 2 == 1
}
