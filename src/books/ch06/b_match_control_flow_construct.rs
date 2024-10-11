enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

impl Coin {
    fn value_in_cents(&self) -> i32 {
        match self {
            Coin::Penny => {
                println!("Penny");
                1
            },
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }
}

// Section : matches are exhaustive

fn plus_two(x: Option<i32>) -> Option<i32> {
    // below line will cause error  : error[E0004]: non-exhaustive patterns: `None` not covered
    // match x {
    //     Some(i) => Some(i + 2),
    // }
    match x {
        None => None,
        Some(i) => Some(i + 2),
    }
}

// Section : Catch-all Patterns and the _ Placeholder
impl Coin {
    fn value_in_cents_with_underscore(&self) -> Option<i32> {
        match self {
            Coin::Penny => Some(1),
            Coin::Nickel => Some(2),
            Coin::Dime => Some(3),
            _ => {
                println!("we can cover rest of Coin's value with '_'");
                None
            }
        }
    }
}

// if let sugar syntax : In other words, you can think of if let as syntax sugar for a match that
// runs code when the value matches one pattern and then ignores all other values.
fn if_let_syntax() {
    let config_max = Some(3u8);

    // 1-1
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    // 1-2
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    // both 1-1, 1-2 is same

    // 2-1
    let mut count = 0;
    let coin = None;
        match coin {
            Some(Coin::Penny) => println!("penny"),
            _ => count += 1,
        };
    //2-2
    if let Some(coin) = coin {
        println!("coin: {}", coin.value_in_cents());
    } else {
        count += 1;
    }

    // both 2-1 and 2-2 are same
}