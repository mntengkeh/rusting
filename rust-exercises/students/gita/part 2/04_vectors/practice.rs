fn main() {
    // creating vectors
    let v:Vec<i32>=Vec::new(); //Empty vector, must specify type
    let v=vec![1, 2, 3, 4]; //vec! macro infers type from content
    adding_elements();


}

fn adding_elements() {
    let mut v=Vec::new();
    v.push(5); //compiler infers v is Vec<i32> from this
    v.push(6);
    v.push(7);
    v.push(8);
    println!("{:?}", v);
}