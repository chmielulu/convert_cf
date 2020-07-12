use std::io;
use std::io::Write;

fn main() {

    println!("<!----Convert----!>");
    println!("1. Celsius to Fahrenheit");
    println!("2. Fahrenheit to Celsius");

    let choose: u8 = get_choose();

    clear_console();

    let grades: i32 = get_temperature();

    if choose == 1 {
        print!("{} °C is {} °F", grades, (grades * 9/5) + 32);
    } else {
        print!("{} °F is {} °C", grades, (grades - 32) * 5/9);
    }
    
}

fn get_choose() -> u8 {
    
    let choose: u8 = loop {
        print!("Please choose option: ");
        io::stdout().flush().unwrap();

        let mut choose = String::new();
    
        io::stdin()
            .read_line(&mut choose)
            .expect("Failed to read line!");

        match choose.trim().parse() {
            Ok(num) => {
                if num == 1 || num == 2 { break num; }
                else { continue; }
            },
            Err(_) => continue,
        };
    };
    
    choose
}

fn get_temperature() -> i32 {
    
    let grades: i32 = loop {
        print!("Type the temperature: ");
        io::stdout().flush().unwrap();
        
        let mut grades = String::new();

        io::stdin()
            .read_line(&mut grades)
            .expect("Failed to read line!");
    
        match grades.trim().parse() {
            Ok(grad) => break grad,
            Err(_) => continue,
        };
    };

    grades
}

fn clear_console() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}