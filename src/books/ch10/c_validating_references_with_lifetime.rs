// Section : role of liftime : The main aim of lifetimes is to prevent dangling references
// , which cause a program to reference data other than the data it’s intended to reference.
/*
  how rust use liftime with "Borrow Checker"
  The Rust compiler has a borrow checker that compares scopes to determine whether all borrows are valid.
  Rust compares the size of the two lifetimes and sees
*/

fn find_longest_word<'a>(word1: &'a str, word2: &'a str) -> &'a str
{
    if word1.len() > word2.len() {
        return word1;
    }
    word2
}

// find_longest_word 함수가 같은 lifetime을 반환하도록 되어있기 때문에 word1에 비해 scope가 작은
// word2를 result를 할당하면 에러 'Borrow Checker'의 lifetime 비교를 통하여 에러가 발생한다.
// fn two_lifetime_are_not_safe() {
//     let word1 = String::from("short sentence");
//     let mut result;
//     {
//         let word2 = String::from("very very long sentence");
//         //error[E0597]: `word2` does not live long enough
//         result = find_longest_word(&word1, &word2);
//     }
//
//     println!("result: {}", result);
// }

fn two_lifetime_are_safe() {
    let word1 = String::from("short sentence");
    let result;
    let word2 = String::from("very very long sentence");
    //error[E0597]: `word2` does not live long enough
    result = find_longest_word(&word1, &word2);

    println!("result: {}", result);
}

// Section Lifetime Annotations in Struct Definitions
// struct가 참조형 필드를 가질때 참조하는 데이터의 lifetime 보다 같거나 작아야한다.
#[derive(Debug)]
struct TestStruct<'a> {
    data : &'a str
}

// 컴파일 안된다.
// fn struct_lifetime_shorter() {
//     let data = String::from("data data");
//     let test = TestStruct { data: &data };
//
//     let moved_data = data;
//     println!("moved data: {}", moved_data);
//     println!("test? {:?}",test);
// }

//Section : Implicit한 lifetime을 컴파일러가 다루는 3가지 규칙
// RULE 1 : 컴파일러는 함수의 각 참조형 파라미터마다 서로다른 lifetime을 할당한다.
// fn test(a:&str, b:&str) -> &str === fn test<'a,'b>(a: &'a str, b: &'b str) -> &str

// RULE 2 : 만일 파라미터에 참조 타입이 하나이고 반환형이 참조형일때 그 lifetime은 파라미터의 lifetime과 같다.
// fn first_word<'a>(s: &'a str) -> &str  === fn first_word<'a>(s: &'a str) -> &'a str

// RULE 3 : The third rule is that, if there are multiple input lifetime parameters, but one of them is &self or &mut self because this is a method, the lifetime of self is assigned to all output lifetime
// struct등에서 메소드 구현할때 파라미터에 &self &mut self 이면 &self의 lifetime 반환한다.

//One special lifetime we need to discuss is 'static,
// which denotes that the affected reference can live for the entire duration of the program.
// All string literals have the 'static lifetime, which we can annotate as follows:
//
// let s: &'static str = "I have a static lifetime.";

fn main() {
    // two_lifetime_are_not_safe();
    two_lifetime_are_safe();
    // struct_lifetime_shorter();

}
