use std::borrow::Cow;

#[derive(Debug)]
pub struct Foo<'a> {
    bar: Cow<'a, str>,
}

// pub enum Cow<'a, B> where B: 'a + ToOwned + ?Sized {
//     Borrowed(&'a B),
//     Owned(B::Owned),
// }

impl<'a> Foo<'a> {
    pub fn new<S: Into<Cow<'a, str>>>(bar: S) -> Foo<'a> {
        Foo { bar: bar.into() }
    }
}

pub fn main() {
    let foo = Foo::new("borrowed");
    println!("{:?}", foo);

    let foo = Foo::new(String::from("owned"));
    println!("{:?}", foo);
}
