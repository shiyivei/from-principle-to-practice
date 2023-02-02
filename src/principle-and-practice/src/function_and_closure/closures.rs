//! 闭包与trait
//!

/**
 ### 闭包与函数及闭包实现原理
 ### 1 闭包与函数

 ```
// 函数无法捕获环境变量
    fn counter(i: i32) -> impl FnMut(i32) -> i32 {
        // 1. 闭包与所有权
        // 闭包使用move关键字把环境变量所有权转移到闭包内
        // 具体执行copy还是move语义需要看具体的类型
        let s1 = "hello".to_string();
        move |s2: &str| s1 + s2;
        // println!("{:?}", s1); // 不可用,move语义

        // 2. 闭包类型与函数指针类型
        // 某闭包类型:|i32| -> i32,同函数指针非常相似
        // 某函数指针类型: fn(i32) -> i32

        // 3. 闭包与函数指针互通 (闭包作为参数),只有未捕获环境变量的闭包才可以

        type RGB = (i32, i32, i32);
        fn show(c: fn(&str) -> RGB) {
            println!("{:?}", c("black"));
        }

        // 定义闭包：类型｜&str｜ -> (i32,i32,i32),实现了 `Fn(&str)-> RGB` trait
        let c = |s: &str| (1, 2, 3);
        show(c);

        // 4. 闭包作为返回值
        // 因为闭包是基于Trait实现的，所以闭包作为返回值时使用的是impl trait语法
        // 返回值是i32 trait的类型，其中 FnMut(i32)->i32 这一整块作为一个trait
        // impl FnMut(i32) -> i32 代表返回的是一个实现了FnMut(i32)
        let closure = move |n| n + i;
        closure
    }

    let mut f = counter(21);
    assert_eq!(42, f(21))

 ```
 ### 2 闭包实现原理
 ```
// 请将下列模块属性放置在执行文件顶部
#![feature(unboxed_closures, fn_traits)]
 // 按使用场景

    // 1. 未捕捉环境变量 对应所有权
    let c1 = || println!("hello");
    c1();

    // 等价于创建了一个闭包结构体，并未闭包结构体实现了 call_once方法
    // 对闭包的调用实际上是对相应trait中的方法进行调用,但使用的名字不同,类似在使用函数项一样
    // 注意call_once方法的第一个参数是self,代表它会消耗结构体,需要拥有所有权

    struct Closure1<T> {
        env_var: T,
    }

    /*
       ### 标准库 FnOnce trait的定义
       pub trait FnOnce<Args>
       where
       Args:Tuple, {
           type Output;
           extern "rust-call" fn call_once(mut self, args: Args) -> Self::Output;
       }
    */

    // 为类型实现trait
    impl<T> FnOnce<()> for Closure1<T> {
        type Output = ();
        extern "rust-call" fn call_once(self, args: ()) -> () {
            println!("hello");
        }
    }

    // 调用

    let c1 = Closure1 { env_var: () };
    c1.call_once(());

    // 2. 可修改环境变量 对应可变借用 &mut T
    let mut arr = [1, 2, 3];
    let mut c2 = |i| {
        arr[0] = i;
        println!("{:?}", arr)
    };

    c2(100);

    // 等价于
    // 继承式的实现实际上是所有权一致性的体现
    // 闭包实例至少需要一个消耗自身的方法

    struct Closure2 {
        env_var: [i32; 3],
    }

    /*
       ### 标准库 FnOnce trait的定义
       pub trait FnOnce<Args> {
           type Output;
           extern "rust-call" fn call_once(mut self, args: Args) -> Self::Output;
       }
    */

    // 为类型实现 FnOnce trait
    impl FnOnce<(i32,)> for Closure2 {
        type Output = ();
        extern "rust-call" fn call_once(mut self, args: (i32,)) -> () {
            self.env_var[0] = args.0;
            println!("{:?}", self.env_var);
        }
    }

    /*
       ### 标准库 FnMut trait的定义
        pub trait FnMut<Args>:FnOnce<Args> {
        where
        Args:Tuple, {
            extern "rust-call" fn call_mut(&mut self, args: Args) -> Self::Output;
        }
    */

    // 为类型实现 FnMut trait
    impl FnMut<(i32,)> for Closure2 {
        extern "rust-call" fn call_mut(&mut self, args: (i32,)) -> () {
            self.env_var[0] = args.0;
            println!("{:?}", self.env_var);
        }
    }

    // 调用

    let arr2 = [1, 2, 3];
    let mut c2 = Closure2 { env_var: arr2 };
    c2.call_mut((0,)); //可变引用调用
    c2.call_once((1,)); //消耗式调用



    // 3. 未修改环境变量 对应不可变借用 &T
    let answer = 42;
    let c3 = || {
        println!("{:?}", answer);
    };

    // 等价于

    struct Closure3 {
        env_var: i32,
    }

    /*
       ### 标准库 FnOnce trait的定义
       pub trait FnOnce<Args>
       where
       Args:Tuple, {
           type Output;
           extern "rust-call" fn call_once(mut self, args: Args) -> Self::Output;
       }
    */

    // 为类型实现 FnOnce trait
    impl FnOnce<()> for Closure3 {
        type Output = ();
        extern "rust-call" fn call_once(mut self, args: ()) -> () {
            println!("{:?}", self.env_var);
        }
    }

    /*
       ### 标准库 FnMut trait的定义
       pub trait FnMut<Args>:FnOnce<Args> {
       where
       Args:Tuple, {
           extern "rust-call" fn call_mut(&mut self, args: Args) -> Self::Output;
       }
    */

    // 为类型实现 FnMut trait
    impl FnMut<()> for Closure3 {
        extern "rust-call" fn call_mut(&mut self, args: ()) -> () {
            println!("{:?}", self.env_var);
        }
    }

    /*
       ### 标准库 Fn trait的定义
       pub trait Fn<Args>:FnMut<Args>
       where
        Args:Tuple, {
           extern "rust-call" fn call(&self, args: Args) -> Self::Output;
       }
    */

    impl Fn<()> for Closure3 {
        extern "rust-call" fn call(&self, args: ()) -> () {
            println!("{:?}", self.env_var);
        }
    }

    let mut c3 = Closure3 { env_var: 42 };
    c3.call(()); // 不可变引用
    c3.call_mut(()); //可变引用
    c3.call_once(()) //消耗式调用

 ```
 ### 3 逃逸和非逃逸闭包
 ```
 // 逃逸闭包
    fn c_mut() -> impl FnMut(i32) -> [i32; 3] {
        let mut arr = [1, 2, 5];
        move |n| {
            arr[2] = n;
            arr
        }
    }

    let i = 42;

    let mut arr_closure = c_mut();
    println!("{:?}", arr_closure(i));

    // 被捕获类型不支持Copy,无法返回闭包，主要是为了防止悬垂引用

    /*
    fn c_mut2() -> impl for<'a> FnMut(&'a str) -> String {
        // 当闭包捕获了未实现Copy trait 的类型时，无法返回
        let mut s = "hello".to_string();
        move |i| {
            s += i;
            s
        }
    }
    */

 ```
 ### 唯一不可变借用（这个说法很有趣，本质上就是可变借用和不可变借用不能同存）


 ```
     // 深刻理解闭包是如何捕获变量的
     // 把可变变量作为一个整体引用给了不可变变量
  let mut a = [1, 2, 3];
    let x = &mut a;

    {
        let mut c = || {
            // 闭包使用了可变借用

            (*x)[0] = 0;
        };

        // let y = &x; // 不能再有不变借用
        c();
    }
    let z = &x;

 ```
 ### 4 闭包实现了哪些trait

 ```
  // 闭包自身实现了Fn Copy trait
    fn foo<F: Fn() + Copy>(f: F) {
        f()
    }

    let s = "hello".to_owned();

    // 不可变借用
    let f = || {
        println!("{}", s);
    };
    foo(f);

    // 消耗
    let g = move || {
        println!("{}", s);
    };

    //foo(g); // 未实现copy trait

```

 */
pub fn closure() {}
