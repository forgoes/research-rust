pub mod garden;
mod street;

mod lib {
    pub fn lib() {
        println!("lib");
    }
}

fn lib() {
    garden::garden();
    street::street();
    lib::lib();
}