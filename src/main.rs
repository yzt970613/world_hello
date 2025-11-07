// pub trait Draw {
//     fn draw(&self) -> String;
// }

// pub struct Button {
//     pub width: u32,
//     pub height: u32,
//     // pub label: String,
// }

// impl Draw for Button {
//     fn draw(&self) -> String {
//         // 绘制按钮的代码
//         format!("Button: ({}x{})", self.width, self.height)
//     }
// }

// struct SelectBox {
//     width: u32,
//     height: u32,
//     // options: Vec<String>,
// }

// impl Draw for SelectBox {
//     fn draw(&self) -> String {
//         // 绘制SelectBox的代码
//         format!("SelectBox:  ({}x{})", self.width, self.height)
//     }
// }

// impl Draw for String {
//     fn draw(&self) -> String {
//         format!("String: {}", self)
//     }
// }

// pub struct Screen {
//     pub components: Vec<Box<dyn Draw>>,
// }

// impl Screen {
//     pub fn run(&self) {
//         for component in self.components.iter() {
//             component.draw();
//             println!("{}", component.draw());
//         }
//     }
// }

// // pub struct Screen<T: Draw> {
// //     pub components: Vec<T>,
// // }
// // impl<T> Screen<T>
// // where
// //     T: Draw,
// // {
// //     pub fn run(&self) {
// //         for component in self.components.iter() {
// //             component.draw();
// //         }
// //     }
// // }
// fn main() {
//     // let screen = Screen {
//     //     components: vec![
//     //         Box::new(Button {
//     //             width: 50,
//     //             height: 10,
//     //             // label: String::from("OK"),
//     //         }),
//     //         Box::new(SelectBox {
//     //             width: 75,
//     //             height: 10,
//     //             // options: vec![
//     //             //     String::from("Yes"),
//     //             //     String::from("Maybe"),
//     //             //     String::from("No"),
//     //             // ],
//     //         }),
//     //         // SelectBox {
//     //         //     width: 75,
//     //         //     height: 20,
//     //         //     // options: ,
//     //         // },
//     //     ],
//     // };

//     let screen = Screen {
//         components: vec![Box::new(String::from("Hi"))],
//     };

//     screen.run();
// }

// trait Draw {
//     fn draw(&self) -> Self;
// }

// #[derive(Debug, Clone)]
// struct Button;
// impl Draw for Button {
//     fn draw(&self) -> Self {
//         return self.clone();
//     }
// }

// fn main() {
//     let button = Button;
//     let newb = button.draw();
//     println!("{:?}", newb);
// }

// struct Counter {
//     count: u32,
// }
// pub trait Iterator {
//     type Item;

//     fn next(&mut self) -> Option<Self::Item>;
// }

// impl Iterator for Counter {
//     type Item = u32;

//     fn next(&mut self) -> Option<Self::Item> {
//         // --snip--
//         Some(self.count)
//     }
// }

// fn main() {
//     let mut c = Counter { count: 16 };
//     c.next();
//     print!("{:?}", c.next());
// }

// use std::ops::Add;

// #[derive(Debug, PartialEq)]
// struct Point {
//     x: i32,
//     y: i32,
// }

// impl Add for Point {
//     type Output = Point;

//     fn add(self, other: Point) -> Point {
//         Point {
//             x: self.x + other.x,
//             y: self.y + other.y,
//         }
//     }
// }

// struct Millimeters(u32);
// struct Meters(u32);

// impl Add<Meters> for Millimeters {
//     type Output = Millimeters;

//     fn add(self, other: Meters) -> Millimeters {
//         Millimeters(self.0 + (other.0 * 1000))
//     }
// }

// fn main() {
//     assert_eq!(
//         Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
//         Point { x: 3, y: 3 }
//     );
//     let result = Millimeters(5000) + Meters(2);
//     println!("5000 millimeters + 2 meters = {} millimeters", result.0);
// }

// trait Pilot {
//     fn fly(&self);
// }

// trait Wizard {
//     fn fly(&self);
// }

// struct Human;

// impl Pilot for Human {
//     fn fly(&self) {
//         println!("This is your captain speaking.");
//     }
// }

// impl Wizard for Human {
//     fn fly(&self) {
//         println!("Up!");
//     }
// }

// impl Human {
//     fn fly(&self) {
//         println!("*waving arms furiously*");
//     }
// }

// fn main() {
//     let person = Human;
//     person.fly();
//     Pilot::fly(&person); // 调用Pilot特征上的方法
//     Wizard::fly(&person); // 调用Wizard特征上的方法
//     person.fly(); // 调用Human类型自身的方法
// }
trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

fn main() {
    println!("A baby dog is called a {}", Dog::baby_name());
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
}
