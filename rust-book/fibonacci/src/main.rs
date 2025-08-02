fn fibonacci(n: i64) -> i64 {
    if n < 2 {
        return n;
    }

    return fibonacci(n - 1) + fibonacci(n - 2);
}


fn main() {
    let res: i64 = fibonacci(100);
    println!("{res}");
}
