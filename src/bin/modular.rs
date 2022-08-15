mod zoom;
mod market;

use zoom::office;

use research_rust::garden;


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
}