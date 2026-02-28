// ============================================
// Student: mntengkeh
// Topic: Strings (Part 1, Day 10)
// Date: 2026-02-28
// ============================================

// Exercise 1
fn exercise_1() {
	let s: &str = "hello";
	let s1: String = String::from(s);
	let s2: String = s.to_string();
	let s3: &str = &s2[..];

	println!("{}", first_word("hello world"));
	println!("{}", first_word("rust"));
	println!("{}", first_word("  leading space"));

	accepts_both(s);
	accepts_both(&s1);
}

fn first_word(s: &str) -> &str {
	s.trim().split(' ').next().unwrap()
}

fn accepts_both(s: &str) {
	println!("{}", s);
}

// Exercise 2
fn exercise_2() {
    // Your solution here
}

// Exercise 3
fn exercise_3() {
    // Your solution here
}

fn main() {
    exercise_1();
    exercise_2();
    exercise_3();
}
