// Section : Slice
// definition of slice : A string slice is a reference to part of a String : ptr(시작위치), len(길이)로 구성

// Section : usage of slice => 전체 collection(string, array 등)의 부분을 참조해야 할때 필요하다
// Copy 타입의 원시값으로 참조할 경우 참조 당하는 collection의 참조가 유효하지 않은 경우 알아낼 수 없다.

// 이 함수의 경우 s가 해당함수의 실행 이후에 s.clear()등으로 변화가 생겨도 연동시키는 작업을 해야한다.
fn first_word_none_slice(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i
        }
    }
    s.len()
}

// 참조의 일부를 넘김으로써 참조값이 기존 값과 변화 된다면 에러를 발생시킨다.
fn first_word_v1(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '{
            return &s[..i];
        }
    }
    &s[..]
}

// literal String은 Slice이다. 따라서 &String 대신 &str을 사용하여 일반성을 획득한다.
fn first_word_v2(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn slice_is_applicable_generally() {
    let a = [1, 2, 3, 4, 5];
    let sub_arr = &a[1..3];

    // [1,3] == &[1,3]
    assert_eq!(sub_arr, &[2,3]);
}

fn main() {
    {
        let mut s = String::from("hello world");

        let word = first_word_none_slice(&s); // word will get the value 5

        s.clear(); // this empties the String, making it equal to ""

        // word still has the value 5 here, but there's no more string that
        // we could meaningfully use the value 5 with. word is now totally invalid!
        assert_eq!(word, 5);
    }
    {
        let mut s = String::from("hello world");
        let first_word = first_word_v1(&s);

        //error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable 발생시킨다.
        // s.clear();
        println!("first_word is {}", first_word);
    }
    {
        slice_is_applicable_generally();
    }
}