use std::io;

fn main() {
    loop {
        let unit = get_unit();
        let temperature = get_temperature();
        let result = cal_temperature(&unit, &temperature);
        println!("Converted temperature: {}", result);
    }
}

fn get_unit() -> String {
    let mut unit = String::new();
    println!("Choose the unit (celsius/fahrenheit):");
    io::stdin()
        .read_line(&mut unit)
        .expect("Failed to read line");
    unit.trim().to_lowercase() // Convert input to lowercase for comparison
}

fn get_temperature() -> f64 {
    let mut temperature = String::new();
    println!("Enter the temperature:");
    io::stdin()
        .read_line(&mut temperature)
        .expect("Failed to read line");
    match temperature.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid temperature input. Please try again.");
            get_temperature() // Recursively call until valid input is provided
        }
    }
}

fn cal_temperature(unit: &str, temperature: &f64) -> f64 {
    if unit == "celsius" {
        temperature * 1.8 + 32.0
    } else if unit == "fahrenheit" {
        (temperature - 32.0) * 5.0 / 9.0
    } else {
        println!("Invalid unit. Assuming Celsius.");
        temperature * 1.8 + 32.0
    }
}
