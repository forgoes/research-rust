mod zoom;
mod market;

use crate::zoom::office;

use research_rust::garden;
use research_rust::garden::flower;
use research_rust::lib;

/*
1. binary crate's module tree & library crate's module tree are two different trees;
2. if a binary module wants use a module from the library crate, use <package_name>::<module_name>; on the contrary, not allowed?
 */

mod park {
    pub fn print(){
        println!("module by inline: park");
    }
}

fn main() {
    // inline
    park::print();

    // file
    market::print();

    // dir
    zoom::print();

    // submodule in module by use
    office::print();

    // package
    garden::garden();

    flower();

    lib();
}