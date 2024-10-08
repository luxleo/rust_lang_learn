struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn long_build_user(username: String, email: String) -> User {
    User {
        username: username,
        email: email,
        active: false,
        sign_in_count: 1,
    }
}

// 파라미터와 struct 필드명 같으면 축약형으로 할당가능
fn build_user(username: String, email: String) -> User {
    User {
        email,
        username,
        active: false,
        sign_in_count: 1,
    }
}

// Section named Tuple
#[derive(Debug)]
struct Color(i32, i32, i32);
#[derive(Debug)]
struct Point(i32, i32, i32);


// Section : Ownership of struct data
struct RefUser {
    // username: &str, // -> lifetime 지정해주어야한다. -> Missing lifetime specifier
    // email: &str,
    active: bool,
    sign_in_count: u64,
}

// Subject : unit-like struct
//We wouldn’t need any data to implement that behavior!
// You’ll see in Chapter 10 how to define traits and implement them on any type, including unit-like structs.
struct UnitLike;

fn main(){
    {
        let user1 = build_user(String::from("user1"), String::from("email@email.com"));
        let mut user2 = User {
            username: String::from("user2"),
            ..user1
        };

        user2.email = String::from("user2.email");
    }
    {
        let black = Color(0, 0, 0);
        let origin = Point(0, 0, 0);

        // struct tuple 은 destructing 할 수 없음.
        // let (r,g,b) = black;

        println!("black = {black:?}");
        println!("origin = {origin:?}");
    }
    {
        let unitLike = UnitLike;
        unitLike.into();
    }
}