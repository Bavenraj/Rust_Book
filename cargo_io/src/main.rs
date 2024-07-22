use art::kinds::PrimaryColor;
use art::utils::mix;
/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
/// 

pub fn add_oe(x: i32) -> i32 {
    x + 1
}
/// Adds two number given.
///
/// # Examples
///
/// ```
/// let arg1 = 5;
/// lat arg2 = 10;
/// let answer = my_crate::add_one(arg1, arg2);
///
/// assert_eq!(15, answer);
/// ```
pub fn add_two(x: i32, y: i32) -> i32 {
    x + y + 1
}

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}