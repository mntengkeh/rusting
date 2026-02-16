// ============================================
// Student: mntengkeh
// Topic: Data types (Part 1, Day 2)
// Date: 2026-02-16
// ============================================


// Exercise 1
fn exercise_1() {
	// u8: u because 255 > 0 and 8 because 255 can be represented fully with a minimum of 8 bits
    let var1: u8 = 255;
		println!("{} is a u8", var1);

  // i16: i because -32000 < 0 and 16 because 32000 can be fully represented with a minimum of 16 bits
		let var2: i16 = -32_000;
		println!("{} is an i16", var2);

	// f64: pi can be fully represented with 32 bits, but f64 is required for higher precision
		let pi: f64 = 3.14159265358979;
		println!("{} is an f64", pi);
	
	// char: crab is a single unicode 4 byte character
		let crab: char = '🦀';
		println!("{} is a char", crab);

	// bool: boolean represents a value which is either true or false
		let boolean: bool = true;
		println!("{} is a bool", boolean);

	// u32: u because var6 is positive and 32 because 1,000,000,000 can be fully represented with a minimum of 32 bits
		let var6: u32 = 1_000_000_000;
		println!("{} is a u32", var6);
}


// Exercise 2
fn exercise_2() {
		let passport: (&str, u8, [char; 2], bool) = ("Jane Doe", 28, ['C', 'M'], true);
		let (name, age, country_code, validity) = passport;
		println!("Name: {}", name);
		println!("Age: {}", age);
		println!("Country code: {}{}", country_code[0], country_code[1]);
		println!("Valid: {}", validity);
		println!("Age via dot notation: {}", passport.1);
}

// Exercise 3
fn exercise_3() {
    let var1: u8 = 250;
		println!("u8 value: {}", var1);

	  let var2: u16 = (var1 as u16) + 1000;
		println!("u16 value after addition: {}", var2);

		let var3: i32 = 2_000_000_000;
		let _var4: f64 = var3 as f64;
		
		let op1: u8 = 7;
		let op2: u8 = 2;

		println!("Integer division 7/2 = {}", op1/op2);
		println!("Float division 7.0/2.0 = {}", (op1 as f32)/(op2 as f32));

}

fn main() {
    //exercise_1();
    //exercise_2();
    exercise_3();
}
