// ## Exercise 1 — FizzBuzz With a Twist 🎯

// The classic. But in Rust — using a `for` loop with a range.

// Write `exercise_1()` that loops from `1` to `30` (inclusive) and:
// - If the number is divisible by 3 AND 5 → print `FizzBuzz`
// - If divisible by 3 only → print `Fizz`
// - If divisible by 5 only → print `Buzz`
// - Otherwise → print the number

// **Hint:** The modulo operator is `%`. `n % 3 == 0` means divisible by 3.

fn exercise_1(){

for i in (1..31) {
    if i%3==0 && i%5==0 {
        println!("FizzBuzz");
    }
    else if i%3==0 {
        println!("Fizz");

    }

    else if i&5==0 {
        println!("Buzz");
    }
    else {
    println!("The value of i :{}", i);

    }
}

}


fn main(){
    exercise_1();
}