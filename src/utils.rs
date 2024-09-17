pub fn print_header_table(type_table: &str) {
    println!("+-----------------------------+");
    println!("|       {}         |", type_table);
    println!("+-----------------------------+");
}


pub fn print_table() {
    println!("+------------------------------------+");
    println!("+ cheack box | id | title | status | +");
    println!("+------------------------------------+");
}


// Вспомогательная функция для ввода строки
pub fn input(prompt: &str) -> String {
    use std::io::{self, Write};
    let mut buffer = String::new();
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().to_string()  // убираем лишние символы
}

pub fn ask_user(prompt: &str) -> bool {
    input(prompt).to_lowercase() == "y"
}


