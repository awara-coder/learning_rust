enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

impl IpAddrKind {
    fn call(&self) {
        println!("print got called");
    }
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddrKind::V4(127, 0, 0, 1);
    let loopback = IpAddrKind::V6(String::from("::1"));
    home.call();
    loopback.call();

    experiment_with_option();

    experiment_with_match();

    experiment_with_if_let();
}

fn experiment_with_option() {
    // When declaring with existing value.
    let some_char = Some('a');
    let some_number = Some(1);

    // When declaring with null value and type.
    let absent_number: Option<i32> = None;

    // Check if the is some value.
    if absent_number.is_some() {
        println!("Absent number has a value")
    } else {
        println!("Absent number has no value");
    }

    let actual_number = some_number.unwrap_or_else(|| -1);

    println!("The actual value of some number is {actual_number}");
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

impl Coin {
    fn value_in_cents(&self) -> u32 {
        match self {
            Coin::Penny => {
                println!("Lucky Penny!");
                1
            }
            Coin::Quarter(state) => {
                println!("{state:?} state");
                25
            }
            _ => 0,
        }
    }
}

fn experiment_with_match() {
    let coin = Coin::Penny;
    let coin_value = coin.value_in_cents();
    println!("The vlaue of Penny is {coin_value}");
}

fn experiment_with_if_let() {
    let value = Some(3u8);

    match value {
        Some(max) => println!("The value is {max}"),
        _ => (),
    };

    // more concise way to avoid boilerplate.
    if let Some(max) = value {
        println!("The value is {max}")
    }

    // if let with else
    let mut count = 0;
    let coin = Coin::Dime;
    if let Coin::Quarter(state) = coin {
        println!("The quarter is of state {state:#?}")
    } else {
        count += 1;
    }

    let Coin::Quarter(state) = coin else {
        // Early exit from function without executing further code.
        return;
    };

    // Some operation with state.
}
