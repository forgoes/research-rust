struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn area_a(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("jayden"),
        email: String::from("hongkang@hongkang.name"),
        sign_in_count: 1,
    };
    user1.active = false;

    let user2 = build_user(String::from("jayden"), String::from("hongkang@hongkang.name"));

    println!("user2: {}", user2.username);

    let user3 = User {
        email: String::from("another@example.com"),
        ..user2
    };

    let width1 = 30;
    let height1 = 50;
    println!(
        "The area of the rectangle is {} square pixels.",
        area((width1, height1))
    );

    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 30,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area_a(&rect1)
    );

    println!(
        "The rectangle is {:#?}.",
        rect1,
    );

    dbg!(&rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    let rect2 = Rectangle::square(30);
}