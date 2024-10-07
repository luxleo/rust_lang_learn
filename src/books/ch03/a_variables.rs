//Constants are valid for the entire time a program runs, within the scope in which they were declared.
const DAY_IN_SECONDS: u64 = 24 * 60 * 60 * 7;
pub fn let_is_mutable() {
    /*
    below code will cause error
    let x = 5;
    x = 3;
    println!("x is {x}");
     */

    // for 'let variable' to allow change, decorate with 'mut' directive
    let mut x = 5;
    x = 3;
    println!("x is {}", x);

    // shadowing is feature of rust
    let shadow_x = "dragon";
    println!("shadow_x is {}", shadow_x);
    let shadow_x = shadow_x.len();
    println!("shadow_x is {}", shadow_x);
}
