#[cfg(test)]
mod test_powers {
    use crate::calc::powers;

    #[test]
    fn test_power() {
        assert_eq!(4, powers::power(2, 2));
    }

    #[test]
    fn test_square() {
        assert_eq!(4, powers::square(2));
    }
}
