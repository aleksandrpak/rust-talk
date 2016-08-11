#[derive(Debug)]
pub struct Foo {
    bar: String,
}

impl Foo {
    pub fn new<S: Into<String>>(bar: S) -> Foo {
        Foo { bar: bar.into() }
    }
}

pub fn main() {
    let foo = Foo::new("borrowed");
    println!("{:?}", foo);

    let foo = Foo::new(String::from("owned"));
    println!("{:?}", foo);
}
