// Section : vector is homogenous size changeable collection

fn defining_vector() {
    let v = Vec::new();
    let v2 : Vec<i32> = vec![]; // vec![1] 과 같이 초기값이 들어있지 않으면 implicit type inference가 불가능 하다.

    let mut vec = Vec::from(v); // v는 소유권을 넘겨준다.
    let mut vec2 = Vec::from(v2); // v2는 vec2에게 소유권을 넘겨준다.

    // updating vector
    vec.push(1);
    vec.push(2);
    vec.push(3);
    vec.push(4);

    vec2.push(1);
    vec2.push(2);
    vec2.push(3);
    vec2.push(4);

    // Reading elements of vectors 'get' return Option, indexing return raw element type

    let third = &vec[2];
    println!("third: {:?}", third);

    let third = vec.get(2);
    if let Some(third) = third {
        println!("third: {:?}", third);
    } else {
        println!("no third element");
    }

    let mut vec3 = Vec::from(vec);
    let first = &vec3[0]; // immutable borrow occurs hear
    vec3.push(6); // mutable borrow occur hear before immutable borrow used
    // println!("first: {:?}", first); // this line use immutable use so occur compile error
    // below is same as above
    // match third {
    //     Some(third) => println!("third: {:?}", third),
    //     None => println!("no third element"),
    // }
}

// for in statement return reference not element
fn iterating_over_the_values_in_a_vector() {
    let mut v = vec![1,2,3,4,5];

    for el in &v{
        println!("el: {:?}", el);
    }

    for el in &mut v {
        // using dereference to access vector's element with '*'
        *el += 10;
    }

    for el in &v {
        println!("el: {:?}", el);
    }
}

enum DynamicElement {
    ElString(String),
    ElInt(i32),
}

fn using_an_enum_to_store_multiple_value() {
    let vec = vec![DynamicElement::ElString("hello".to_string()), DynamicElement::ElInt(0)];
    for el in &vec{
        match el {
            DynamicElement::ElString(s) => {
                println!("el_string: {:?}", s);
            },
            DynamicElement::ElInt(i) => {
                println!("el_int: {:?}", i);
            }
        }
    }
}

fn main() {
    // defining_vector();
    iterating_over_the_values_in_a_vector();
    using_an_enum_to_store_multiple_value();
}