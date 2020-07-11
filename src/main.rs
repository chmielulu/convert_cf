use std::io;
use std::io::Write;

fn main() {

    println!("<!----Convert----!>");
    println!("1. Celsius to Fahrenheit");
    println!("2. Fahrenheit to Celsius");

    let choose: u8 = get_choose();

    clear_console();

    let number: i32 = get_number();

    if choose == 1 {
        print!("{} celsius is {} fahrenheit", number, (number * 9/5) + 32);
    } else {
        print!("{} fahrenheit is {} celsius", number, (number - 32) * 5/9);
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

fn get_number() -> i32 {
    let mut number = String::new();

    let number: i32 = loop {
        print!("Type a number: ");
        io::stdout().flush().unwrap();
        
        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read line!");
    
        match number.trim().parse() {
            Ok(num) => break num,
            Err(_) => continue,
        };
    };

    number
}

fn clear_console() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}