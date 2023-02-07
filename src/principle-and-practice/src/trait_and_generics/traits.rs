//! 什么是trait
//!
/**
 ### trait 的几种用途
 ```
 // trait 作为泛型限定
    use std::string::ToString;

    fn print<T: ToString>(v: T) {
        println!("{}", v.to_string());
    }

    let c = 'a';
    let s = "hello world";

    print::<char>(c);
    print::<&'static str>(s);

    // 静态分发：impl trait

    use std::fmt::Display;

    // 返回一个实现了 Display trait 的类型
    fn make_value<T: Display>(index: usize) -> impl Display {
        match index {
            0 => "Hello,World",
            1 => "Hello,world (1)",
            _ => panic!(),
        }
    }

    println!("{}", make_value::<&'static str>(0));
    println!("{}", make_value::<&'static str>(1));

    // trait 与生命周期
    //     fn make_debug<T>(_: T) -> impl std::fmt::Debug {
    //         42u8
    //     }

    // late bound
    fn make_debug<'a, T: 'static>(_: &'a T) -> impl std::fmt::Debug {
        42u8
    }

    fn test() -> impl std::fmt::Debug {
        let value = "value".to_string();
        make_debug(&value)
    }

 ```

*/

pub fn traits() {}
