const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    let mut x = 5;
    println!("Value of x : {x}");
    x = 6;
    println!("Updated value of x: {x}");

    println!("The number of seconds in 3 hours is  {THREE_HOURS_IN_SECONDS}");

    let x = x + 1;

    {
        let x = x * 2;
        println!("the update value of x in scopte is {x}");
    }

    println!("The value of x outside scope is {x}");

    let tup: (i32, f64, i32) = (1, 2.3, 4);

    // let (x, y, z) = tup;
    let one = tup.0;
    let two_point_three = tup.1;
    let four = tup.2;
    println!("Tuple values are {one} {two_point_three} {four}");
    print_value(12);
    let total = sum(12, 12);
    println!("the total is {total}");
}

fn print_value(value: i32) {
    println!("Hi, the value is {value}")
}

fn sum(value_1: i32, value_2: i32) -> i32 {
    value_1 + value_2
}
