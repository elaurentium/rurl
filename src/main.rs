fn main() {
    let mut x;
    for i in 0..1000000 {
        x = i % 10 * i;
        println!("{}\n", x);
    }
}
