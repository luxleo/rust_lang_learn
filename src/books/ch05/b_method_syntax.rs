use std::path::Component::ParentDir;

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    // 변경을 가하기 위해서는 mut ref 를 이용하여 'borrowing'인 경우에도 변경될 수 있도록 한다.
    fn expand_height(&mut self, diff : u32) {
        self.height += diff;
    }

    // Associated Function 은 &self가 없어서 instantiate 필요없이 '::'으로 바로 사용할 수 있다.
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    {
        let mut rc1 = Rectangle {
            width: 10,
            height: 20,
        };
        println!("area of rc1 is {}", rc1.area());
        rc1.expand_height(20);

        assert_eq!(rc1.height, 40);

        let mut rc2 = Rectangle::square(10);
        assert_eq!(rc2.width, rc2.height);
    }
}