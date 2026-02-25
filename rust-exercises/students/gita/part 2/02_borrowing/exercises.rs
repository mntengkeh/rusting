// // Borrowing helps us use a value without takinng ownership
// fn main()  {
//     let s1=String::from("hello");
//     let len=calculate_len(&s1);
//     println!("The length of '{}' is {}", s1, len);
// }

// fn calculate_len(s:&String)->usize {
//     s.len()
// }

// // you can change data you borrow if you use mutable reference
// fn main() {
//     let mut s1=String::from("hello");
//     change(&mut s1);
//     println!("The changed string is: {}", s1)
// }


// fn change(s:&mut String){
//     s.push_str(", world");
// }


// exercise 1 — Fix the Borrow Checker 🔧
// The following functions all have borrow checker violations. For each one: copy it, add a comment explaining the violation, then fix it using references.


// Function A: shouldn't need to take ownership
fn get_length(s: String) -> usize {
    s.len()
}

// Function B: has an illegal double mutable borrow
// This rule prevents data races at compile time. No runtime drama. No segfaults. No undefined chaos.
// fn double_mut_borrow() {
//     // cannot borrow `s` as mutable more than once at a time
//     let mut s = String::from("hello");
//     let r1 = &mut s;
//     let r2 = &mut s;
//     println!("{} and {}", r1, r2);
// }

// FIX: use scoped referencing
// limit the lifetime of the first time mutable borrow
fn double_borrow_fixed() {
    let mut s=String::from("hello");
    {
        let r1=&mut s;
        println!("r1 is {} and has gone out of scop", r1);
        
    } //r1 goes out of scope here, so we can make a new mutable reference
    let r2=&mut s;
    println!("r2 is {}", r2);
}

fn main() {

    double_borrow_fixed();
}