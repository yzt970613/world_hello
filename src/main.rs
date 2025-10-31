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

pub trait Summary {
    fn summarize(&self) -> String;
}
pub struct Post {
    pub title: String,   // 标题
    pub author: String,  // 作者
    pub content: String, // 内容
}

impl Summary for Post {
    fn summarize(&self) -> String {
        format!("文章{}, 作者是{}", self.title, self.author)
    }
}

#[derive(Debug)]
pub struct Weibo {
    pub username: String,
    pub content: String,
}

impl Summary for Weibo {
    fn summarize(&self) -> String {
        format!("{}发表了微博{}", self.username, self.content)
    }
}

fn main() {
    let post = Post {
        title: "Rust语言简介".to_string(),
        author: "Sunface".to_string(),
        content: "Rust棒极了!".to_string(),
    };
    let weibo = Weibo {
        username: "sunface".to_string(),
        content: "好像微博没Tweet好用".to_string(),
    };

    println!("{}", post.summarize());
    println!("{}", weibo.summarize());
    println!("{:#?}", weibo);
}
