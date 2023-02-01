//! 泛型
//!
/**
 ### 泛型函数
 ```
 fn foo<T>(x: T) -> T {
    x
}
fn main() {
    assert_eq!(foo(1), 1);
    assert_eq!(foo("hello"), "hello");

    // 上述的函数会单态化为两个不同参数类型的函数
    fn foo_1(x: i32) -> i32 {
        x
    }
    fn foo_2(x: &'static str) -> &'static str {
        x
    }

    foo_1(1);
    foo_2("2");

    // Rust根据上下文有一定的推断能力，但是推断不出来时需要手工通过turbofish指定

    // foo(1) 等价于 foo::<i32>(1);
    // foo("hello") 等价于 foo::<&'static str>("hello");
}
```
 */

pub fn generic() {}
