struct Struct {
    g: i32,
}

fn main() {
    // mut 关键字表示这是个可变变量
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    // println!("Hello, world!");

    // 在变量前面加上_  让rust编译器忽略这个没有使用过的变量
    let _y = 12;

    /* 解构赋值 */
    let (a, mut b): (bool, bool) = (true, true);
    println!("a = {:?}, b = {:?}", a, b);

    b = true;
    assert_eq!(a, b);

    // 使用元祖、切片和结构体模式
    let (c, d, e, f, g);
    (c, d) = (1, 2);
    [e, .., f, _] = [1, 2, 3, 4, 5];
    Struct { g, .. } = Struct { g: 5 };

    // TODO: cargo run 之后 assert_eq 没有输出
    assert_eq!([1, 2, 1, 4, 5], [c, d, e, f, g]);

    // 常量使用const来定义
    const _MAX_LEN: u32 = 100_000;

    // 变量遮蔽
    print!("\x1b[2J");
    {
        let x = 5;
        let x = x + 1;
        {
            let x = x * 2;
            println!("the value of x in the inner scope is: {}", x);
        }
        println!("The value of x is: {}", x);
    }
}
