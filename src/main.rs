// what happens when you try to access an out of bound index in rust?
// panic and crash. it doesn't print a garbage value like C or C++
fn arr_size() {
    let array: [isize; 5] = [1,2,3,4,5];
    let s: usize = array.len();
    println!("{s}");
}

/*
expressions vs statements in rust
expressions - must return a value, e.g. function calls or operations
statements - don't return a value, variable assignment
*/

fn statement_expression() {
    // this is a  statement
    let y: isize = 6; // also a note, use isize when you're not sure about the arch
    
    // this is also an expression
    // since it evaluates to a string (or returns a string to stdout)
    // println is a macro
    print!("{y}");
}


fn if_else() {
    let condition: bool = true;
    // you can also do this, conditional assignment
    let number: i64 = if condition { 5 } else { 6 };
    println!("{number}")
}

fn main() {
    // arr_size();
    // statement_expression();
    if_else();
}
