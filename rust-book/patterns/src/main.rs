enum Automanufaktur {
    BMW, Mercedes, AUDI, Volvo, Mini, Porsche
}

fn get_manufaktur_id(manufaktur: Automanufaktur) -> i32 {
    // match is akin to switch in other languages
    match manufaktur {
        Automanufaktur::AUDI => 1,
        Automanufaktur::BMW => 2,
        Automanufaktur::Porsche => 3,
        Automanufaktur::Mercedes => 4,
        Automanufaktur::Mini => 5,
        Automanufaktur::Volvo => 6
    }
}

fn main() {
    let auto_id: i32 = get_manufaktur_id(Automanufaktur::AUDI);
    println!("{auto_id}");
}
