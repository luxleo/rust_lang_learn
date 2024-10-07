/*
    A scalar type represents a single value. Rust has four primary scalar types: integers,
    floating-point numbers,
    Booleans,
    and characters.
 */
use std::io;

pub fn scalar_type() {
    // floating-point types
    let x = 2.0; // f64 is default
    let x_32: f32 = 3.0;

    // numeric operations
    println!("x + x_32 = {}", x + x_32);
    println!("x - x_32 = {}", x_32.abs());
    println!("x * x_32 = {}", x_32.abs());
    println!("x / x_32 = {}", x_32.abs() / x_32.abs());
    println!("x % x_32 = {}", x_32.abs() % x_32.abs());
}

/*
    Compound types can group multiple values into one type.
    Rust has two primitive compound types: tuples and arrays.
 */
pub fn compound_type() {
    //tuple
    let tup = (5.13, 3, 'c'); // tuple can be set of different types of data
    let (a, b, c) = tup; // destructure is possible

    println!("a = {}, b = {}, c = {}", a, b, c);
    println!("a = {}, b = {}, c = {}", tup.0, tup.1, tup.2);

    // array
    let arr = [1, 2, 3];
    let arr_1 = [3; 5]; // initializing array with specific value
    let arr_2: [i16; 10] = [1,2,3,4,5,6,7,8,9,10];

    println!("arr[1] : {:?}", arr);
    println!("arr_1 = {:?}", arr_1);
    println!("arr_2 = {:?}", arr_2);
}

fn main() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    loop {
        let mut index = String::new();
        println!("Enter the index you want to search for (q for quit) : ");
        io::stdin()
            .read_line(&mut index)
            .expect("Failed to read line");

        if index.trim() == "q" {
            break;
        }

        let index : u32 = match index.trim().parse() {
            Ok(num) => {
                if num > 5 {
                    println!("Please enter a number less then 5!");
                    continue;
                }
                num
            },
            Err(_) => {
                println!("Please enter a number!");
                continue;
            },
        };

        println!("your guess is {}",arr[index as usize]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn scalar_type_test() {
        scalar_type();
    }

    #[test]
    fn compound_type_test() {
        compound_type();
    }
}
