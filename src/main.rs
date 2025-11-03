// mod my {
//     pub struct Rectangle {
//         width: u32,
//         pub height: u32,
//     }

//     impl Rectangle {
//         pub fn new(width: u32, height: u32) -> Self {
//             Rectangle { width, height }
//         }
//         pub fn width(&self) -> u32 {
//             return self.width;
//         }
//         pub fn height(&self) -> u32 {
//             return self.height;
//         }
//     }
// }

// fn main() {
//     let rect1 = my::Rectangle::new(30, 50);

//     println!("{}", rect1.width()); // OK
//     println!("{}", rect1.height()); // OK
//                                     // println!("{}", rect1.width); // Error - the visibility of field defaults to private
//     println!("{}", rect1.height); // OK
// }

// struct Point<T, U> {
//     x: T,
//     y: U,
// }

// impl<T, U> Point<T, U> {
//     fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
//         Point {
//             x: self.x,
//             y: other.y,
//         }
//     }
// }

// fn main() {
//     let p1: Point<i32, f64> = Point { x: 5, y: 10.4 };
//     let p2: Point<&str, char> = Point {
//         x: "Hello",
//         y: '中',
//     };

//     let p3: Point<i32, char> = p1.mixup(p2);

//     println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

//     const MAX_POINTS: u32 = 100_000;
//     println!("The maximum points is: {}", MAX_POINTS);
// }

// pub trait Summary {
//     fn summarize(&self) -> String;
// }
// pub struct Post {
//     pub title: String,   // 标题
//     pub author: String,  // 作者
//     pub content: String, // 内容
// }

// impl Summary for Post {
//     fn summarize(&self) -> String {
//         format!("文章{}, 作者是{}", self.title, self.author)
//     }
// }

// #[derive(Debug)]
// pub struct Weibo {
//     pub username: String,
//     pub content: String,
// }

// impl Summary for Weibo {
//     fn summarize(&self) -> String {
//         format!("{}发表了微博{}", self.username, self.content)
//     }
// }

// fn main() {
//     let post = Post {
//         title: "Rust语言简介".to_string(),
//         author: "Sunface".to_string(),
//         content: "Rust棒极了!".to_string(),
//     };
//     let weibo = Weibo {
//         username: "sunface".to_string(),
//         content: "好像微博没Tweet好用".to_string(),
//     };

//     println!("{}", post.summarize());
//     println!("{}", weibo.summarize());
//     println!("{:#?}", weibo);
// }

// fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
//     let mut largest = list[0];

//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];

//     let result = largest(&number_list);
//     println!("The largest number is {}", result);

//     let char_list = vec!['y', 'm', 'a', 'q'];

//     let result = largest(&char_list);
//     println!("The largest char is {}", result);
// }

// use std::ops::Add;

// // 为Point结构体派生Debug特征，用于格式化输出
// #[derive(Debug)]
// struct Point<T: Add<T, Output = T>> {
//     //限制类型T必须实现了Add特征，否则无法进行+操作。
//     x: T,
//     y: T,
// }

// impl<T: Add<T, Output = T>> Add for Point<T> {
//     type Output = Point<T>;

//     fn add(self, p: Point<T>) -> Point<T> {
//         Point {
//             x: self.x + p.x,
//             y: self.y + p.y,
//         }
//     }
// }

// fn add2<T: Add<T, Output = T>>(a: T, b: T) -> T {
//     a + b
// }

// fn main() {
//     let p1 = Point {
//         x: 1.1f32,
//         y: 1.1f32,
//     };
//     let p2 = Point {
//         x: 2.1f32,
//         y: 2.1f32,
//     };
//     println!("{:?}", &p1);
//     println!("{:?}", &p2);
//     let p3 = p1 + p2;
//     // println!("{:?}", add2(p1, p2));
//     println!("{:?}", p3);

//     let p3 = Point { x: 1i32, y: 1i32 };
//     let p4 = Point { x: 2i32, y: 2i32 };
//     println!("{:?}", add2(p3, p4));
// }

// #![allow(dead_code)]

// use std::fmt;
// use std::fmt::Display;

// #[derive(Debug, PartialEq)]
// enum FileState {
//     Open,
//     Closed,
// }

// #[derive(Debug)]
// struct File {
//     name: String,
//     data: Vec<u8>,
//     state: FileState,
// }

// impl Display for FileState {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match *self {
//             FileState::Open => write!(f, "OPEN txt"),
//             FileState::Closed => write!(f, "CLOSED txt"),
//         }
//     }
// }

// impl Display for File {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "<{} ({})>", self.name, self.state)
//     }
// }

// impl File {
//     fn new(name: &str) -> File {
//         File {
//             name: String::from(name),
//             data: Vec::new(),
//             state: FileState::Closed,
//         }
//     }
// }

// fn main() {
//     let f6 = File::new("f6.txt");
//     //...
//     println!("{:?}", f6);
//     println!("{}", f6);
// }

trait Draw {
    fn draw(&self) -> String;
}

impl Draw for u8 {
    fn draw(&self) -> String {
        format!("u8: {}", *self)
    }
}

impl Draw for f64 {
    fn draw(&self) -> String {
        format!("f64: {}", *self)
    }
}

// 若 T 实现了 Draw 特征， 则调用该函数时传入的 Box<T> 可以被隐式转换成函数参数签名中的 Box<dyn Draw>
fn draw1(x: Box<dyn Draw>) -> String {
    // 由于实现了 Deref 特征，Box 智能指针会自动解引用为它所包裹的值，然后调用该值对应的类型上定义的 `draw` 方法
    x.draw()
}

fn draw2(x: &dyn Draw) -> String {
    x.draw()
}

fn main() {
    let x = 1.1f64;
    // do_something(&x);
    let y = 8u8;

    // x 和 y 的类型 T 都实现了 `Draw` 特征，因为 Box<T> 可以在函数调用时隐式地被转换为特征对象 Box<dyn Draw>
    // 基于 x 的值创建一个 Box<f64> 类型的智能指针，指针指向的数据被放置在了堆上
    //  draw1(Box::new(x));
    println!("{:?}", draw1(Box::new(x)));
    // 基于 y 的值创建一个 Box<u8> 类型的智能指针
    draw1(Box::new(y));
    draw2(&x);
    draw2(&y);
    // println!("{}", x.draw());
    // println!("{}", y.draw());
}
