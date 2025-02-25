use std::io::Error;

pub trait FormData{
    fn new() -> Self;
    fn collect_data(&mut self)  -> Result<(), Error>;
}