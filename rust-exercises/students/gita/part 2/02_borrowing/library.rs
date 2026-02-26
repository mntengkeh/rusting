struct Library {
    books: Vec<String>,
}

// Mutable borrow - we modify the ibrary
// String->owned heap data...
fn add_book(library: &mut Library, title: String) {
    library.books.push(title);
}

// Immutable borrow - we just read the library
fn display_catalog(library: &Library) {
    println!("Libary Catalog:");
    for book in &library.books {
        println!("- {}", book);
    }
}

// IMMUTABLE BORROW -SEARCHING ONLY READ
fn search_book(library: &Library, title:&str)->bool {
    //    library.books.iter().any(|book| book == title)
    for book in &library.books {
        if book==title {
            return true;
        }
        
    }
    false
}


// Immutable borrow - just counting
fn count_books(library: &Library)->usize {
    library.books.len()
}


fn exercise_3() {
    // create library (it owns the vector)
    let mut library=Library {
        books:Vec::new(),
    };

    // add books (mutable borrow)
    add_book(&mut library, String::from("The Rust Programming Language"));
    add_book(&mut library, String::from("Programming Rust"));   
    add_book(&mut library, String::from("Rust in Action"));
    add_book(&mut library, String::from("The Rust Book"));
    // display catalog (immutable borrow)
    display_catalog(&library);

    // we can call immutable functions multiple times without issue
    println!("Total books: {}", count_books(&library));
    println!("Is 'The Rust Book' in the library? {}", search_book(&library, "The Rust Book"));
    println!("Is 'The Rust Book' in the library? {}", search_book(&library, "The Rust Book"));  


}

fn main(){
    exercise_3();
}