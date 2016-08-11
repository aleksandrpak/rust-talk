#[derive(Debug)]
struct Bar {
    counter: usize,
}

#[derive(Debug)]
struct Foo {
    bar: Bar,
}

impl Foo {
    pub fn new() -> Foo {
        Foo { bar: Bar { counter: 1 } }
    }

    pub fn get_bar(&self) -> &Bar {
        &self.bar
    }
}

pub fn main() {
    let mut foo = Foo::new();
    println!("{:?}", foo);

    {
        let mut bar = foo.get_bar();
        bar.counter += 1;
    }

    println!("{:?}", foo);
}
