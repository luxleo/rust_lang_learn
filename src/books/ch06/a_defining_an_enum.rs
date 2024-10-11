enum IPAddrV1 {
    V4,
    V6
}

fn enum_is_for_typed_instances() {
    let v4 = IPAddrV1::V4;
    let v6 = IPAddrV1::V6;
}

// Section : using enum reduce unnecessary boilerplate code when using struct

struct IPAddrV2Struct {
    kind: IPAddrV1,
    address: String,
}

struct IPAddrV3Struct {
    is_implemented: bool,
    address: String,
}

enum IPAddrV2 {
    V4(u8, u8, u8, u8),
    V6(String),
    VFuture(IPAddrV3Struct)
}

// Section : variety of types can be hold to enums value

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Message type : Quit"),
            Message::Move {x,y} => println!("Message type : Move"),
            Message::Write(_) => println!("Message type : Write"),
            Message::ChangeColor(_,_,_) => println!("Message type : ChangeColor"),
        }
    }
}

fn enum_is_for_reducing_unnecessary_code() {
    // when using struct for display kind and value
    let v4 = IPAddrV2Struct {
        kind: IPAddrV1::V4,
        address: String::from("127.0.0.1"),
    };
    let enum_v4 = IPAddrV2::V4(127, 0, 0, 1);
    let enum_v6 = IPAddrV2::V6(String::from("::1"));

    let my_message: Message = Message::Write(String::from("Hello, world!"));
    my_message.call();
}

// Section : Option<T> for complement null value

// is for
// enum Option<T> {
//     Some(T),
//     None,
// }

fn operation_with_option() {
    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;

    // below line will cause error because Option<i32> and i32 are different type
    // let sum = some_number + 13;
}

fn main() {
    let my_message: Message = Message::Write(String::from("Hello, world!"));
    my_message.call();
}

