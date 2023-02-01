//! 词法结构: 路径
//!
#[allow(unused)]
/**

```
pub fn main() {


    /// 1.模块路径
    ///
    pub mod a {
        fn foo() {
            println!("a")
        }
        pub mod b {
            pub mod c {
                pub fn foo() {
                    super::super::foo();
                    self::super::super::foo();
                }
            }
        }
    }

    a::b::c::foo();


    /// 2.方法调用
    ///
    struct S;

    impl S {
        fn correlation_function(){
            println!("correlation function");
        }
    }

    trait T1 {
        fn method1() {
            println!("method1");
        }
    }

    impl T1 for S {}

    trait T2 {
        fn method2() {
            println!("method2")
        }
    }

    impl T2 for S {}

    // 注意：调用方法有两种情况
    // 两个trait中的方法相同时使用完全限定无歧义调用
    <S as T1>::method1();
    <S as T2>::method2();

    // 其他情况下，调用关联函数和方法的方式相同
    S::correlation_function();
    S::method1();

    /// 3.泛型函数-turbofish操作符
    ///

    // 将0到9收集到Vec中,默认类型是i32，但是可以指定为u64
    let vec0 = (0..10).collect::<Vec<_>>();
    let vec1 = (0..10).collect::<Vec<u64>>();
    println!("{:?}", vec1);

    // 开辟一个容量为1024的u8Vec
    let vec2 = Vec::<u8>::with_capacity(1024);

    println!("{:?}", vec2);
}

```
*/

pub fn path() {
    let vec0 = (0..10).collect::<Vec<_>>();
    let vec1 = (0..10).collect::<Vec<u64>>();
    println!("{:?}", vec1);

    let vec2 = Vec::<u8>::with_capacity(1024);

    println!("{:?}", vec2);
}
