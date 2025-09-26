fn main() {
    // 2.2
    two2_1();
    two2_2();
    two2_3();
    two2_4();

    // 2.1
    two1_1();
    two1_2();
    two1_3();
    two1_4();

    practive();

    assert_eq!(ret_unit_type(), ())
}

// 2.3

fn ret_unit_type() {
    let x = 1;
    // if 语句块也是一个表达式，因此可以用于赋值，也可以直接返回
    // 类似三元运算符，在Rust里我们可以这样写
    let _y = if x % 2 == 1 { "odd" } else { "even" };
    // 或者写成一行
    let _z = if x % 2 == 1 { "odd" } else { "even" };
}

// 2.2
fn two2_4() {
    for i in 1..5 {
        println!("{}", i);
    }
    // for i in 'a'..='z' {
    //     println!("{}", i);
    // }
}

fn two2_3() {
    // 二进制为00000010
    let a: i32 = 2;
    // 二进制为00000011
    let b: i32 = 3;

    println!("(a & b) value is {}", a & b);

    println!("(a | b) value is {}", a | b);

    println!("(a ^ b) value is {}", a ^ b);

    println!("(!b) value is {} ", !b);

    println!("(a << b) value is {}", a << b);

    println!("(a >> b) value is {}", a >> b);

    let mut a = a;
    // 注意这些计算符除了!之外都可以加上=进行赋值 (因为!=要用来判断不等于)
    a <<= b;
    println!("(a << b) value is {}", a);
}

fn two2_2() {
    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);

    println!("abc (f32)");
    println!("   0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits());
    println!("         0.3: {:x}", (abc.2).to_bits());
    println!();

    println!("xyz (f64)");
    println!("   0.1 + 0.2: {:x}", (xyz.0 + xyz.1).to_bits());
    println!("         0.3: {:x}", (xyz.2).to_bits());
    println!();

    assert!(abc.0 + abc.1 == abc.2);
    assert!((xyz.0 + xyz.1 - xyz.2).abs() < 0.00001);
}

fn two2_1() {
    let guess: i32 = "42".parse().expect("Not a number!");
    print!("guess = {}", guess)
}

// 2.1
fn two1_1() {
    let _x = 5;
    let _y = 10;
}

fn two1_2() {
    let (a, mut b): (bool, bool) = (true, false);
    // a = true,不可变; b = false，可变
    println!("a = {:?}, b = {:?}", a, b);

    b = true;
    assert_eq!(a, b);
}

fn two1_3() {
    let mut x: i32 = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}

struct Struct {
    e: i32,
}

fn two1_4() {
    let (a, b, c, d, e);

    (a, b) = (1, 2);
    // _ 代表匹配一个值，但是我们不关心具体的值是什么，因此没有使用一个变量名而是使用了 _
    [c, .., d, _] = [1, 2, 3, 4, 5];
    Struct { e, .. } = Struct { e: 5 };

    assert_eq!([1, 2, 1, 4, 5], [a, b, c, d, e]);
}

// practive
fn practive() {
    // let x: i32 = 5;
    // let mut y = 5;

    // y = x;

    // let z = 10; // 这里 z 的类型是?

    // let v: u16 = 38_u8 as u16;
    // assert_eq!(v, 38);

    // let x = 5;
    // assert_eq!("i32".to_string(), type_of(&x));

    // // 以下函数可以获取传入参数的类型，并返回类型的字符串形式，例如  "i8", "u8", "i32", "u32"
    // fn type_of<T>(_: &T) -> String {
    //     format!("{}", std::any::type_name::<T>())
    // }

    // assert_eq!(i8::MAX, 127);
    // assert_eq!(u8::MAX, 255);

    // let v1 = 247_u8 + 8;
    // let v2 = i8::checked_add(119, 8).unwrap();
    // println!("{},{}", v1, v2);

    // let x = 1_000.000_1; // ?
    // let y: f32 = 0.12; // f32
    // let z = 0.01_f64; // f64
    // use std::ops::{Range, RangeInclusive};

    // assert_eq!((1..5), Range { start: 1, end: 5 });
    // assert_eq!((1..=5), RangeInclusive::new(1, 5));

    // // 整数加法
    // assert!(1u32 + 2 == 3);

    // // 整数减法
    // assert!(1i32 - 2 == -1);
    // assert!(1i8 - 2 == -1);

    // assert!(3 * 50 == 150);

    // assert!(9 / 3 == 3); // error ! 修改它让代码工作

    // assert!(24 % 5 == 4);

    // // 逻辑与或非操作
    // assert!(true && false == false);
    // assert!(true || false == true);
    // assert!(!true == false);

    // // 位操作
    // println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    // println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    // println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    // println!("1 << 5 is {}", 1u32 << 5);
    // println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // let c1 = '中';
    // print_char(c1);

    // fn print_char(c: char) {
    //     println!("{}", c);
    // }

    // use std::mem::size_of_val;

    // let unit: () = ();
    // assert!(size_of_val(&unit) == 0);

    // println!("Success!");

    // let v = {
    //     let mut x = 1;
    //     x += 1;
    //     x
    // };

    // assert_eq!(v, 3);

    // let x = 3;
    // let v = x;

    // assert!(v == 3);

    // fn plus_or_minus(x: i32) -> i32 {
    //     if x > 5 {
    //         return x - 5;
    //     }

    //     x + 5
    // }

    // let x = plus_or_minus(5);

    // println!("The value of x is: {}", x);
    // let s1 = String::from("hello");
    // let s2 = s1;

    // println!("{}, world!", s1);
    // println!("{}, world!", s2);
    // let s1 = String::from("hello");
    // let s2 = s1.clone();

    // println!("s1 = {}, s2 = {}", s1, s2);
    // let s = String::from("hello"); // s 进入作用域

    // takes_ownership(s); // s 的值移动到函数里 ...
    //                     // ... 所以到这里不再有效

    // println!("在move进函数后继续使用s: {}", s);

    // let x = 5; // x 进入作用域

    // makes_copy(x); // x 应该移动函数里，
    //                // 但 i32 是 Copy 的，所以在后面可继续使用 x

    // let s1 = String::from("hello");

    // let len = calculate_length(&s1);

    // println!("The length of '{}' is {}.", s1, len);
    // 使用尽可能多的方法来通过编译
    // let x = String::from("hello, world");
    // let y = x.clone();
    // println!("{},{}", x, y);
    // let s = give_ownership();
    // println!("{}", s);

    // let s = String::from("hello, world");

    // print_str(s.clone());

    // println!("{}", s);
    // let x = (1, 2, (), "hello");
    // let y = x;
    // println!("{:?}, {:?}", x, y);

    // let s = String::from("hello, ");

    // // 只修改下面这行代码 !
    // let mut s1 = s;

    // s1.push_str("world");

    // println!("{}", s1);

    let x = Box::new(5);

    let mut y = Box::new(3); // 完成该行代码，不要修改其它行！

    *y = 4;

    assert_eq!(*x, 5);
}

// fn takes_ownership(some_string: String) {
//     // some_string 进入作用域
//     println!("{}", some_string);
// } // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

// fn makes_copy(some_integer: i32) {
//     // some_integer 进入作用域
//     println!("{}", some_integer);
// } // 这里，some_integer 移出作用域。不会有特殊操作

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

// 只能修改下面的代码!
// fn give_ownership() -> String {
//     let s = String::from("hello, world");
//     // convert String to Vec
//     // 将 String 转换成 Vec 类型
//     let _s = s.as_bytes();
//     s
// }
// fn print_str(s: String) {
//     println!("{}", s)
// }

fn main() {
    // let mut s = String::from("hello world");

    // let word = first_word(&s);

    // s.clear(); // error!

    // println!("the first word is: {}", word);
    // let mut s = String::from("中国人！");
    // s.insert(3, ',');
    // println!("插入字符 insert() -> {}", s);
    // s.insert_str(4, " I like");
    // println!("插入字符串 insert_str() -> {}", s);
    // let string_replace = String::from("I like rust. Learning rust is my favorite!");
    // let new_string_replace = string_replace.replace("rust", "RUST");
    // dbg!(new_string_replace);
    // dbg!(string_replace);
    // factorial(11);
    // let mut string_replace_range = String::from("I like rust!");
    // string_replace_range.replace_range(7..9, "R");
    // dbg!(string_replace_range);
    // let s1 = String::from("hello,");
    // let s2 = String::from("world!");
    // // 在下句中，s1的所有权被转移走了，因此后面不能再使用s1
    // let s3 = s1 + &s2;
    // assert_eq!(s3, "hello,world!");
    // // 下面的语句如果去掉注释，就会报错
    // println!("{}", s2);
    // let s1 = "hello";
    // let s2 = String::from("rust");
    // let s3 = String::from("rust");
    // let s = format!("{} {}!", s2, s3);
    // println!("{}", s);
    // // 通过 \ + 字符的十六进制表示，转义输出一个字符
    // let byte_escape = "I'm writing \x52\x75\x73\x74!";
    // println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

    // // \u 可以输出一个 unicode 字符
    // let unicode_codepoint = "\u{211D}";
    // let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

    // println!(
    //     "Unicode character {} (U+211D) is called {}",
    //     unicode_codepoint, character_name
    // );

    // // 换行了也字会保持之前的符串格式
    // // 使用\忽略换行符
    // let long_string = "String literals
    //                         can span multiple lines.
    //                         The linebreak and indentation here ->\
    //                         <- can be escaped too!";
    // println!("{}", long_string);

    // println!("{}", "hello \\x52\\x75\\x73\\x74");
    // let raw_str = r"Escapes don't work here: \x3F \u{211D}";
    // println!("{}", raw_str);

    // // 如果字符串包含双引号，可以在开头和结尾加 #
    // let quotes = r#"And then I said: "There is no escape!""#;
    // println!("{}", quotes);

    // // 如果还是有歧义，可以继续增加，没有限制
    // let longer_delimiter = r###"A string with "# in it. And even "##!"###;
    // println!("{}", longer_delimiter);
}
// fn factorial(n: u32) -> u32 {
//     if dbg!(n <= 1) {
//         dbg!(1)
//     } else {
//         dbg!(n * factorial(n - 1))
//     }
// }
// fn first_word(s: &String) -> &str {
//     &s[..1]
// }
// fn main() {
//     let my_name = String::from("Pascal");
//     greet(my_name);
//     let s = String::from("hello world");

//     let hello = &s[0..5];
//     let world = &s[6..11];
//     println!("{} {}", hello, world);

//     let s = "中国人";
//     let a = &s[0..2];
//     println!("{}", a);
// }

// fn greet(name: String) {
//     println!("Hello, {}!", name);
// }

// #![allow(unused_variables)]
// type File = String;
// #[allow(dead_code)]
// fn open(f: &mut File) -> bool {
//     true
// }
// #[allow(dead_code)]
// fn close(f: &mut File) -> bool {
//     true
// }

// #[allow(dead_code)]
// fn read(f: &mut File, save_to: &mut Vec<u8>) -> ! {
//     unimplemented!()
// }

// fn main() {
//     let mut f1 = File::from("f1.txt");
//     // open(&mut f1);
//     read(&mut f1, &mut vec![]);
//     // close(&mut f1);
// }

// main.rs
// mod test; // 首先声明模块
// use crate::test::test; // 使用 use 引入特定项

// fn main() {
// test();
// let a = 3.1 as i8;
// let b = 100_i8 as i32;
// let c = 'a' as u8; // 将字符'a'转换为整数，97

// let c: char = 'z';
// let z = 'ℤ';
// let g = '国';
// let heart_eyed_cat = '😻';

// println!("{},{},{},{}", c, z, g, heart_eyed_cat);
// println!("字符'中'占用了{}字节的内存大小", size_of_val(&z));

// // let x = plus_or_minus(7);

// // println!("The value of x is: {}", x);

// let mut x = 5;
// let y: i32 = x;
// x += 1;
// println!("The value of x,y is: {},{}", x, y);

// let mut b = "hello";
// let c = b;
// b = "world";
// println!("{} {}", b, c);

// let a = &56;
// let a_raw_ptr = a as *const i32;
// let b = &mut 555;
// let b_mut_ptr = b as *mut i32;
// unsafe {
//     println!("{} {}", *a_raw_ptr, *b_mut_ptr);
// }
// }

// fn plus_or_minus(x: i32) -> i32 {
//     if x > 5 {
//         return x - 5;
//     }

//     x + 5
// }
// fn main() {
//     let x = &[1, 2, 4];
//     let mut iterator = x.iter();

//     assert_eq!(iterator.next(), Some(&1));
//     assert_eq!(iterator.next(), Some(&2));
//     assert_eq!(iterator.next(), Some(&4));
//     assert_eq!(iterator.next(), None);

//     // 编译器自动推导出one的类型
//     let one = [1, 2, 3];
//     // 显式类型标注
//     let two: [u8; 3] = [1, 2, 3];
//     let blank1 = [0; 3];
//     let blank2: [u8; 3] = [0; 3];

//     // arrays是一个二维数组，其中每一个元素都是一个数组，元素类型是[u8; 3]
//     let arrays: [[u8; 3]; 4] = [one, two, blank1, blank2];

//     // 借用arrays的元素用作循环中
//     for a in &arrays {
//         print!("{:?}: ", a);
//         // 将a变成一个迭代器，用于循环
//         // 你也可以直接用for n in a {}来进行循环
//         for n in a.iter() {
//             print!("\t{} + 10 = {}", n, n + 10);
//         }

//         let mut sum = 0;
//         // 0..a.len,是一个 Rust 的语法糖，其实就等于一个数组，元素是从0,1,2一直增加到到a.len-1
//         for i in 0..a.len() {
//             sum += a[i];
//         }
//         println!("\t({:?} = {})", a, sum);
//     }
// }
// fn main() {
//     let penguin_data = "\
//     common name,length (cm)
//     Little penguin,33
//     Yellow-eyed penguin,65
//     Fiordland penguin,60
//     Invalid,data
//     ";

//     let records = penguin_data.lines();

//     for (i, record) in records.enumerate() {
//         if i == 0 || record.trim().len() == 0 {
//             continue;
//         }

//         // 声明一个 fields 变量，类型是 Vec
//         // Vec 是 vector 的缩写，是一个可伸缩的集合类型，可以认为是一个动态数组
//         // <_>表示 Vec 中的元素类型由编译器自行推断，在很多场景下，都会帮我们省却不少功夫
//         let fields: Vec<_> = record.split(',').map(|field| field.trim()).collect();
//         if cfg!(debug_assertions) {
//             // 输出到标准错误输出
//             eprintln!("debug: {:?} -> {:?}", record, fields);
//         }

//         let name = fields[0];
//         // 1. 尝试把 fields[1] 的值转换为 f32 类型的浮点数，如果成功，则把 f32 值赋给 length 变量
//         //
//         // 2. if let 是一个匹配表达式，用来从=右边的结果中，匹配出 length 的值：
//         //   1）当=右边的表达式执行成功，则会返回一个 Ok(f32) 的类型，若失败，则会返回一个 Err(e) 类型，if let 的作用就是仅匹配 Ok 也就是成功的情况，如果是错误，就直接忽略
//         //   2）同时 if let 还会做一次解构匹配，通过 Ok(length) 去匹配右边的 Ok(f32)，最终把相应的 f32 值赋给 length
//         //
//         // 3. 当然你也可以忽略成功的情况，用 if let Err(e) = fields[1].parse::<f32>() {...}匹配出错误，然后打印出来，但是没啥卵用
//         if let Ok(length) = fields[1].parse::<f32>() {
//             // 输出到标准输出
//             println!("{}, {}cm", name, length);
//         }
//     }
// }
