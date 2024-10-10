use modules::eat_at_restaurant;
use modules::calc::arithmetic;


fn main() {
    eat_at_restaurant();

    let res: i32 = arithmetic::add(5, 10);
    println!("{:?}", res);
}
