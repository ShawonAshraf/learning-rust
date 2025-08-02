// traits are akin to what you'd call abstract classes in an OOP based
// language to define common functionalities for multiple types
// in rust, traits are implemented by a type and can also be passed to a function
// as param (or super type) and returned as well.
// a type can implement multiple traits

use std::fmt::Display;

trait WhatSay {
    // a method which will be implemented
    // can also have a default impl
    fn say(&self) -> String;
}


struct Animal {
    name: String,
}

// impl Whatsay for animal
impl WhatSay for Animal {
    fn say(&self) -> String {
        format!("But what does the {} say?", &self.name)
    }
}

// trait as a param
// this tells the compiler that any type that impl WhatSay can be used here

// this is similar to naming the function as:
// fn call animal<T: WhatSay>(animal: &T)
// fn call(animal: &impl WhatSay) {
//     println!("{}", animal.say());
// }


// multiple traits?
impl Display for Animal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("Name: {}", &self.name))
    }
}

// now call be updated as such
fn call(animal: &(impl WhatSay + Display)) {
    println!("{}", animal.say());
}

// or with the where clause
// useful for multiple generic types
fn call_generic<T>(animal: &T)
where
    T: WhatSay + Display,
{
    println!("{}", animal.say());
}

fn main() {
    let fox = Animal { name: String::from("fox") };
    call(&fox);
    call_generic(&fox);
}
