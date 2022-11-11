// move_semantics2.rs
// Make me compile without changing line 13 or moving line 10!
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand for a hint.

fn main() {

// 方法一：不把vec0传给vec2
//   let vec0 = fill_vec(Vec::new());
//   let vec2 = Vec::new();
//   let mut vec1 = fill_vec(vec2);
//
// 方法二：不变引用，直接把vec0传递给函数，然后copy
    let vec0 = Vec::new();
    let mut vec1 = fill_vec(&vec0);
//
// 方法三：可变引用，1.把vec0变成可被引用，把可变引用传给函数.
//   let mut vec0 = Vec::new();
//   let mut vec1 = fill_vec(&mut vec0);
    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

// 方法一：
// fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
//    let mut vec = vec;
// 方法二：
  fn fill_vec(vec: &Vec<i32>) -> Vec<i32> {
      let mut vec = vec.to_vec();
// 方法三：
//fn fill_vec(vec: &mut Vec<i32>) -> Vec<i32> {
//    let mut vec = vec;
//
    vec.push(22);
    vec.push(44);
    vec.push(66);
// 方法一：
// vec
// 方法二：
    vec
// 方法三：
// vec.to_vec()
}
