// mod front_of_house;
pub mod calc;

// pub use crate::front_of_house::hosting;
//
// pub fn eat_at_restaurant() {
//     hosting::add_to_waitlist();
// }

/*
So here's how the module system in rust works. At the top crate level
you make a file that is the name of the module, then declare that in your lib.rs
and then for submodules you create a directory with the same name and put the subs in there

the module scope is private by default so make sure to explicitly mention public when you need
to.

*/