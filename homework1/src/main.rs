mod level1;
mod level2;

fn main() {
    println!("一层子模块，循环打印从’a’~’Z’ 之间的所有字符（采用老模块风格）");
    level1::print();

    println!("二层子模块，循环打印从’A’~’z’ 之间的所有字符");
    level2::printer::print();
}
