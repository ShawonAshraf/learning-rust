// the language in the rust docs is nauseating
// you guys never took a language comprehension class at school?
// anyway

// here's a generic function that returns the smallest number from a list
// why return a reference? To borrow and not move the element from the list

// now what happens if you pass a literal instead of a reference?
// not supported by the rust compiler or the PartialOrd for comparison
// since the size of T isn't known at compile time
// but from the reference of an actual var, it can be inferred
// same reason the return type is also a reference
fn smallest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut min = &list[0];
    for e in list {
        if e < min {
            min = e;
        }
    }

    min
}

// generic struct with single type

#[derive(Debug)]
struct SingleGeneric<T> {
    x: T, y: T
}

// multiple types
#[derive(Debug)]
struct MultiGeneric<T, U> {
    x: T, y: U
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    println!("smallest : {}", smallest(&number_list));

    let single = SingleGeneric {x: 4, y: 5};
    println!("SingleGeneric : {:?}", single);
    let mgen = MultiGeneric {x: 4.5, y: 9.9};
    println!("SingleGeneric : {:?}", mgen);

}
