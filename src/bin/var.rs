
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    let mut x = 5;
    println!("the value of x is: {}", x);

    let mut x = 7.7;

    x = 6.6;
    println!("the value of x is: {}", x);

    let guess: u32 = "42".parse().expect("Not a number");

    println!("the value of guess is: {}", guess);

    let a: [i32; 5] = [1,2,3,4,5];
    println!("the value of a is {}", a[0])
}