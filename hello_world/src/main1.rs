fn main() {
    let chinese = "世界，你好";
    let english = "hello, world";
    let regions = [chinese, english];

    // regions.iter() 转化为迭代器
    for region in regions.iter() {
        // 宏操作符 是一种特殊类型的函数
        println!("{}", &region);
    }
    println!("Hello, world!");
}
