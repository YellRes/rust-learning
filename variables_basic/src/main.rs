fn main() {
    println!("Hello, world!");
    // let x = 2.0;
    // let y: f32 = 3.0;
    // {
    //     // 数学运算
    //     let twenty = 20;
    //     let twenty_one = 21;
    //     let twenty_two = 22i32;

    //     let addition = twenty + twenty_one + twenty_two;
    //     println!(
    //         "{} + {} + {} = {}",
    //         twenty, twenty_one, twenty_two, addition
    //     );

    //     let one_million: i64 = 1_000_000;
    //     println!("{}", one_million.pow(2));

    //     let forty_twos = [42.0, 42f32, 42.0_f32];
    //     println!("{:.2}", forty_twos[0]);
    // }

    // 位运算
    {
        // 2  3  1  (2^32 - 4) 16 0 16

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
}
