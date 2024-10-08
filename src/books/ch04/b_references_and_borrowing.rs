// Section : Borrow with Reference
// 함수에 변수를 할당할 때 소유권이 이전되는 문제는 '참조'를 인수로 넘김으로써 소유권 이전을 방지한다.
pub fn references_and_borrowing(s: &String) {
    println!("i borrow string : {s}");
}

fn calculate_str_len(s: String) -> usize {
    s.len()
}

// 참조를 인수로 받으면 소유권 이전 side effect 가 없다.
fn calculate_str_len_ref(s: &String) -> usize {
    s.len()
}

// Section : Mutable reference
fn immutable_ref(s: &String) {
    // 참조는 소유권 이전이 아닌 대여의 개념이기 때문에 일반적으로 변경을 금지한다.
    // s.push_str(", additive string");
    println!("{s}");
}

fn mutable_ref(s : &mut String) {
    s.push_str(", additive string");
}

// 두개 이상의 mutable ref 를 사용할 수 없다.
fn multiple_mut_ref_at_sametime_is_impossible() {
    let mut s = String::from("hello");

    // 'error[E0499]: cannot borrow `s` as mutable more than once at a time' 발생
    let mr1 = &mut s;
    let mr2 = &mut s;

    mr1.push_str(" add from 1,");
    mr2.push_str(" add from 2,");
    // println!("mr1, mr2 = {mr1},{mr2}");

    // The benefit of having this restriction is that Rust can prevent data races at compile time.
    // A data race is similar to a race condition and happens when these three behaviors occur:
    //
    //     Two or more pointers access the same data at the same time.
    //     At least one of the pointers is being used to write to the data.
    //     There’s no mechanism being used to synchronize access to the data.
}

fn separate_scope_for_simultaneous_mut_ref() {
    let mut s = String::from("string");
    {
        let mr1 = &mut s;
        // some logic ...
        mr1.push_str(" add from 1,");
    }
    let mr2 = &mut s;
    mr2.push_str(" add from 2,");
    // other logic ...
}

fn mutable_ref_before_immutable_ref_usage_is_impossible() {
    let mut s = String::from("hello");
    let imr1 = &s;
    let imr2 = &s;
    let mr1 = &mut s;

    //error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
    // above error is because immutable reference's scope and mutable reference's scope overlapped
    println!("{imr1},{imr2},{mr1}");
}

fn if_mutable_ref_and_immutable_ref_scope_not_overlapped() {
    let mut s = String::from("hello");
    let imr1 = &s;
    let imr2 = &s;
    println!("{imr1},{imr2}");

    // end of imr1, imr2 scope

    // it's good because immutable ref and mutable ref's scope does not overlap
    let mr1 = &mut s;
    mr1.push_str(" add from 1,");

    // Users of an immutable reference don’t expect the value to suddenly change out from under them!
    // However, multiple immutable references are allowed because no one who is just reading the data
    // has the ability to affect anyone else’s reading of the data.
}

// Section : Dangle reference
fn dangle() -> &String { // dangle return ref to string
    let s = String::from("hello"); // new string 's'
    &s //return ref to new string 's'
} // 's' scope is over from here, so it is dropped and its memory goes away

fn no_dangle() -> String{
    let s = String::from("hello");
    s
}

fn main() {
    let str = String::from("hello");
    println!("size with ref = {}", calculate_str_len_ref(&str));
    println!("str still have ownership {str}");
    println!("size with ref = {}", calculate_str_len(str));
    // println!("str lose ownership {str}");

    let mut str = String::from("hello");
    mutable_ref(&mut str);
    println!("str : {str}");

    // raise error
    // multiple_mut_ref_at_sametime_is_impossible();
    separate_scope_for_simultaneous_mut_ref();
    if_mutable_ref_and_immutable_ref_scope_not_overlapped();
}