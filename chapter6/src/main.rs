fn main() {
    println!("Hello, world!");
}

fn lesson1() {
    // enum IpAddrKind {
    //     V4,
    //     V6
    // }
    // struct IpAddr {
    //     kind: IpAddrKind,
    //     address: String,
    // }
    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };
    // let loopbook = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1")
    // };

    // 上記をenumだけで簡潔にかける
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

}

fn lesson2() {
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    fn value_in_cents(coin: Coin) -> u32 {
        match coin {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            },
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }
}