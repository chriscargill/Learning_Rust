fn main() {
    let mut data: Vec<User> = Vec::new();
    println!("Hello, world!");
    println!("{}", add(10, 12));
    println!("{}", greatest_common_denominator(1337,7));
    data = mutable_data(data); // pass ownership back to the main function by storing it in the variable data
    data[1].username = String::from("Chris");
    data[1].active = true;
    println!("{}_{}:{}/{}", data[1].username, data[1].email, data[1].hashed_pass, data[1].active);
}

fn add(a: u64, b: u64) -> u64 {
    let c: u64 = a + b;
    c
}

fn greatest_common_denominator(mut first_num: u64, mut second_num: u64) -> u64 {
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


struct User {
    username: String,
    email: String,
    hashed_pass: u128,
    active: bool,
}

fn mutable_data(mut data: Vec<User>) -> Vec<User> {

    for mut every_number in 0..10 {
        if every_number % 16 == 0 || every_number % 50 == 0{ // set some values to be zeros
            every_number = 0;
        }
        let user = User{ // add the user struct to data vector
            username: String::from("none"), 
            email: String::from("default@home.com"), 
            hashed_pass: every_number, 
            active: false};
        println!("{}_{}:{}/{}", user.username, user.email, user.hashed_pass, user.active);
        data.push(user);
    }
    return data;
}
