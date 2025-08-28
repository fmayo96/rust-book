use std::io;

fn main() {
    println!("===Convert °F to °C===");
    println!("Enter temperature in °F:");
    let temp_f = read_temp();
    let temp_c = farenheit_to_celsius(temp_f);
    println!("{}°C", temp_c);
}

fn farenheit_to_celsius(temp_f: f32) -> f32 {
    (temp_f - 32.0) * 5.0 / 9.0
}

fn read_temp() -> f32 {
    loop {
        let mut temp_f = String::new();
        io::stdin()
            .read_line(&mut temp_f)
            .expect("Error reading input.");
        match temp_f.trim().parse::<f32>() {
            Ok(num) => return num,
            Err(_) => println!("Invalid temperature, please try again."),
        }
    }
}
