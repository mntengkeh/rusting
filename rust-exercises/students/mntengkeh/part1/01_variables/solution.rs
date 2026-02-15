// ============================================
// Student: your_github_username
// Topic: Variables (Part 1, Day 1)
// Date: YYYY-MM-DD
// ============================================

// Exercise 1
fn exercise_1() {
	// x was immutable but its value was changed
    let mut x = 10;
    x = 20;
    println!("x is {}", x);

	// greeting was immutable but its value was changed
    let mut greeting = "hello";
    greeting = "goodbye";
    println!("{}", greeting);

	// max_score was in lower case, and it had no explicit type declaration
    const MAX_SCORE: u8 = 100;
    println!("Max score is {}", MAX_SCORE);
}

// Exercise 2
fn exercise_2() {
    let value = 2;
		let value = value * 10;
		let value = value + 5;
		let value = value.to_string();
		let value = format!("{}{}", value, " is the answer");

		println!("{}", value);
}

// Exercise 3
fn exercise_3() {
    let ABSOLUTE_ZERO: f64 = -273.15;
		let degree_celcius = "\u{00B0}C";
		let mut temperature = 100.0;
		println!("Starting temperature: {}{}", temperature, degree_celcius);
		let temperature = -10.5;
		println!("Current temperature: {}{}", temperature, degree_celcius);
		println!("Absolute zero: {:.2}{}", ABSOLUTE_ZERO, degree_celcius);
		println!("Difference from absolute zero: {:.2}{}", temperature - ABSOLUTE_ZERO, degree_celcius);
}

fn main() {
    // exercise_1();
    // exercise_2();
    exercise_3();
}
