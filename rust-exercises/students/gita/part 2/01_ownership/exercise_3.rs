// ## Exercise 3 — Clone vs Move 🧬

// Write `exercise_3()` demonstrating when clone is appropriate
// and when it's wasteful.

// **Part A:** Show a case where `.clone()` is necessary:
// Create a String, pass it to a function, but also print it
// after the function call. Use clone so you can do both.



// **Part B:** Show a case where clone is wasteful:
// You have a String and you want to find its length. Instead
// of cloning to pass it to a function, redesign the function
// to not need ownership (for now, just pass the String in and
// return it back as a tuple — no references yet).

fn no_ownership_needed(s:String)->(String, usize) {
    let my_length=s.len();
    (s, my_length)
}

// **Part C:** Show what happens with Copy types:
// Demonstrate that `i32`, `f64`, `bool`, and `char` all
// automatically copy when assigned, so you never need `.clone()`.

fn never_needing_clone(){
    let x=42;
    let y=x;
    println!("x is {} and y is {}", x, y);
    let a=3.14;
    let b=a;
    println!("a is {} and b is {}", a, b);
    let c=true;
    let d=c;
    println!("c is {} and d is {}", c, d);
    let e='R';
    let f=e;
    println!("e is {} and f is {}", e, f);
}

// For each part, print output that proves your code works correctly
// and add comments explaining your ownership decisions.


fn exercise_3() {
    // part a
    let name=String::from("Gita");
    call_my_name(name.clone());
    let name1=String::from("Gita");
    let no_ownership=no_ownership_needed(name1);
    let (my_name, length)=no_ownership;
    println!("My name is {} and the length of my name is {}", my_name, length);
    println!("My name is {:?}", name);
    never_needing_clone();
}

// dewfine the call_my_name function

fn call_my_name(name:String) {
println!("FROM FUNCTION My name is {}", name);
}

fn main () {
    exercise_3();
}