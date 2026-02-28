// ============================================
// Student: mntengkeh
// Topic: Borrowig (Part 1, Day 9)
// Date: 2026-02-27
// ============================================

// Exercise 1
fn exercise_1() {
	let s = String::from("some string");
  let s = get_length_fix(&s);
	println!("{}", s);
	double_mut_borrow_fix();
	mixed_borrows();
}

//original get_length
// the problem here is that get_length accepts a String instance, whose size is unknown at compile time, and hence making the passed string unusable after a call to get_length
// Function A: shouldn't need to take ownership
//fn get_length(s: String) -> usize {
//  s.len()
//}

//fixed get_length
//this alternative accepts a referece to the string, thereby borrowing it for use in this function without taking ownership
fn get_length_fix(s: &String) -> usize {
	s.len()
}

// Function B: has an illegal double mutable borrow
// the issue here is due to the fact that rust allows only one mutable reference to any value, which prevents data races
//fn double_mut_borrow() {
  //  let mut s = String::from("hello");
    //let r1 = &mut s;
    //let r2 = &mut s;
    //println!("{} and {}", r1, r2);
//}

// fixed double_mut_borrow
// the value of the first ref should be printed before the intruduction of the second, leveraging reference scope
fn double_mut_borrow_fix(){
	let mut s = String::from("hello");
	let r1 = &mut s;
	print!("{} and ", r1);
	let r2 = &mut s;
	println!("{}", r2);
}

// Function C: mutable + immutable borrow overlap
// rust doesn't allow mutable and immutable references to the same value. 
fn mixed_borrows() {
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
		print!("{}, {}, and ", r1, r2);
    let r3 = &mut s;
    println!("{}", r3);
}

// Exercise 2
fn exercise_2() {
  let mut library: Library = Library {
    books: Vec::new()
  };

  let books = [String::from("time travel"), String::from("solo leveling"), String::from("superintelligence"), String::from("hunter"), String::from("witcher")];
  
  for book in books.iter() {
    add_book(&mut library, book.clone());
  }

  display_catalog(&library);

  let found = find_book(&library, "hunter");
  println!("Found book hunter: {}", found);
  let found = find_book(&library, "farmer");
  println!("Found book farmer: {}", found); 

  println!("Book count: {}", count_books(&library));

  // display_catalog(&library);
  // display_catalog(&library);

  // let found = find_book(&library, "some book");
  // println!("Found book : {}", found);
  
}

struct Library {
    books: Vec<String>,
}

fn add_book(library: &mut Library, title: String) {
  library.books.push(title.trim().to_string());
}

fn display_catalog(library: &Library){
  println!("Library Catalog: \n{:#?}", library.books);

}

fn find_book(library: &Library, title: &str) -> bool {
  for book in library.books.iter() {
    if book.to_lowercase() == title.to_lowercase() {
      return true;
    }
  }
  false
}

fn count_books(library: &Library) -> usize {
  library.books.len()
}

// Exercise 3
fn exercise_3() {
    let mut s = String::from("some string");
    let r1 = &s;
    let r2 = &s;
    println!("From immutable refs: {}, {}", r1, r2);

    let m1 = &mut s;
    m1.push_str(" xtra");
    println!("From mutable ref: {}", m1);

    let r3 = &s;
    println!("From new immutable ref: {}", r3);

}



fn main() {
    //exercise_1();
    //exercise_2();
    exercise_3();
}
