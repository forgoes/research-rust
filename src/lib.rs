pub mod garden;
mod street;
mod restaurant;

use crate::restaurant::print;

mod lib {
    pub fn lib() {
        println!("lib");
    }
}

pub fn lib() {
    garden::garden();
    street::street();
    lib::lib();

    print();
}