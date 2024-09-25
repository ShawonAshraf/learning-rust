use std::fmt::Debug;

// structs can be used to create custom data types
// consider data classes in python
// or structs from C
struct Person {
    name: String,
    active: bool
}



fn main() {
    // just normally create a var
    let p1: Person = Person {
        name: String::from("p1"),
        // : is used for assignment
        active: false
    };
    println!("{} -> {}", &p1.name, &p1.active);
    
    // creating obj using another obj props
    
    let p2: Person = Person {
        name: p1.name,
        active: p1.active
    };
    println!("{} -> {}", &p2.name, &p2.active);
    
    // is p1 still in scope or dropped?
    // p1 will get  moved since, name is a string and is not copied
    // doesn't have copy implementation actually.
    // 


    // what if references are used?
    // nope. doesn't work.

    // field init shortcut
    let name: String = String::from("p3");
    let active: bool = true;


    let p3: Person = Person {
        name, active
    };
    println!("{} -> {}", &p3.name, &p3.active);

    // tuple structs
    struct Colour (i32, i32, i32);
    let colour: Colour = Colour(0, 0, 1);
    let blue: i32 = colour.2;
    // how to access this?
    println!("{blue}");

    // structs without any fields
    // perhaps as types?
    struct NoFields;
    let ff: NoFields = NoFields; 


    // can't print structs directly with println
    // the debug format can help for debug logs though
    // (or implement Display for the struct)

    #[derive(Debug)]
    struct Cat {
        breed: String,
        age: i32
    };

    let c1: Cat = Cat { breed: String::from("Ragdoll"), age: 2 };
    // println!("c1 is {c1:?}");

    // another way is

    // dbg returns ownership, so doesn't get dropped
    dbg!(&c1);

    // methods in strcucts
    #[derive(Debug)]
    struct Rectangle {
        w: i32,
        h: i32
    };

    // add a method

    // a struct can have multiple blocks
    impl Rectangle {
        // &self is a short form for self: &Self
        // recall self from python and swift
        fn area(&self) -> i32 {
            // one line returns
            self.h * self.w
        }
    }

    let r1: Rectangle = Rectangle {w:16, h:10};
    let area: i32 = r1.area();
    dbg!(area);
    dbg!(r1);

    // associated methods
    // which change or add to the struct itself
    impl Rectangle {
        fn make_square(size: i32) -> Self {
            Self {
                h: size,
                w: size
            }
        }
    }

    // like String::from ? 
    let square: Rectangle = Rectangle::make_square(32);
    let area_2: i32 = square.area();
    dbg!(&area_2);

}
