fn exercise_3(){
    // create a muatble string
    let mut text=String::from("Hello");
    println!("Initial text: {}", text);

    // create 2 immutable references
    let r1=&text;
    let r2=&text;
    println!("Immutable references: r1:{}, r2:{}", r1, r2); //after this line, r1 and r2 are no longer used
    // their borrow ends above,not at the end of the block  so we can create a mutable reference now

    // create a mutable reference here
    let r3=&mut text;
    r3.push_str(", world!");
    println!("Mutable reference r3: {}", r3);
    // mutable borrow ends here, we can create immutable references again
    let r4=&text;
    let r5=&text;
    println!("Immutable references after mutable borrow: r4:{}, r5:{}", r4, r5);


}

fn main() {
    exercise_3();
}