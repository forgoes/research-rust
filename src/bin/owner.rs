
fn main() {
    let mut s1 = String::from("hello");
    s1.push_str("， world!");
    println!("{}", s1);
    let s2 = s1;
    println!("{}", s2);

    takes_ownership(s2);

    let x = 5;
    makes_copy(x);
    println!("{}", x);

    let s3 = gives_ownership();
    println!("s3: {}", s3);

    let s4 = String::from("hello");
    println!("s4: {}", s4);
    let s5 = takes_and_gives_back(s4);
    println!("s5: {}", s5);

    let len = calculate_length(&s5);
    println!("The length of '{}' is {}.", s5, len);

    let mut s6 = String::from("mutable string");
    println!("s6: {}", s6);

    let len = calculate_mut_length(&mut s6);
    println!("The length of '{}' is {}.", s6, len);
}

fn takes_ownership(s: String) {
    println!("{}", s);
}

fn makes_copy(x: i32) {
    println!("{}", x);
}

fn gives_ownership() -> String {
    String::from("yours")
}

fn takes_and_gives_back(s: String) -> String {
    s
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn calculate_mut_length(s: &mut String) -> usize {
    s.push_str(" hi");
    s.len()
}