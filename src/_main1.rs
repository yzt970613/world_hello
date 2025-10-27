// main.rs
mod test; // 首先声明模块
use test::test; // 使用 use 引入特定项
use test::Coin;

#[derive(Debug)]
enum IpAddr {
    // Ipv4,
    Ipv6,
}

enum Action {
    Say(String),
    MoveTo(i32, i32),
    ChangeColorRGB(u16, u16, ()), // Removed the unused u16 parameter
}

#[derive(Debug)]
enum Direction {
    East,
    West,
    North,
    South,
}

fn main() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five: Option<i32> = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("six: {:?}, none: {:?}", six, none);

    let age: Option<i32> = Some(30);
    println!("在匹配前，age是{:?}", age);
    if let Some(age) = age {
        println!("匹配出来的age是{}", age);
    }

    println!("在匹配后，age是{:?}", age);

    let age = Some(30);
    println!("在匹配前，age是{:?}", age);
    match age {
        Some(x) => println!("匹配出来的age是{}", x),
        _ => (),
    }
    println!("在匹配后，age是{:?}", age);

    let dire = Direction::South;
    match dire {
        Direction::East => println!("East"),
        other => println!("other direction: {:?}", other),
    };

    let some_u8_value = 1u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
    let actions = [
        Action::Say("Hello Rust".to_string()),
        Action::MoveTo(1, 2),
        Action::ChangeColorRGB(255, 255, ()),
    ];
    for action in actions {
        match action {
            Action::Say(s) => {
                println!("{}", s);
            }
            Action::MoveTo(x, y) => {
                println!("point from (0, 0) move to ({}, {})", x, y);
            }
            Action::ChangeColorRGB(r, g, _) => {
                println!(
                    "change color into '(r:{}, g:{}, b:0)', 'b' has been ignored",
                    r, g,
                );
            }
        }
    }

    test();

    let coin = Coin::Penny;
    let value = test::value_in_cents(coin);
    println!("The value of the coin is: {} cents", value);

    let ip1: IpAddr = IpAddr::Ipv6;
    let ip_str = match ip1 {
        // IpAddr::Ipv4 => "127.0.0.1",
        _ => "::1",
    };

    // 打印 ip1
    println!("IpAddr: {:?}", ip1);
    println!("{}", ip_str);
}

// // Vec<u8> 示例
// fn process_vec(mut vec_bytes: Vec<u8>) {
//     vec_bytes.push(4);
//     println!("Vec bytes: {:?}", vec_bytes);
// }

// // &[u8] 示例
// fn process_slice(slice: &[u8]) {
//     println!("Slice bytes: {:?}", slice);
// }

// fn main() {
//     let vec_bytes = vec![1, 2, 3];
//     process_vec(vec_bytes); // 所有权转移

//     let arr = [1, 2, 3];
//     let slice = &arr;
//     process_slice(slice); // 借用数据
// }
