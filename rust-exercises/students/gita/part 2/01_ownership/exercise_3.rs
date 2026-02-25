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

fn exercise_3() {
    // part a
    let name=String::from("Gita");
    call_my_name(name.clone());
    let name1=String::from("Gita");
    let no_ownership=no_ownership_needed(name1);
    let (my_name, length)=no_ownership;
    println!("My name is {} and the length of my name is {}", my_name, length);
    println!("My name is {:?}", name);
}

// dewfine the call_my_name function

fn call_my_name(name:String) {
println!("FROM FUNCTION My name is {}", name);
}

fn main () {
    exercise_3();
}