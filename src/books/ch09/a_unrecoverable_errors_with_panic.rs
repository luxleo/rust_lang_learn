// Section : explicitly calling the panic!, By default, these panics will
// print a failure message, unwind, clean up the stack, and quit.

/*
Unwinding the Stack or Aborting in Response to a Panic
By default, when a panic occurs the program starts unwinding,
which means Rust walks back up the stack and cleans up the data from each function it encounters.
However, walking back and cleaning up is a lot of work. Rust, therefore, allows you to choose
the alternative of immediately aborting, which ends the program without cleaning up.

Memory that the program was using will then need to be cleaned up by the operating system.
If in your project you need to make the resultant binary as small as possible,
you can switch from unwinding to aborting upon a panic by adding panic = 'abort' to the appropriate
[profile] sections in your Cargo.toml file. For example, if you want to abort on panic in
release mode, add this:

[profile.release]
panic = 'abort'
*/
use std::fs::File;
use std::io::ErrorKind;

fn explicit_panic_call() {
    panic!("crash and burn");
}

fn implicit_panic_call() {
    let vec = vec![1, 2, 3];

    // below code will call panic to protect access to non-exist element
    vec[100];
}

// Section : recoverable error with Result
// Result<T,E> {
//  OK(T), Err(E)
//}
fn open_file_with_result() {
    let file_result = File::open("hello.txt");
    let file = match file_result {
        Ok(file) => file,
        Err(error) => panic!("File could not be opened, {error:?}"),
    };
}

// parent 에러가 여러종류의 서브 에러를 가질떄 분기하기 위해서 사용됨.
fn matching_on_different_errors() {
    let greeting_file_open = File::open("hello.txt");
    let greeting_file = match greeting_file_open {
        Ok(file) => file,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
            },
            other_error => panic!("There was a problem opening the file: {:?}", other_error),
        }
    };

    // below code is same as above but remove unnecessary match indentation
    let greeting_file = File::open("hello.txt").unwrap_or_else(|err| {
        if err.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Tried to create file but there was a problem: {:?}", error);
            })
        } else {
            panic!("Tried to create file but there was a problem: {:?}", err);
        }
    });
}

// using unwrap and expect for abbreviate throwing panic call
fn shorts_cut_for_throw_panic() {
    File::open("hello.txt").unwrap();

    // use expect for more detailed and comprehensible message
    File::open("hello.txt")
        .expect("Something went wrong while trying to open hello.txt");
}