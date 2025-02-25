//use crate::traits::shape::Shape;
use std::io::{self, Error, ErrorKind};
use crate::traits::{shape::Shape, form_data::FormData};
#[derive(Debug)]

pub struct Square {
    side: f64,
}
impl Shape for  Square {
    fn area(&self) -> f64 {
        self.side.powf(2.0)
    }
}
impl FormData for Square {
    fn collect_data(&mut self) -> Result<(), Error> {
        println!("Enter the side : ");
        let mut side = String::new();

        io::stdin().read_line(&mut side)?;
        let side = match side.trim().parse() {
            Ok(value) => value,
            Err(_) => {
                //Returns a error of kind invalid input
                return Err(Error::new(ErrorKind::InvalidInput, "oh! sorry, please input a number"))
            }
        };
        self.side = side;
        Ok(())
    }

    fn new() -> Self  {
        Self { side: 0.0}
    }
}
