fn main() {
    println!("Hello, world!");
    println!("{}", add(10, 12));
    println!("{}", greatest_common_denominator(300,60));
}

fn add(a: u64, b: u64) -> u64 {
    let c: u64 = a + b;
    c
}

fn greatest_common_denomenator(mut first_num: u64, mut second_num: u64) -> u64 {
    while first_num != 0 {
        if first_num < second_num {  // if the first number is not the largest, switch the two numbers
            let temp_number: u64 = first_num;
            first_num = second_num;
            second_num = temp_number;
        }
        first_num = first_num % second_num
    }
    second_num

}