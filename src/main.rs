use core::panic;

use mylib::helper::{menu,check_range,calculate_power,get_num};
fn main() {
    use std::io;
    let mut base = 1;
    let mut exp = 1;

    loop {
        let mut select = String::new();
        let mut input = String::new();
        menu();

        io::stdin()
            .read_line(&mut select)
            .expect("Failed to read line");

        let select: i32 = match select.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match select {
            1 => {
                let input=get_num(&mut input);
                let ret = check_range(input as i32, 1, 25);
                if ret == 0 {
                    println!("out of range")
                } else if ret == 1 {
                    base = input;
                }
            }
            2 => {
                let input=get_num(&mut input);
                let ret = check_range(input as i32, 1, 5);
                if ret == 0 {
                    println!("out of range")
                } else if ret == 1 {
                    exp = input;
                }
            }
            3 => println!("{base}^{exp}={}", calculate_power(base, exp)),
            4 => break,
            _ => continue,
        };
    }
}

