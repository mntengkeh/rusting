// ## Exercise 2 — Tuple Passport 🛂

// Model a passport using a tuple. A passport contains:
// - Full name (use a string literal `&str` for now)
// - Age (u8 — a person can't be older than 255)
// - Country code (a 2-char array: `[char; 2]`)
// - Is valid (bool)

// Write a function `exercise_2()` that:
// 1. Creates the passport tuple
// 2. Destructures it into four separate variables
// 3. Prints each field with a label
// 4. Access the age a second time using **dot notation** (not
//    the destructured variable) and print it

// **Expected output:**
// ```
// Name: Jane Doe
// Age: 28
// Country: CM
// Valid: true
// Age via dot notation: 28
// ```

fn exercise_2(){
    // create a turple here
    let password_tuple:(&str, u8, [char; 2], bool)=("john",33,['w','r'], false);
    let (name, age, char_2,is_valid)=password_tuple;

    println!("The values of my tuples are :{:?}", password_tuple);
    println!("NAME: {}", name);
    println!("AGE: {}", age);
    println!("CHAR 1: {}", char_2[0]);
    println!("ISVALID: {}", is_valid);
    println!("DOT..{}", password_tuple.0);

}



// Write a function exercise_3() that:

// Declares a u8 variable with value 250
// Prints it
// Declares a u16 variable that takes the u8 value and adds 1000 to it — you'll need to cast: value as u16
// Prints the u16 result
// Declares an i32, an f64, and converts the i32 to f64 using as f64. Show that 7 / 2 in integers equals 3, but 7.0 / 2.0 in floats equals 3.5
// Print both results with labels


fn exercise_3(){
    // WRITE EXERCISE 3 HERE
    let value1:u8=250;
   
    println!("VALUE 1_ : {}", value1);
    let value2:u16=value1 as u16;
    println!("VALUE_2: {}", value2);
    let value3:i32=7/3;
    println!("VALUE 3: {}", value3);
    let value4:f64=7.0/2.0;
    println!("VALUE 4: {}", value4);
    let value3:f64=value3 as f64;
    println!("VALUE 3: {}", value3);


}

fn main(){
    exercise_3();
}