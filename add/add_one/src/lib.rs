
pub fn add_one(x: i32) -> i32 {
    x + 1
}
pub fn add_two(x: i32) -> i32 {
    x +2 
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(3, add_one(2));
    }
}