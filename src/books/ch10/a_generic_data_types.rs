// Section generic for abstraction
// using generic prevent for code duplicating .and add more flexibility to code
fn largest_number(x: &[i32]) -> &i32 {
    let mut local_largest = &x[0];
    for el in x {
        if el > local_largest {
            local_largest = el;
        }
    }
    local_largest
}

fn largest_character(x: &[char]) -> &char {
    let mut local_largest = &x[0];
    for el in x {
        if local_largest < el {
            local_largest = el;
        }
    }
    local_largest
}

// above two function can be replaced with generic function
// have to restrict T type with std::cmp::PartialOrd to using > operator on generic type
fn find_largest<T: PartialOrd>(list : &[T]) -> &T {
    let mut local_largest = &list[0];
    for el in list {
        if el > local_largest {
            local_largest = el;
        }
    }
    local_largest
}

fn simulate_generics() {
    let num_list = vec![1, 2, 3, 4, 5];
    let largest_num = find_largest(&num_list);
    println!("The largest number is {:?}", *largest_num);

    let char_list = ['a', 'b', 'c', 'd', 'e', 'f'];
    let largest_char = find_largest(&char_list);
    println!("The largest char is {}", largest_char);
}

// Section : generics on struct
struct PointV1<T> {
    x: T,
    y: T,
}

impl <T> PointV1<T> {
    fn get_x(&self) -> &T{
        &self.x
    }

    fn get_y(&self) -> &T {
        &self.y
    }
}

fn simulate_generic_on_struct_v1() {
    let valid_point = PointV1 {
        x: 1,
        y: 2,
    };
    throw_ownership(valid_point);
    // let invalid_point = PointV1 { x: 1, y: 2.0 }; // should be same type
}

struct PointV2<T,U> {
    x: T,
    y: U,
}

impl <T,U> PointV2<T,U> {
    fn get_x(&self) -> &T{
        &self.x
    }
    fn get_y(&self) -> &U {
        &self.y
    }
    // method의 parameter에 사용되는 generic 지시자와 impl 이후에 오는 generic 지시자와 다르다.
    fn blend_point<T2,U2> (self, other: PointV2<T2,U2>) -> PointV2<T, U2> {
        PointV2 {
            x: self.x,
            y: other.y,
        }
    }
}

fn simulate_generic_on_struct_v2() {
    let mixed_up_point = PointV2 { x: 1, y: 2.0 };
    throw_ownership(mixed_up_point);
}

fn throw_ownership<T>(target: T) {
    println!("The target is {}", target);
}

fn main() {
    simulate_generics();
}