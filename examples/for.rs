pub fn main() {
    let mut vec = vec![1, 2, 3];

    for item in vec {
        println!("{}", item);
    }

    vec[0] = 0;
    println!("{}", vec[0]);
}
