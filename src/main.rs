// main.rs
mod test; // 首先声明模块
use test::test; // 使用 use 引入特定项

fn main() {
    test();
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
