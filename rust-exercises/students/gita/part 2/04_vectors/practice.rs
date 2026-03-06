fn main() {
    // creating vectors
    let v: Vec<i32> = Vec::new(); //Empty vector, must specify type
    let v = vec![1, 2, 3, 4]; //vec! macro infers type from content
    adding_elements();
    accessing_elements();
    iterating_over_vectors();
}

fn adding_elements() {
    let mut v = Vec::new();
    v.push(5); //compiler infers v is Vec<i32> from this
    v.push(6);
    v.push(7);
    v.push(8);
    println!("{:?}", v);
}

fn accessing_elements() {
    let v = vec![1, 2, 3, 4, 5];
    // using indexing:
    let third = v[2];
    println!("The third element is {}", third);
    // method 2: using get (returns Option, safer)
    match v.get(2) {
        Some(answer) => println!("The answer is : {}", answer),
        None => println!("There is no third element"),
    }
}

fn mutable_borrow() {
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0]; //Immutable borrow of the vector
    //v.push(6); //Error, can mutably borrow while immutably borrowed
    println!("The first element is: {}", first);
    // now first is no longer in use, so we can modify it
    v.push(6);
    println!("{:?}", v);
}

// iteratting over vectors
fn iterating_over_vectors() {
    let v = vec![30, 32, 100];
    // immutable iteration
    for i in &v {
        println!("{}", i);
    }

    // Mutable iteration
    let mut v = vec![20, 34, 34, 56];
    // explain dereferencing:the * operator is used to dereference a mutable reference, allowing us to modify the value it points to. In this case, *i gives us access to the actual integer value in the vector, which we can then modify by adding 50 to it.
    for i in &mut v {
        *i += 50;
    }
    println!("{:?}", v); //

    // Taking ownership, vectors become unusable after
    for element in v {
        println!("{}", element);
    }

    //println!("{:?}", v); // Error! v has been moved into the for loop
}
