fn creating_new_string() {
    let s = "hello world";
    let s1 = s.to_string();
    // string indexing is not allowed
    // &s[1];

    let mut hello = String::from("Зд равствуйте");
    let mut sub_str = "";
    for (i, &item) in hello.as_bytes().iter().enumerate(){
        if item == b' '{
            sub_str = &hello[..i];
        }
    }
    println!("sub_str: {}", sub_str);

    hello.push_str(" world");
    hello.push('!');
    println!("after push to hello {}", hello);

    // concatenating using add operation
    let hello_and_good_morining = hello + ", good morning";
    println!("{}", hello_and_good_morining);

    // after add operation the ownership move to new variable uncomment: to raise error
    // println!("{hello}")
}

fn methods_for_iterating_over_strings(s: &str) {
    for c in s.chars(){
        println!("{c}");
    }
}

fn main() {
    creating_new_string();
    methods_for_iterating_over_strings(&String::from("равствуйте"));
}