use postgres::{Client, NoTls};
#[macro_use]
extern crate fstrings;


fn main() {
    let mut data: Vec<User> = Vec::new();
    println!("Hello, world!");
    println!("{}", add(10, 12));
    println!("{}", greatest_common_denominator(1337,7));
    data = mutable_data(data); // pass ownership back to the main function by storing it in the variable data
    data[1].username = String::from("Chris");
    data[1].active = true;
    println!("{}_{}:{}/{}", data[1].username, data[1].email, data[1].hashed_pass, data[1].active);
    add_user("pythonz", "applesauce");
}

fn add_user(username: &str, password: &str) -> u8 { // Connect to postgres database
    let pass  = "";
    // postgresql://user[:password]@host[:port][/database][?param1=val1[[&param2=val2]...]]
    let url = f!("postgres://postgres:{}@localhost:5432/rust_test", &pass);
    let mut client = Client::connect(&url, NoTls).unwrap();


    client.execute(
        "INSERT INTO users (username, password) VALUES ($1, $2)",
        &[&username, &password]
    ).unwrap();

    for row in client.query("SELECT username, password FROM users", &[]).unwrap() {
        let username: &str = row.get(0);
        let password: &str = row.get(1);

        println!("found person: {} {:?}", username, password);
    }
    200
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