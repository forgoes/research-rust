use crate::zoom::cinema;

pub fn print() {
    super::zoo::print();

    self::cinema::print();
    super::cinema::print();

    cinema::print();
    println!("submodule by file: office");
}
