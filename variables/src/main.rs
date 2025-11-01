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

    let mut number = 3;
    while number != 0 {
        println!("{number}");
        number -= 1;
    }
    let mut counter = 0;
    let counter = loop {
        counter = counter + 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("value fo counter {counter}");

    println!("LIFTOFF!");

    for i in 1..=10 {
        let ith_fibonacci_number = nth_fibonacci_number(i);
        println!("{i}'th number is {ith_fibonacci_number}");
    }
}

fn print_value(value: i32) {
    println!("Hi, the value is {value}")
}

fn sum(value_1: i32, value_2: i32) -> i32 {
    value_1 + value_2
}

// Fibonacci series: 0, 1, 1, 2, 3, 5, 7
fn nth_fibonacci_number(n: i64) -> i64 {
    if n <= 2 {
        return n - 1;
    }

    let mut last: i64 = 1;
    let mut second_last: i64 = 0;
    let mut solution: i64 = 1;

    for _ in 1..(n - 1) {
        solution = last + second_last;
        second_last = last;
        last = solution;
    }

    return solution;
}
