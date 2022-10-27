use std::io;
use std::io::Write;

fn main() {

    const CRITICAL_VALUES: [f64; 9] = [3.841, 5.991, 7.815, 9.488, 11.070, 12.592, 14.067, 15.507, 16.919];

    println!("This program calculates if the null hypothesis is");
    println!("accpted or rejected for a critical value of 0.05.");
    
    
    let degrees_of_freedom = possibility_input();
    println!("{degrees_of_freedom}");

}

// Checks whether the compared value is between minimum and maximum
fn check_valid_input(min: i8, max: i8, compared: i8) -> bool {
    if compared > max || compared <= min {
        println!("Invalid input!");
        return false;
    }
    else {
        return true;
    }
}

// Gets input from user and finds degree of freedom
fn possibility_input() -> i8 {
    loop {
        print!("Please input the number of possibilities: ");
        io::stdout().flush().unwrap(); // Flush stdout immediately
        let mut degrees_of_freedom = String::new();

        io::stdin()
            .read_line(&mut degrees_of_freedom)
            .expect("Invalid input!");

        let degrees_of_freedom: i8 = match degrees_of_freedom.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if check_valid_input(0, 9, degrees_of_freedom) == true {
        return degrees_of_freedom - 1;
        }
        else {
            continue;
        }
    }
}
