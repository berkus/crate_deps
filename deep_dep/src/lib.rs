pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(feature = "private_apis")]
pub mod private_apis;

#[doc(hidden)]
pub fn add_private(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
