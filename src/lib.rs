pub mod helper {
    pub fn menu() {
        println!("1. Change base");
        println!("2. Change exponent");
        println!("3. Display base raised to exponent");
        println!("4. Exit program");
    }

    pub fn check_range(val: i32, min: i32, max: i32) -> i32 {
        let mut ret = 0;
        if val < min || val > max {
            ret = 0;
        } else {
            ret = 1;
        }
        ret
    }
    pub fn calculate_power(base: i32, exp: i32) -> i32 {
        if exp > 1 {
            return base * calculate_power(base, exp - 1);
        } else {
            return base;
        }
    }
    pub fn get_num(mut input: &mut String) -> i32 {
        println!("Enter an integer number");
        use std::io;
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => 0,
        };
        input
    }
}
