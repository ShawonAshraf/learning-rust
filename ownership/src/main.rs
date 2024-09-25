fn main() {
    // the string will go out of scope once the block exits
    {
        let s: String = String::from("hello from inside the block");
        // you still can't directly print it since 
        // println needs a string literal
        // "hello" is a sting literal
        println!("{s}");
    } // s is no longer valid here
    
    // rust usually calls `drop` to remove unused references
    let s: String = String::from("Hello from outside the block");
    println!("{s}");
    drop(s);

    // this will throw an error
    // println!("{s}")

    // this kind of ownership or refencing is similar to RAII in C++

    let mut x: String = String::from("This is another string x");
    println!("Address of x -> {:p}", &x);
    let y = x;
    println!("Address of y -> {:p}", &y);

    // pointers gets copied and y gets a new address but
    // they still point to the same data in the heap

    // modifying x
    x = String::from("Slightliy modified x");
    println!("y :: {y}");
    println!("x :: {x}");
    // this doesn't modify y, as it still points to the same data in the heap

    // this will throw an error
    // the thing in rust is that, as soon as you copy and bind a val to a var
    // the previous var goes out of scope
    // this avoids dangling reference cycles


    // https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#variables-and-data-interacting-with-move
    // the val gets moved to the new var

    // rust doesn't do deep copy by default
    // but what if you have to?
    // use clone
    let mut a: String = String::from("String a");
    let a_copy = a.clone();

    // addresses will be different
    println!("a -> {:p}", &a);
    println!("a_copy -> {:p}", &a_copy);

    a.push_str("string");
    println!("a -> {a}");
    // surprise, won't change a_copy!
    println!("a_copy -> {a_copy}");

    // however, String stays in the heap
    // data in the stack don't get moved (dropped refs after x = y)
    // it's because stack data, int, float have a predefined size at compile time
    // and can be checked by the compiler
    // P.S. -> heap is used for types where you don't know the size at compile time so you
    // allocate memory during runtime. e.g. the String type.

}
