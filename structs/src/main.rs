struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
struct Rectangle {
    length: u64,
    breadth: u64,
}

// Method of rectangle
impl Rectangle {
    fn area(&self) -> u64 {
        self.length * self.breadth
    }

    // Takes more than self parameter.
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.breadth >= other.breadth && self.length >= other.length
    }

    // Associated function to create a square
    fn square(size: u64) -> Rectangle {
        Rectangle {
            length: size,
            breadth: size,
        }
    }
}

// Struct tuple, without named fields.
struct Color(i32, i32, i32);

// Unit struct
struct AlwaysEqual;

fn build_user(active: bool, username: String, email: String, sign_in_count: u64) -> User {
    User {
        active,
        username,
        email,
        sign_in_count,
    }
}

fn get_rectangle_area(rectangle: &Rectangle) -> u64 {
    rectangle.breadth * rectangle.length
}

fn main() {
    println!("Creating a new user");
    let mut user1 = User {
        active: true,
        username: String::from("John_doe"),
        email: String::from("john@gmail.com"),
        sign_in_count: 12,
    };

    let user_email = user1.email;
    println!("The email id is {user_email}");

    user1.email = String::from("new_email@gmail.com");
    println!("The updated email is {user_email}");

    // Create new user
    let user2 = build_user(
        true,
        String::from("sagar_gupta"),
        String::from("sagar@gmail.com"),
        11,
    );
    println!("Created user 2");

    let user3 = User {
        username: String::from("awaracoder"),
        ..user2
    };

    let red_color = Color(100, 0, 0);
    let red_color_first_value = red_color.0;
    println!("Red color first value {red_color_first_value}");

    let subject = AlwaysEqual;

    let shape = Rectangle {
        length: 12,
        breadth: 15,
    };

    let area_of_rectangle = get_rectangle_area(&shape);
    println!("Area of rectangle is {area_of_rectangle}");

    // println!("Failed attempt at printing rectangle {shape}");

    println!("Successful attempt at debugging rectangle {shape:#?}");

    let square = Rectangle::square(12);
}
