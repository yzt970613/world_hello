pub trait Draw {
    fn draw(&self) -> String;
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    // pub label: String,
}

impl Draw for Button {
    fn draw(&self) -> String {
        // 绘制按钮的代码
        format!("Button: ({}x{})", self.width, self.height)
    }
}

struct SelectBox {
    width: u32,
    height: u32,
    // options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) -> String {
        // 绘制SelectBox的代码
        format!("SelectBox:  ({}x{})", self.width, self.height)
    }
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
            println!("{}", component.draw());
        }
    }
}

// pub struct Screen<T: Draw> {
//     pub components: Vec<T>,
// }
// impl<T> Screen<T>
// where
//     T: Draw,
// {
//     pub fn run(&self) {
//         for component in self.components.iter() {
//             component.draw();
//         }
//     }
// }
fn main() {
    let screen = Screen {
        components: vec![
            Box::new(Button {
                width: 50,
                height: 10,
                // label: String::from("OK"),
            }),
            Box::new(SelectBox {
                width: 75,
                height: 10,
                // options: vec![
                //     String::from("Yes"),
                //     String::from("Maybe"),
                //     String::from("No"),
                // ],
            }),
            // SelectBox {
            //     width: 75,
            //     height: 20,
            //     // options: ,
            // },
        ],
    };

    screen.run();
}
