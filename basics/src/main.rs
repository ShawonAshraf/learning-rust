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


// loops
// there are 3 loops in rust, loop, while, for

fn looping_break() {
    // loop
    let mut counter: i64 = 0;

    // loop can return values with breaks
    let result: i64 = loop {
        counter += 1; // this works for a mutable type? 

        if counter == 10 {
            break counter * 2; // this will retrun the value from the loop
            // but won't exit the function.
            // only retrun will exit the function
        }
    };

    println!("The result is {result}");
}

// okay a bubble sort fn
fn bubble_sort(arr: &[i64]) -> Vec<i64> {
    let size: usize = arr.len();
    // array size has to be a constant and known at
    // compile time. if the size is computed, use a vector
    let mut sorted_arr: Vec<i64> = arr.to_vec();

    // yeah you can assign labels to loops
    for i in 0..size  {
        for j in 0..size - 1 - i {
            if sorted_arr[j] > sorted_arr[j + 1] {
                // yeah you can swap vector elements like that
                sorted_arr.swap(j, j + 1);
            }
        }
    }

    return sorted_arr;
}

fn main() {
    // arr_size();
    // statement_expression();
    // if_else();
    // looping_break();

    let arr: [i64; 5] = [23, 86, -1, 10, -100];
    let sorted_arr: Vec<i64> = bubble_sort(&arr);
    println!("Sorted array: {:?}", sorted_arr);

}
