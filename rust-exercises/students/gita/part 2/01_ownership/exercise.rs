// ---

// ## Exercise 1 — Move Detective 🕵️

// Read each code snippet below. For each one, write in a comment:
// - Does it compile? Yes or No
// - If no, WHY does it fail?
// - Fix it WITHOUT using references (use cloning or restructuring)

// If we want to deeply copy the heap data of the `String`, not just the stack data, we can use a common method called `clone`.

// ```rust
// let s1 = String::from("hello");
// let s2 = s1.clone();

// println!("s1 = {}, s2 = {}", s1, s2);
// ```



// Snippet A
// NO: it doesn't compile
// because we are trying to print s1, which is not still in the memeory as ownership has already been taken

fn snippet_a() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("{}", s1);
    println!("{}", s2);

}

// Snippet B
// SAME REASON
fn snippet_b() {
    let s = String::from("world");
    takes_string(s.clone());
    println!("{}", s);
}
fn takes_string(s: String) {
    println!("{}", s);
}

// // Snippet C
// fn snippet_c() {
//     let x = 42;
//     let y = x;
//     println!("{}", x); // Does this compile?
// }
// // ```

// Explain in comments why Snippet C behaves differently from A and B.

// ---

fn main() {
    // snippet_a();
    snippet_b();
}

// ## Exercise 2 — Ownership Chain 🔗

// Write a function chain that passes ownership between functions:

// 1. `create_greeting() -> String`
//    Creates and returns the String `"Hello from Rust!"`
fn create_greeting() -> String {
    let greeting = String::from("Hello from Rust!");
    greeting
}

// 2. `shout_it(s: String) -> String`
fn shout_it(s: String) -> String {
    s.to_uppercase()
}
g//    Takes ownership, converts to uppercase using `.to_uppercase()`,
//    returns the new owned String

// 3. `measure_it(s: String) -> (String, usize)`
fn measure_it(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}   
//    Takes ownership, returns a tuple of the String AND its length

// 4. In `exercise_2()`:
fn exercise_2() {
    let greeting = create_greeting(); // ownership of the string is created and stored in greeting
    let shouted = shout_it(greeting); // ownership of the string is moved to shouted, greeting is no longer valid
    let (final_string, length) = measure_it(shouted); // ownership of the string is moved to final_string, shouted is no longer valid, length is stored in length variable
    println!("Final string: {}, Length: {}", final_string, length); // final_string is
}
//    - Call `create_greeting()` → store in variable
//    - Call `shout_it()` with it → store in new variable
//    - Call `measure_it()` with it → destructure the tuple
//    - Print the final string and its length

// Trace in comments where ownership lives at each step.
