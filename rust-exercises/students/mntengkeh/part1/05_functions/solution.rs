// ============================================
// Student: mntengkeh
// Topic: Functions (Part 1, Day 7)
// Date: 2026-02-25
// ============================================

// Exercise 1
fn exercise_1() {
    println!("{}", is_even(9));
		println!("{}", classify_temperature(20.0));
		println!("BMI: {:.4}", calculate_bmi(60.0, 1.8));
		println!("BMI description: {}", describe_bmi(15.5));
}

fn is_even(n: i32) -> bool {
	//rust returns the value of the last expression in any function
	//absence of a trailing ';' makes the line below an expression, not a statement
	n & 1 == 0
}

fn classify_temperature(temp: f64) -> &'static str {
  let status =	if temp < 0.0 {"freezing"} else if temp < 15.0 {"cold"} else if temp < 25.0 {"warm"} else {"hot"};
	status
}

fn calculate_bmi(weight_kg: f64, height_m: f64) -> f64 {
	weight_kg / (height_m * height_m)
}

fn describe_bmi(bmi: f64) -> &'static str {
	let description = if bmi < 18.5 {"Underweight"} else if bmi < 25.0 {"Normal"} else if bmi < 30.0 {"Overweight"} else {"Obese"};
	description
}

// Exercise 2
fn exercise_2() {
	for i in 1..11{
    println!("{}! = {}", i, factorial(i));
	}
	println!("\n");
	for i in 1..11 {
		println!("fib({}) = {}", i, fibonacci(i));
	}
}

fn factorial(n: u64) -> u64 {
	if n == 0 {
		1
	} else {
		n * factorial(n-1)
	}
}

fn fibonacci(n: u32) -> u64 {
	if n == 0 {
		0
	}else if n == 1 {
		1
	} else {
		fibonacci(n-1) + fibonacci(n-2)
	}
}

// Exercise 3
fn exercise_3() {
	//convert 100 degree C to fahrenheit
  let celcius_to_fahrenheit = {
		let temp_c = 100.0;
		(temp_c * 9.0 / 5.0) + 32.0
	};
	println!("100\u{00B0}C = {}\u{00B0}F", celcius_to_fahrenheit);
	
	let string_stats = {
		let sample = "Hello, Rustaceans!";
		(sample.len(), sample.to_uppercase())
	};
	println!("{:#?}", string_stats);

	let collatz_steps = {
		let mut n = 27;
		let mut count = 0;
		loop {
			if is_even(n) {
				n /= 2;
			} else {
				n = 3 * n + 1;
			}
			//println!("{}", n);
			count += 1;
			if n == 1 {
				break count;
			}
		}
	};
	println!("It takes {} steps to get from 27 to 1", collatz_steps);
}

fn main() {
    //exercise_1();
    //exercise_2();
    exercise_3();
}
