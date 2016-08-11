#[derive(Debug)]
pub struct Foo {
    bar: String,
}

impl Foo {
    pub fn new(bar: String) -> Foo {
        Foo { bar: bar }
    }
}

pub fn main() {
    let foo = Foo::new("bar");
    // let foo = Foo::new(String::from("bar"));

    println!("{:?}", foo);
}
