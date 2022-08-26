fn main() {
    let mut x = 3;
    println!("before x: {}", x);
    print_hello(x);
    println!("after x: {}", x);

    let t:(i32, char) = (10, 'a');

    let y = {
        let s = 3;
        3 + 1
    };
    println!("y: {}", y);

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

fn print_hello(mut x: i32) {
    x = 4;
    println!("hello functions: {}", x);
}

fn print_tup(x: i32, y: char) -> i32 {
    println!("hello tup: {}, {}", x, y);
    x + 1
}