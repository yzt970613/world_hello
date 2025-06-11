fn main() {
    // let a = 3.1 as i8;
    // let b = 100_i8 as i32;
    // let c = 'a' as u8; // 将字符'a'转换为整数，97

    let c: char = 'z';
    let z = 'ℤ';
    let g = '国';
    let heart_eyed_cat = '😻';

    println!("{},{},{},{}", c, z, g, heart_eyed_cat);
    println!("字符'中'占用了{}字节的内存大小", size_of_val(&z));

    // let x = plus_or_minus(7);

    // println!("The value of x is: {}", x);

    let mut x = 5;
    let y: i32 = x;
    x += 1;
    println!("The value of x,y is: {},{}", x, y);

    let mut b = "hello";
    let c = b;
    b = "world";
    println!("{} {}", b, c);

    let a = &56;
    let a_raw_ptr = a as *const i32;
    let b = &mut 555;
    let b_mut_ptr = b as *mut i32;
    unsafe {
        println!("{} {}", *a_raw_ptr, *b_mut_ptr);
    }
}

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
