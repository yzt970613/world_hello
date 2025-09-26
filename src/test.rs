// pub fn test() {
//     // 通过 \ + 字符的十六进制表示，转义输出一个字符
//     let byte_escape = "I'm writing \x52\x75\x73\x74!";
//     println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

//     // \u 可以输出一个 unicode 字符
//     let unicode_codepoint = "\u{211D}";
//     let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

//     println!(
//         "Unicode character {} (U+211D) is called {}",
//         unicode_codepoint, character_name
//     );

//     // 换行了也会保持之前的字符串格式
//     // 使用\忽略换行符
//     let long_string = "String literals
//                         can span multiple lines.
//                         The linebreak and indentation here ->\
//                         <- can be escaped too!";
//     println!("{}", long_string);
// }
// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }

// pub fn test() {
//     let user1 = User {
//         email: String::from("someone@example.com"),
//         username: String::from("someusername123"),
//         active: true,
//         sign_in_count: 1,
//     };
//     let _user2 = User {
//         active: user1.active,
//         username: user1.username,
//         email: String::from("another@example.com"),
//         sign_in_count: user1.sign_in_count,
//     };
//     println!("{}", user1.active);
//     println!("{}", user1.email);
//     // println!("{}", user1.username);
//     // 下面这行会报错
//     // println!("{:?}", user1);
// }

pub fn test() {
    let test1: Vec<i32> = Vec::new();
    println!("test1: {:?}", test1);
}
