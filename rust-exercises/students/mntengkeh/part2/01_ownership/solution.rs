// ============================================
// Student: mntengkeh
// Topic: Variables (Part 1, Day 8)
// Date: 2026-02-26
// ============================================

// Exercise 1
fn exercise_1() {
    snippet_a();
    snippet_b();
    snippet_c();
}

// Snippet A
// snippet_a won't compile because s1 is used after ownership of its value has been moved into s2, making s1 invalid after the assignment to s2

fn snippet_a() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("{}", s1);
}

// Snippet B
// snippet_b won't compile because s is used after ownership of its value has been moved into the function takes_string, making s invalid after the function call
fn snippet_b() {
    let s = String::from("world");
    takes_string(s.clone());
    println!("{}", s);
}
fn takes_string(s: String) {
    println!("{}", s);
}

// Snippet C
//snippet_c compiles because x holds an i32 value, which implements the Copy trait. Assigning x to y copies the value stored in x into y creating two identical values on the stack
fn snippet_c() {
    let x = 42;
    let y = x;
    println!("{}", x); // Does this compile?
}

// Exercise 2
fn exercise_2() {
    //ownership lives in greeting, moved from the create_greeting function
    let greeting = create_greeting();
    //ownership is moved from greeting into the shout_it function and returned to greeting from shout_it
    let greeting = shout_it(greeting);
    //ownership is moved from greeting above into the measure_it function
    //ownership is then moved back through the return value of the measure_it function
    //ownership is now in the greeting variable in the destructuring tuple
    let (greeting, length) = measure_it(greeting);
    println!("Greeting: {}, Length: {}", greeting, length)
}

fn create_greeting() -> String {
    String::from("Hello from Rust!")
}

fn shout_it(s: String) -> String {
    s.to_uppercase()
}

fn measure_it(s: String) -> (String, usize) {
    let size = s.len();
    (s, size)
}

// Exercise 3
fn exercise_3() {
    // i'm leveraging some of the functions defined above
    // Part A
    //ownership of this string belongs to my_string
    let my_string = String::from("Some random string");
    //after the function call below, ownership still belongs to my_string because we passed a clone, not the original variable
    takes_string(my_string.clone());
    //successfull print proves that ownership was never moved
    println!("Part A\n{}", my_string);

    // Part B
    //ownership belongs to my_string1
    let my_string1 = String::from("Some other random string");
    //ownership is moved into the measure_it function
    //ownership is then moved back out through the return the original string in the tuple
    let (original, size) = measure_it(my_string1);
    //successful display of original proves that ownership has been moved back out
    println!("Part B\nMy string: {}, Length: {}", original, size);

    let var = 45;
    let var0 = 0.0;
    let var1 = false;
    let var2 = 'a';

    let a_var = var;
    let a_var0 = var0;
    let a_var1 = var1;
    let a_var2 = var2;

    //successfull print here shoes that the values were never moved, but rather copied
    println!("Part C\n{} : {} : {} : {}", var, var0, var1, var2);

}

fn main() {
    //xercise_1();
    //exercise_2();
    exercise_3();
}
