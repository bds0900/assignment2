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
// fn menu() {
//     println!("1. Change base");
//     println!("2. Change exponent");
//     println!("3. Display base raised to exponent");
//     println!("4. Exit program");
// }

// fn check_range(val: i32, min: i32, max: i32) -> i32 {
//     let mut ret = 0;
//     if val < min || val > max {
//         ret = 0;
//     } else {
//         ret = 1;
//     }
//     ret
// }
// fn calculate_power(base: i32, exp: i32) -> i32 {
//     if exp > 1 {
//         return base * calculate_power(base, exp - 1);
//     } else {
//         return base;
//     }

// }
// fn get_num(mut input: &mut String)->i32 {
//     println!("Enter an integer number");
//     use std::io;
//     io::stdin()
//         .read_line(&mut input)
//         .expect("Failed to read line");

//     let input: i32 = match input.trim().parse() {
//         Ok(num) => num,
//         Err(_) => 0,
//     };
//     input
// }
