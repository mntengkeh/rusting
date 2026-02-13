// ## Exercise 3 — The Temperature Vault 🌡️

// Write a function `exercise_3()` that:

// 1. Declares a constant `ABSOLUTE_ZERO_C: f64` equal to `-273.15`
// 2. Declares a mutable variable `temperature` starting at `100.0` (f64)
// 3. Prints the starting temperature
// 4. Changes temperature to `-10.5`
// 5. Prints the new temperature
// 6. Prints absolute zero from the constant
// 7. Calculates and prints the difference between temperature and
//    absolute zero (temperature - ABSOLUTE_ZERO_C)

fn exercise_3() {
    const ABSOLUTE_ZERO_C: f64 = -273.15;
    let mut temperature: f64 = 100.0;
    println!("STARTING TEMPERATURE: {}", temperature);
    temperature = -10.5;
    println!("The new temperature is: {}", temperature);
    println!("The absolute temperature: {}", ABSOLUTE_ZERO_C);
    let absolute_zero = temperature - ABSOLUTE_ZERO_C;
    println!("printing ABSOLUTE TEMPERATURE: {}", absolute_zero);
}

fn main() {
    exercise_3();
}
