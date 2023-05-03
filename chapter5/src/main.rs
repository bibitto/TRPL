fn main() {
    lesson_1();
    lesson_2();
    lesson_3();
}

// 5-1
fn lesson_1() {
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    };

    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    // emailを変更
    user1.email = String::from("another@example.com");

    // userを生成する関数
    fn build_user(email: String, username: String) -> User {
        User {
            email,
            username,
            active: true,
            sign_in_count: 1,
        }
    };

    // user1のインスタンスからuser2のインスタンスを作成する
    let user2 = User {
        email: String::from("hogehoge@example.com"),
        username: String::from("fugafuga"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };
    // こんな感じでも書ける
    let user3 = User {
        email: String::from("hogehoge@example.com"),
        username: String::from("fugafuga"),
        ..user1
    };

    // タプル構造体ってやつもある
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

// 5-2
fn lesson_2() {
    let width1 = 30;
    let height1 = 50;
    println!(
        " The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    // 上記をタプルを使ってリファクタリング
    let rect1 = (30, 50);
    println!(
        " The area of the rectangle is {} square pixels.",
        area_2(rect1)
    );

    // 上記を構造体でさらにリファクタリング
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    };
    let rect2 = Rectangle {
        width: 30,
        height: 50
    };
    println!(
        " The area of the rectangle is {} square pixels.",
        rect2.width * rect2.height
    );

    println!("The Rectangle is {:?}", rect2);
    println!("The Rectangle is {:#?}", rect2);
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn lesson_3() {
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        // method #1
        fn area(&self) -> u32 {
            self.width * self.height
        }
        // method #2
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
        // 関連関数（ ex. String::from ） -> 引数にselfを取らないやつ
        fn square(size: u32) -> Rectangle {
            Rectangle {
                width: size,
                height: size,
            }
        }
    }
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let rect4 = Rectangle::square(50);
    println!(
        "The area of the rectangle is {} square pixels.",
        rect4.area()
    );
}