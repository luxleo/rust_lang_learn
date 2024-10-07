
/*
TODO:
1. 소유권 개념 힙,스택 엮어서 정리
2. x.clone()으로 deep copy
3. Copy 자료형 (u32, float64,boolean,char) 등은 소유권으로 취급하지 않는다.
*/
pub fn core_of_ownership() {
    //ownership move with non Copy data-type in 3 cases
    // case 1 : allocate to other variable
    let x = String::from("Hello");
    let y = x;
    println!("{}", y);
    // below code throw an error because ownership is moved to y
    // println!("{}", x);

    // case 2 : pass data to function's parameter
    let x = String::from("Hello");
    takes_ownership(x);

    // case 3 : take ownership from function's return value
    let x = String::from("Hello");
    let y = takes_ownership_and_gives_back(x);
    // below code throw an error because ownership is moved to y
    // println!("x is {x}");
    takes_ownership(y);
}

fn takes_ownership(data: String) {
    println!("take ownership: {}", data);
}

fn takes_ownership_and_gives_back(data: String) -> String {
    println!("take ownership and gives_back: {}", data);
    data
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn core_of_ownership_test() {
        core_of_ownership();
    }

}