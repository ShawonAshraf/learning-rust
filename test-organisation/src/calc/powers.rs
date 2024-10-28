pub fn power(base: u64, exponent: u64) -> u64 {
    let mut result = base;

    for _ in 1..exponent {
        result *= base;
    }

    result
}

pub fn square(n: u64) -> u64 {
    n * n
}
