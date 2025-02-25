use rectangle::Rectangle;
use crate::traits::{shape::Shape, form_data::FormData};
use std::io;

use square::Square;
fn main() {
    let mut rectangle = Rectangle::new();
    let mut square = Square::new();
    println!("Do you want to calculate the area of square or rectangle/1.rectangle, 2.square /");
   let  mut string = String::new();
   //let string =  string.as_str();
   io::stdin()
       .read_line(&mut string)
       .expect("failed to read line");
let string: char = match string.trim().parse() {
    Ok(u8) => u8,
    Err(e) => return println!(" {}", e),
};

//let t = String::from("Circle");
// let string = string.parse();
//let u = String::from("rectangle");

    if '1' == string {
        loop {
            
            //read user input and supply it to Circle
            let result = rectangle.collect_data();
            if result.is_ok() {
                break;
            }
            eprintln!(":{:?}", result);
        }
        println!("{:?} area: {}", rectangle, rectangle.area());
   }
   else if '2' == string {
        loop {
            let result1 = square.collect_data();
            if result1.is_ok() {
                break;
            }
            eprintln!(":{:?}", result1);
        }
        println!("{:?} area: {}", square, square.area());
    }
}
mod rectangle;
mod square;
mod traits;
