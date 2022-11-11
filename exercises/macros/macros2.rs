// macros2.rs
// Execute `rustlings hint macros2` or use the `hint` watch subcommand for a hint.


fn main() {
    my_macro!();
}
// 由于宏是在定义前被调用，所以需要使用#[macro_export]将宏引入作用域
#[macro_export]
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}
