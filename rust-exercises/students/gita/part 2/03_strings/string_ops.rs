fn main() {
//  CREATING STRINGS
let mut s=String::new(); //Just an empty string
let s=String::from("Initialized content");
let s="initial content".to_string(); //same as above
appending_string();
concatenating_string();

}


fn appending_string() {
    // appending to a string
    let mut s=String::from("fooo");
    s.push_str("bar"); //append a string slice
    s.push('!'); //append a single character
    println!("{}", s);

}


fn concatenating_string() {
    let s1=String::from("Hello, ");
    let s2=String::from("World!");
    let s3=s1+&s2;
    println!("{}", s3);
}