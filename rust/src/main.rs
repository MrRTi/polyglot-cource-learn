fn main() {
    let mut x = 5;
    let z = &mut x;
    *z = 7;
    println!("{:?}", z);
}
