use std::io;

// if는 expression 으로 동작한다.

pub fn if_else_if_executed_order() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // only "number is divisible by 3" printed because the function executed sequentially
    // and when one if or if-else expression meet the cause, whole expression's evaluate is end
}

pub fn if_in_let_statement() {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("type message");

        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };

        // because
        let result = if input > 50 { "upper class" } else { "down class" };

        // uncomment below code to find out inconsistent data type in each if and else
        // let wrong_result = if true { 5 } else { "the if and else type should be same" };
    }
}

// loop 는 break와 함께 expression으로 동작할 수도 있다. loop, while, for
pub fn returning_value_from_loop() {
    let mut count = 0;
    let result = loop {
        if count == 10 {
            break count * 2;
        }
        count += 1;
    };
}

//Loop Labels to Disambiguate Between Multiple Loops
pub fn labeled_loop() {
    let mut count = 0;
    /*
    If you have loops within loops, break and continue apply to the innermost loop at that point.
    You can optionally specify a loop label on a loop that you can then use with break or continue
    to specify that those keywords apply to the labeled loop instead of the innermost loop.

    Loop labels must begin with a single quote. Here’s an example with two nested loops:
     */
    'count_loop: loop {
        let mut remaining = 3;
        println!("COUNT : {count}");

        loop {
            if remaining < 0 { break;}
            if count == 2 {
                println!("count is equal to 2!!");
                break 'count_loop;
            }
            println!("COUNT : {count} , remaining : {remaining}");
            remaining -= 1;
        }
        count += 1;
    }
}

pub fn while_and_for() {
    let arr = [3; 5];
    let mut index = 0;
    while index < arr.len() {
        println!("{index} th element is {}", arr[index]);
        index += 1;
    }
}

pub fn iterating_collection_with_for() {
    let arr: [u32; 10] = [1,2,3,4,5,6,7,8,9,10];

    for element in arr.iter() {
        println!("{element}");
    }
}

pub fn count_with_for() {
    println!("count_with_for called");
    for num in (1..10).rev() {
        println!("{num}");
    }
}

fn main() {
    if_else_if_executed_order();
    // labeled_loop();
    while_and_for();
    iterating_collection_with_for();
    count_with_for();
}