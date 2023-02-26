use serde::Deserialize;
use std::borrow::Cow;

fn main() {
    /// 1. 用于类型导入
    ///
    // 模块 a (注意：模块以及模块中的类型通过 pub 来控制其可见性)
    pub mod a {
        // 模块 a 中的函数 a()
        fn a() {
            println!("a")
        }

        // 在模块 a 中定义子模块 b
        pub mod b {

            // 在模块 b 中定义子模块 c
            pub mod c {

                // 在模块内部访问上层
                pub fn c() {
                    // super 代表当前模块的父模块
                    super::super::a();
                    // self 代表当前模块
                    self::super::super::a();
                    println!("c")
                }
            }
        }
    }

    // 在模块的外部层层向内访问
    a::b::c::c();

    /// 2.用于方法调用
    ///
    // 定义类型：元组结构体
    struct S(u64);

    // 定义两个 trait，并且trait中有相同的方法，都是默认实现的
    trait T1 {
        fn same_method() {
            println!("same");
        }
    }
    // 第二个trait中有个方法不是默认实现
    trait T2 {
        fn same_method() {
            println!("same")
        }
        fn to_string(self) -> String;
    }

    // 为类型实现 trait
    impl T1 for S {}

    impl T2 for S {
        fn to_string(self) -> String {
            self.0.to_string()
        }
    }

    // 注意：调用方法有两种情况,使用 S::same_method()进行调用时编译器并不知道是哪个trait中的方法
    // 两个trait中的方法相同时使用完全限定无歧义调用，即 <Type as Trait>::method_name
    <S as T1>::same_method();
    <S as T2>::same_method();

    // 而不同方法名可以直接调用，并不会产生歧义
    let s = S(42u64);
    s.to_string();

    // 关联函数的实现和调用
    impl S {
        fn relation_function() {
            println!("correlation function");
        }
    }

    // 类型名 + ::
    S::relation_function();

    /// 3.用于类型指定 ::<type>
    ///
    let nums = (0..9);
    // 将元组中的元素收集到Vec中,默认类型是i32，但是可以指定为u64
    let vec_i32 = (0..10).collect::<Vec<_>>();
    let vec_u64 = (0..10).collect::<Vec<u64>>();

    println!("{:?},{:?}", vec_i32, vec_i32);

    // 开辟一个容量为1024的String Vec 和 u8 Vec
    let vec_string = Vec::<String>::with_capacity(1024);
    let vec_u8 = Vec::<u8>::with_capacity(1024);

    println!("{:?},{:?}", vec_u64, vec_string);
}
