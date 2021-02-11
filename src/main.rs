fn main() {
    println!("Hello, world!");
    println!("{}", test(10, 12));
}

fn test(a: u64, b: u64) -> u64 {
    let c: u64 = a + b;
    c
}

