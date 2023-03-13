//! 闭包与trait
//!

/**
 ### 闭包与函数及闭包实现原理
 ### 1 闭包与函数

 ```

 // 一、闭包的定义

    // 1.未捕获环境变量
    let add_one = |x: i32| x + 1; // 等价于 fn add_one(x:i32) -> i32 { x + 1 }
    let result = add_one(2); // 调用时二者相同

    // 2.捕获Copy类型的值
    let x = 10;
    let add_x = |y: i32| x + y; // 此处捕获了x，这种情况下，没有对应的函数
    let result = add_x(2); // result = 12

    // 3.使用move移动所有权
    let x = vec![1, 2, 3];
    let print_x = move || println!("{:?}", x);

    // 下面的代码将无法编译，因为 x 已经被移动到闭包内
    // println!("{:?}", x);

    // 二 闭包与函数指针
    let closure = |x: i32| x + 1; // 闭包类型：|i32| -> i32
    fn func(x: i32) -> i32 {
        // 函数指针类型： fn(i32) -> i32
        x + 1
    }

    fn show(c: fn(i32) -> i32) {
        c(100);
    }

    show(closure); // 二者都可以作为参数
    show(func);

    // 三 闭包类型与 trait

    /*  pub trait FnOnce<Args>
    where
        Args: Tuple,
    {
        type Output;
        extern "rust-call" fn call_once(mut self, args: Args) -> Self::Output;
    }

    // ### 标准库 FnOnce trait的定义
     pub trait FnOnce<Args> {
        type Output;
        extern "rust-call" fn call_once(mut self, args: Args) -> Self::Output;
    }

    // ### 标准库 FnMut trait的定义
    pub trait FnMut<Args>:FnOnce<Args> {
    where
    Args:Tuple, {
        extern "rust-call" fn call_mut(&mut self, args: Args) -> Self::Output;
    }

    // ### 标准库 Fn trait的定义
    pub trait Fn<Args>:FnMut<Args>
    where
     Args:Tuple, {
        extern "rust-call" fn call(&self, args: Args) -> Self::Output;
    }
    */

    let answer = 42;
    // 声明一个闭包,未捕获环境变量
    let c3 = || {
        println!("{:?}", "rust");
    };

    // 等价于生成一个匿名空结构体，然后为结构体实现三个trait
    struct Closure3;

    /*// 为类型实现 FnOnce trait
    impl FnOnce<()> for Closure3 {
        type Output = ();
        extern "rust-call" fn call_once(mut self, args: ()) -> () {
            println!("{:?}", "rust");
        }
    }

    //为类型实现 FnMut trait
    impl FnMut<()> for Closure3 {
        extern "rust-call" fn call_mut(&mut self, args: ()) -> () {
            println!("{:?}", "rust");
        }
    }

    // 为类型实现 Fn trait
    impl Fn<()> for Closure3 {
        extern "rust-call" fn call(&self, args: ()) -> () {
            println!("{:?}", "rust");
        }
    }

    let mut c3 = Closure3;
    c3.call(()); // 不可变引用
    c3.call_mut(()); //可变引用
    c3.call_once(()) //获取所有权

    */

    // 四 闭包与函数返回值

    fn _can_return_closure(_x: i32) -> impl FnMut(i32) -> i32 {
        let a = 42;
        // 注意：这里要将闭包作为函数的返回值，所以不能再使用引用变量（否则会发生悬垂引用）
        // 所以要使用 move 将变量的所有权转移到闭包内
        // let c =  |x: i32| x + a;

        let c = move |x: i32| x + a;

        c
    }

    fn _cannot_return_closure(_x: i32) -> impl FnOnce(String) -> String {
        let a = "42".to_string();
        // 依旧是转移所有权
        // 但是因为a是动态大小类型，所以在被闭包补获时，默认转移所有权

        let c = |x: String| {
            let y = a + x.as_str(); // 此处 a 自动发生了move语义
            y
        };

        // let c = move |x: String| a + x.as_str(); // 但也可以使用 move 显式说明将环境变量的所有权移入闭包

        c
    }

    fn _return_closure(_x: i32) -> impl Fn(String) -> String {
        // 没有捕获环境变量
        let c = |x: String| {
            let a = "42".to_string();
            a + x.as_str()
        };
        c
    }

```

 */
pub fn closure() {}
