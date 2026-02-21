// # ✏️ Functions — Exercises

// Write your solutions in:
// `students/your_username/part1/05_functions/solutions.rs`

// ---

// ## Exercise 1 — Expression vs Statement 🤔

// This exercise is about deeply understanding how Rust functions return
// values through expressions.

// Write these four functions AND explain with a comment what each
// one returns and why:



// **Function 1:** `is_even(n: i32) -> bool`
fn is_even(n:i32)->bool {
    if n%2==0 {
     return true;
    }

    return false;
    
}
// Returns true if n is even using an expression (no `return` keyword)

// **Function 2:** `classify_temperature(temp: f64) -> &'static str`
fn classify_temperature(temp:f64)->& 'static str {
    if temp<0.0 {
        "freezing"
    }
    else if temp <15.0 {
        "cold"
    }
    else if temp<25.0 {
        "hot"
    }
    else{
        "warning"
    }
 
}


fn calculate_bmi(weight_kg:f64, height_m:f64)->f64 {
    weight_kg/(height_m * weight_kg)

}

fn describe_bmi(bmi:f64)->& 'static str {

    if bmi<18.5 {
        "underweight"
    }
    else if bmi>=18.5 && bmi<=25.0 {
        "Normal"
    }
    else if bmi >=25.0 && bmi<=30.0 {
        "overweight"
    }
    else if bmi>30.0 {
        "Obese"
    }
    else {
        "Invalid BMI value"
    }

}

fn exercise_1(){
    is_even(23);
    let temp_classify=classify_temperature(20.7);
    println!("TEMPERATURE CLASS: {}", temp_classify);
    let bmi=calculate_bmi(45.4, 2.4);
    println!("BMI PRINTED IS: {}", bmi);
    let bmi_description=describe_bmi(bmi);
    println!("DESCRIBE BMI: {}", bmi_description);


}

fn main() {
    exercise_1();
    exercise_2();
}



// ## Exercise 2 — Recursive Thinking 🌀

// Write `exercise_2()` that calls two recursive functions:

fn exercise_2() {
    for i in 0..=10 {
        println!("{}! = {}", i, factorial(i));
    }
    for i in 0..=10 {
        println!("fib({}) = {}", i, fibonacci(i));
    }
}

// **Function:** `factorial(n: u64) -> u64`
fn factorial(n:u64)->u64 {
    if n==0 {
        1
    }
    else{
        n*factorial(n-1)
    }
    
}
// Computes n! recursively.
// - Base case: `factorial(0) = 1`
// - Recursive case: `n * factorial(n - 1)`

// **Function:** `fibonacci(n: u32) -> u64`
fn fibonacci(n:u32)->u64 {
    if n==0 {
        0
    }
    else if n==1 {
        1
    }
    else {
        fibonacci(n-1)+fibonacci(n-2)
    }
    
}
// Computes the nth Fibonacci number.
// - Base cases: `fibonacci(0) = 0`, `fibonacci(1) = 1`
// - Recursive case: `fibonacci(n-1) + fibonacci(n-2)`

// In `exercise_2()`, print:
// - Factorials of 0 through 10
// - Fibonacci numbers 0 through 10

// **Expected output (partial):**
// ```
// 0! = 1
// 1! = 1
// 2! = 2
// ...
// 10! = 3628800
// fib(0) = 0
// fib(1) = 1
// ...
// fib(10) = 55
// ```

// ---
