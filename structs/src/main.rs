struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.area() > other.area()
    }
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { height: size, width: size }
    }
}

fn main() {
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let mut user1 = build_user(String::from("ziukristian@gmail.com"), String::from("Kristian"));
    println!("My username is {}", user1.username);

    let rect1 = Rectangle {
        width: 50,
        height: 20,
    };

    println!("{:#?}", rect1);

    let area = area(&rect1);

    println!("My area is {}", area);

    println!("My area is {}", rect1.area());

    let rect2 = Rectangle {
        width: 100,
        height: 1,
    };

    println!("Can hold : {}", rect1.can_hold(&rect2));

    let rect3 = Rectangle::square(5);

    println!("{:#?}", rect3);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 5,
    }
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}
