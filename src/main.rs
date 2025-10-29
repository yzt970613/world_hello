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
}
