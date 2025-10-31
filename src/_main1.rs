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


// use std::str::FromStr;

// fn get_count_item(s: &str) -> (u64, &str) {
//     let mut it = s.split(' ');
//     println!("it: {:?}", it);
//     let (Some(count_str), Some(item)) = (it.next(), it.next()) else {
//         panic!("Can't segment count item pair: '{s}'");
//     };
//     println!("count_str: '{count_str}', item: '{item}'");
//     let Ok(count) = u64::from_str(count_str) else {
//         panic!("Can't parse integer: '{count_str}'");
//     };
//     println!("count: {count}");
//     // error: `else` clause of `let...else` does not diverge
//     // let Ok(count) = u64::from_str(count_str) else { 0 };
//     (count, item)
// }

// fn main() {
//     // assert_eq!(get_count_item("3 chairs"), (3, "chairs"));
//     // let some_value: Option<i32> = None;

//     // // 使用 let-else 解构 Option
//     // let Some(x) = some_value else {
//     //     println!("值是 None，提前返回");
//     //     return;
//     // };

//     // println!("解构成功: x = {}", x); // 如果 some_value 是 None，这行不会执行

//     let x = Some(5);
//     let y = 10;

//     match x {
//         Some(50) => println!("Got 50"),
//         Some(y) => println!("Matched, y = {:?}", y),
//         _ => println!("Default case, x = {:?}", x),
//     }

//     println!("at the end: x = {:?}, y = {:?}", x, y);
// }

// struct Point {
//     x: i32,
//     y: i32,
// }

// #[derive(Debug, PartialEq)]
// enum Test {
//     A,
// }

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    // let p = Point { x: 0, y: 7 };

    // let Point { x, y } = p;
    // assert_eq!(0, x);
    // assert_eq!(7, y);

    // let test = Test::A;
    // match test {
    //     Test::A => println!("A"),
    // }
    // assert_eq!(test, Test::A);
    // println!("test matched");

    // let arr: &[u16] = &[114, 514];

    // if let [x, ..] = arr {
    //     assert_eq!(x, &114);
    // }

    // if let &[.., y] = arr {
    //     assert_eq!(y, 514);
    // }

    // let arr: &[u16] = &[];

    // assert!(matches!(arr, [..]));
    // assert!(!matches!(arr, [x, ..]));
    // println!("arr matched");
    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }

    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 55 };

    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => {
            println!("Found an id in range: {}", id_variable)
        }
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => {
            println!("Found some other id: {}", id)
        }
    }
    // 绑定新变量 `p`，同时对 `Point` 进行解构
    let p @ Point { x: px, y: py } = Point { x: 10, y: 23 };
    println!("x: {}, y: {}", px, py);
    println!("{:?}", p);

    let point = Point { x: 10, y: 5 };
    if let p @ Point { x: 10, y } = point {
        println!("x is 10 and y is {} in {:?}", y, p);
    } else {
        println!("x was not 10 :(");
    }

    match 3 {
        num @ (1 | 2) => {
            println!("{}", num);
        }
        _ => {}
    }
}
