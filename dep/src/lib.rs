use deep_dep::private_apis::__private_api_DO_NOT_USE::__private_do_not_use_add;

pub fn user_facing(left: u64, right: u64) -> u64 {
    __private_do_not_use_add(left, right)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = user_facing(2, 2);
        assert_eq!(result, 4);
    }
}
