use std::process::exit;

mod db_connection;

fn main() {
    get_menu();
}

fn get_menu() {
    loop {
        println!("TODO APP");
        println!("1. insert new todo");
        println!("2. set todo to done");
        println!("3. show open todos");
        println!("4. delete todo");
        println!("5. recreate db");
        println!("6. exit");
        let input = get_console_input("enter number to select");
        match input.as_str() {
            "1" => db_connection::insert_todo(get_console_input("Insert new todo name")),
            "3" => { db_connection::get_open_todos(); }
            "5" => db_connection::recreate_db(),
            "6" => exit(0),
            _ => println!("{} is not a valid menu option. try again", input)
        }
    }
}

fn get_console_input(input_text: &str) -> String {
    use std::io::{stdin, stdout, Write};
    let mut s = String::new();
    println!("{}", input_text);
    let _ = stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }
    return s;
}
