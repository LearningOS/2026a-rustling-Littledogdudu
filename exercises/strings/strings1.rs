// strings1.rs
//
// Make me compile without changing the function signature!
//
// Execute `rustlings hint strings1` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {}", answer);
}

fn current_favorite_color() -> &'static str {
    // 字符串常量值本身就放在可执行程序的只读数据段，所以直接修改返回类型 String 为 &'static str
    // 如果使用 "blue".to_string()，比这种方式多一次堆内存分配和数据复制
    "blue"
}

fn current_favorite_color_example() -> String {
    // 这两种互相等价，都会把"blue"复制到堆上，即产生一次堆内存分配和数据复制
    "blue".to_string()
    // String::from("blue")
}
