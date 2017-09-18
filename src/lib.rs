use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub struct Coordinates {
    pub Origin: i32,
    pub OppositeAnchor: i32,
}

impl Coordinates {
    pub fn new(args &[String]) -> Result<Coordinates, &'static str> {
        
    }
}

pub fn run(coordinates: Coordinates) -> Result<(), Box<Error>> {

}
