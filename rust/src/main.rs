use std::io;
use std::io::Write;

const CRITICAL_VALUES: [f64; 9] = [3.841, 5.991, 7.815, 9.488, 11.070, 12.592, 14.067, 15.507, 16.919];

fn main() {
    println!("This program calculates if the null hypothesis is");
    println!("accepted or rejected for a critical value of 0.05.");
    println!();
    
    let possibilities = possibility_input();

    let expected_value = get_float_input("Expected value: ");

    let chi_sum = get_chi_sum(possibilities, expected_value);

    println!();
    println!("The expected value is {expected_value}");
    println!("The possibilities is {possibilities}");
    println!("Chi sum: {chi_sum}");
    if chi_sum > get_critical_value(possibilities.try_into().unwrap()) {
        println!("Null hypothesis is rejected.")
    }
    else {
        println!("Null hypothesis is accepted.")
    }
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
        let mut possibilities = String::new();

        let result = io::stdin()
            .read_line(&mut possibilities);
        if let Err(_) = result {
            continue
        }

        let possibilities: i8 = match possibilities.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if check_valid_input(0, 9, possibilities) {
            break possibilities;
        }
    }
}

fn get_float_input(message: &str) -> f64 {
    loop {
        print!("{message}");
        io::stdout().flush().unwrap();

        let mut expected_value = String::new();
        let result = io::stdin()
            .read_line(&mut expected_value);
        if let Err(_) = result {
            continue
        }

        let expected_value: f64 = match expected_value.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        return expected_value;
    }

}

fn chi_square_calculation(expected: f64, observed: f64) -> f64 {
    let mut chi_value = observed - expected;
    chi_value *= chi_value;
    chi_value = chi_value / expected;

    return chi_value;
}

fn get_critical_value(critical_value: usize) -> f64 {
    return CRITICAL_VALUES[critical_value];
}

fn get_chi_sum(possibilities: i8, expected_value: f64) -> f64 {
    let mut chi_sum: f64 = 0.0;
    for _ in 0..possibilities {
        let observed_value = get_float_input("Observed value: ");
        let chi_value = chi_square_calculation(expected_value, observed_value);
        chi_sum = chi_sum + chi_value;
    }
    return chi_sum;
}
