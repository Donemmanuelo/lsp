//use crate::traits::shape::Shape;
use std::io::{self, Error, ErrorKind};
use crate::traits::{shape::Shape, form_data::FormData};

#[derive(Debug)]
pub struct Rectangle {
    length: f64,
    width: f64,
}
impl Shape for Rectangle {
    fn area(&self)  -> f64 {
        self.length * self.width
    }
}
impl Rectangle {
    pub  fn from(width: f64, length: f64) -> Self {
        Self {width, length}
    }
}

impl FormData for Rectangle{
    fn collect_data(&mut self) -> Result<(), Error> {
        println!("Enter the Rectangle length: ");
        let mut length = String::new();

        io::stdin().read_line(&mut length)?;
        let length = match length.trim().parse() {
            Ok(value) => value,
            Err(_) => {
                //Returns a error of kind invalid input
                return Err(Error::new(ErrorKind::InvalidInput, "oh! sorry, please input a number"))
            }
        };
        println!("Enter the Rectangle length: ");
        let mut width = String::new();

        io::stdin().read_line(&mut width)?;
        let width = match width.trim().parse() {
            Ok(value) => value,
            Err(_) => {
                //Returns a error of kind invalid input
                return Err(Error::new(ErrorKind::InvalidInput, "oh! sorry, please input a number"))
            }
        };
        self.width = width;
        self.length = length;
        Ok(())
    }

    fn new() -> Self  {
        Self { length: 0.0, width: 0.0}
    }
}
