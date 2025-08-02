// defining a custom smartptr

use std::ops::Deref;

struct CustomBox<T>(T);

impl<T> CustomBox<T> {
    fn new(x: T) -> CustomBox<T> {
        CustomBox(x)
    }
}

// implement deref type for custom box
impl<T> Deref for CustomBox<T> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


fn main() {
    // c style dereferencing
    let a = 100;
    let b = &a;
    
    assert_eq!(100, a);
    assert_eq!(100, *b);
    
    // using Box
    let x = 100;
    // use the Box instead of a reference
    let y = Box::new(x);
    
    assert_eq!(100, *y);
    
    // custom box
    let y_custom_box = CustomBox::new(x);
    assert_eq!(x, *y_custom_box);
}
