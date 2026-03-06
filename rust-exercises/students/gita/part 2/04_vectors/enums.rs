// use enums to store element in our vectors of different types

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String)
}

fn exploring_enums() {
    let row=vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12)
    ];

    for cell in &row {
        match cell {
            SpreadsheetCell::Int(i)=>println!("Integer: {}", i),
            SpreadsheetCell::Float(f)=>println!("Float: {}", f),
            SpreadsheetCell::Text(s)=>println!("Text:{}", s)
        }
    }
}

fn main(){
    exploring_enums();
}