//! 生命周期参数
//!

/**
#### 生命周期参数
 ```
 // 示例 1: 声明和使用
    fn return_str<'a>() -> &'a str {
        let mut s = "Rust".to_string();

        for i in 0..3 {
            s.push_str("Good ");
        }

        //    &s[..] // 不能返回局部变量，会造成悬垂指针
        "hello"
    }

    let x = return_str();
```
#### 早限定与晚限定
```
    // 示例 2 生命周期参数的之间的关系

    // 当多个参数都是借用类型，需要指定这些参数生命周期参数之间的关系
    // 指定规则:入参声明周期 >= 返回值生命周期
    // 'a: 'c 代表 'a > 'c

    fn the_longest<'c, 'a: 'c>(s1: &'a str, s2: &'a str) -> &'c str {
        if s1.len() > s2.len() {
            s1
        } else {
            s2
        }
    }

    let s1 = String::from("rust");
    let s1_r = &s1;
    {
        let s2 = String::from("c");
        let res = the_longest(s1_r, &s2);
        println!("{:?}", res);
    }

    // 示例 3 Rust如何识别并判断生命周期参数的？

    // Rust 实际上使用两套规则：early bound 和 late bound
    // 核心是匹配参数和指定的生命周期参数

    // 示例 4 early bound 和 late bound

    // 先看看泛型案例
    struct A<T>(T);

    // 定义时不指定类型，实际使用时再指定
    let a = A::<i32>(3);
    let b = A("string");


    // early bound 和 late bound 区别

    fn f<'a>() {} // late bound,只声明，不指定。
    fn g<'a: 'a>() {} // early bound，声明且指定
    fn h<T>() {}

    //let pf = f::<'static> as fn(); // late bound 不能手动指定，编译器会自动生成具体的生命周期参数实例
    let pf = f(); // late bound 直接使用，不用指定

    // 声明周期参数实际上是泛型的一种
    let ph = h::<i32>(); // 普通泛型可以指定，会在编译期单态化
    let pg = g::<'static> as fn(); // early bound 可以具体再指定

    // 案例 4.1 late bound

    struct Buffer {
        buf: Vec<u8>,
        pos: usize,
    }

    // 没有在实现方法时指定在impl处
    // 导致的结果时在调用函数时，编译器会检查local 变量
    // 但是使用early bound，编译器只会检查参数类型是否正确以及和生命周期是否匹配

    impl Buffer {
        fn new() -> Buffer {
            Buffer {
                buf: vec![1, 2, 3, 4, 5, 6],
                pos: 0,
            }
        }

        // late bound: 声明和使用
        fn read_bytes<'a>(&'a mut self) -> &'a [u8] {
            self.pos += 3;
            &self.buf[self.pos - 3..self.pos]
        }
    }

    fn print(b1: &[u8], b2: &[u8]) {
        println!("{:#?} {:#?}", b1, b2);
    }


    let mut buf = Buffer::new();
    let b1 = buf.read_bytes(); // work

    // late bound 带来的后果
    // 编译器会检查重复可变引用

    //     let b2 = buf.read_bytes(); // 不能同时有两个借用
    //     let b3 = &(buf.read_bytes().to_owned());
    //     print(b1,b2)


    // 案例 4.2 late bound 改写 early bound
    // early bound 提高了生命周期，本质上编译器检查的是两个变量
    // 所以并没有违反借用规则

    struct Buffer1<'a> {
        buf: &'a [u8],
        pos: usize,
    }

    impl<'a> Buffer1<'a> {

        fn new(b: &'a [u8]) -> Buffer1 {
            Buffer1 { buf: b, pos: 0 }
        }

        fn read_bytes(&mut self) -> &'a [u8] {
            self.pos += 3;
            &self.buf[self.pos - 3..self.pos]
        }
    }

    let v = vec![1, 2, 3, 4, 5, 6];

    let mut buf = Buffer1::new(&v);
    let b1 = buf.read_bytes(); // work
    let b2 = buf.read_bytes(); //

    print(b1, b2);

    // 与如下同理
    let mut s = String::from("hello");
    let a = s.push_str(" jian");
    let b = s.push_str(" quan");
 ```
 ### &T和T

 ```
  trait Trait {
        fn f(self);
    }
    impl<T> Trait for fn(T) { // 此处的T指值或者引用
        fn f(self) {
            print!("1")
        }
    }
    impl<T> Trait for fn(&T) { // &T 指引用
        fn f(self) {
            print!("2")
        }
    }

    let a: fn(_) = |_: u8| {};
    let b: fn(_) = |_: &u8| {};
    let c: fn(&_) = |_: &u8| {}; //实例化

    // 编译器会根据地函数参数以及制定的类型来调用
    a.f(); // 1
    b.f(); // 1
    c.f(); // 2

    // T 与 &T，后者是前者的子集
    // 但是可以为后者实现特殊的方法重载
    trait Trait {
        fn f(self);
    }
    impl<T> Trait for fn(T) {
        fn f(self) {
            print!("1")
        }
    }

    //     impl<'a,T> Trait for fn(&'a T) {
    impl<T> Trait for fn(&T) {
        // 此处会生命和使用默认的生命周期参数，不可以手动显式声明
        // 因为参数是引用类型
        fn f(self) {
            print!("2")
        }
    }

    let a: fn(_) = |_: u8| {};
    let b: fn(_) = |_: &u8| {}; // 编译器会为参数默认加上生命周期参数x，同 &'x u8
    let c: fn(&_) = |_: &u8| {}; //在此处会实例化，也是一个late bound的case

    a.f(); // 1
    b.f(); // 1
    c.f(); // 2

    // early bound 还体现在函数第二个（第n个）参数的调用上
    //case 2

    use std::collections::HashSet;
    let hello = "hello".to_owned();
    let mut items: HashSet<&str> = HashSet::new();

    items.insert(hello.as_str());

    let mut global_set: HashSet<&str> = HashSet::new();
    global_set.insert(hello.as_str());

    while !global_set.is_empty() {
        let mut temp_set: HashSet<&str> = HashSet::new();

        for &item in global_set.iter() {
            let copy = item.to_owned();
            let copy_str = copy.as_str(); // &self

            // 此处不能使用get(&copy_str),如果再借用一次，当离开copy的作用域时，仍然使用
            // 再借用就判断符合上下文
            // 不加编译器只会判断get函数
            if let Some(inner) = items.get(copy_str).cloned() {
                temp_set.insert(inner);
            }
        }

        std::mem::swap(&mut global_set, &mut temp_set);
        break;
    }
 ```
 ### trait 对象的生命周期参数

 ```
 struct FooImpl<'a> {
        s: &'a [u32],
    }
    impl<'a> Foo<'a> for FooImpl<'a> {}

    // trait 对象必须使用 Box包裹
    // 任何实现了 某个trait的类型，它的实例都是 trait对象
    // trait 对象默认为静态生命周期，当作为返回值时，需要手动“缩短”（指定生命周期参数，如‘a）

    // fn foo<'a, 'b: 'a>(s: &'a [u32]) -> Box<dyn Foo<'a> + 'a> { //第一种写法
    fn foo<'a>(s: &'a [u32]) -> Box<dyn Foo<'a> + 'a> {
        // 第二种写法
        Box::new(FooImpl { s: s })
    }

 ```
 ### 高阶生命周期参数

 ```
    // 1.使用高阶生命参数 for语法
    use std::fmt::Debug;
    trait DosSomething<T> {
        fn do_something(&self, value: T);
    }

    impl<'a, T: Debug> DosSomething<T> for &'a usize {
        fn do_something(&self, value: T) {
            println!("{:?}", value);
        }
    }

    // 高阶生命周期，高阶限定，for语法，是一种late bound
    //     fn foo<'a>(b: Box<dyn DosSomething<&'a usize>>) { 改动前
    fn foo<'a>(b: Box<dyn for<'f> DosSomething<&'f usize>>) {
        // 不在当前作用域判断
        let s: usize = 10;
        b.do_something(&s) // 在do something 函数作用域判断
    }

    let x = Box::new(&2usize);
    foo(x)

    // 2.高阶生命参数在闭包中的用法

    // 不能通过高阶语法来修复
    // let f = |x: &i32| x;
    // 如：
    // let f: for<'a> Fn(&'a i32) -> &'a i32 = |x| x;
    // fn foo<'a>(b: Box<dyn for<'f> DosSomething<&'f usize>>) {

    // 修复方法 1：通过外部函数 late bound
    // 函数返回F,F为实现了 Fn trait的闭包，并且是late bound
    fn annotate1<T, F>(f: F) -> F
    where
        for<'a> F: Fn(&'a T) -> &'a T,
    {
        f
    }

    // 修复方法2：通过外部函数 early bound
    fn annotate2<'a, T: 'a, F>(f: F) -> F
    where
        F: Fn(&'a T) -> &'a T,
    {
        f
    }

    // 不管怎么样，都是告诉编译器该怎么判断

    let f = annotate1(|x: &i32| x);
    let f = annotate2(|x: &i32| x);
    let i = &3;
    let j = f(i); // 本身是为了使用这个闭包，在这里才会检查x

    // 3.高阶生命周期参数 trait应用

    // trait 对象高阶生命参数 for 在多态调用中的应用
    use rand;
    use std::io::Read;

    trait CheckSum<R: Read> {
        fn calc(&mut self, r: R) -> Vec<u8>;
    }

    struct Xor;

    impl<R: Read> CheckSum<R> for Xor {
        fn calc(&mut self, mut r: R) -> Vec<u8> {
            let mut res: u8 = 0;
            let mut buf = [0u8; 8];

            loop {
                let read = r.read(&mut buf).unwrap();
                if read == 0 {
                    break;
                }

                for b in &buf[..read] {
                    res ^= b;
                }
            }
            vec![res]
        }
    }

    struct Add;

    impl<R: Read> CheckSum<R> for Add {
        fn calc(&mut self, mut r: R) -> Vec<u8> {
            let mut res: u8 = 0;
            let mut buf = [0u8; 8];

            loop {
                let read = r.read(&mut buf).unwrap();
                if read == 0 {
                    break;
                }
                // late bound
                for b in &buf[..read] {
                    let tmp = res as u16 + *b as u16;
                    res = tmp as u8;
                }
            }
            vec![res]
        }
    }

    let mut buf = [0u8; 128];

    // late bound
    let mut checker: Box<dyn for<'f> CheckSum<&'f [u8]>> = if rand::random() {
        println!("Initializing Xor Checksum");
        Box::new(Xor)
    } else {
        println!("Initializing Add Checksum");
        Box::new(Add)
    };

    let mut data = "data.read(&mut buf).unwrap()".as_bytes();
    let mut i = 0;

    loop {
        let chunk_size = data.read(&mut buf).unwrap();
        if chunk_size == 0 {
            break;
        }
        let cs = checker.calc(&buf[..chunk_size]);
        println!("CheckSum {} is {:?}", i, cs);
        i += 1;
    }

 ```
*/

pub fn lifetime() {}
