// ============================================
// Student: mntengkeh
// Topic: Conditionals (Part 1, Day 6)
// Date: 2026-02-24
// ============================================

// Exercise 1
fn exercise_1() {
    let score: u32 = 45;
		if score >= 90 {
			println!("{}: A - Excellent!", score);
		}else if score >= 80 {
			println!("{}: B - Good", score);
		}else if score >= 70 {
			println!("{}: C - Average", score);
		}else if score >= 60 {
			println!("{}: D - Below Average", score);
		}else{
			println!("{}: F - Failing", score);
		}
}

// Exercise 2
fn exercise_2() {
    let n = 17;
		let state = if n & 1 == 0 {"even"} else {"odd"};
		println!("{} is {}", n, state);
		let size = if n < 10 {"small"} else if n < 99 {"medium"} else {"large"};
		println!("{} is {}", n, size);
		let n = -42;
		let abs_n = if n < 0 {-n} else {n};
		println!("Absolute value of {} is {}", n, abs_n);
}

// Exercise 3
fn exercise_3() {
    let mut light: &str = "red";
		let mut action = "";
		let mut duration = "";
		let mut cross = "No";

		if light == "red" {
			action = "Stop";
			duration = "30";
		}else if light == "yellow" {
			action = "Caution";
			duration = "5";
		} else if light == "green" {
			action = "Go";
			duration = "25";
			cross = "Yes";
		}else {
			light = "invalid";
		}

		println!("{} light: {} | Duration: {}s | Safe to cross : {}", light, action, duration, cross);

}

fn main() {
    //exercise_1();
    //exercise_2();
    exercise_3();
}
