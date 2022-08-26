pub mod office;
mod cinema;
mod zoo;

pub fn print() {
    println!("module by dir: zoom");

    office::print();
    cinema::print();
}