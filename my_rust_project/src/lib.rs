use std::io;

pub fn read_my_name() {
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    name = name.trim().to_string();
    if name.is_empty() {
        println!("You didn't enter a name!");
    } else {
        println!("Hello, {}! Nice to meet you!", name);
    }
}
