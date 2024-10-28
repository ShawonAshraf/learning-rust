#![cfg(test)]
mod test_basics {
    use crate::calc::basics;

    #[test]
    fn test_add() {
        assert_eq!(10, basics::add(5, 5));
    }

    #[test]
    fn test_sub() {
        assert_eq!(0, basics::sub(5, 5));
    }

    #[test]
    fn test_mul() {
        assert_eq!(25, basics::mul(5, 5));
    }

    #[test]
    fn test_div() {
        assert_eq!(1, basics::div(5, 5));
    }
}
