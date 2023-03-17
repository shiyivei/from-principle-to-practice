use std::{collections::HashMap, slice::Iter, slice::IterMut};

use std::vec::IntoIter;

fn main() {
    // 1 标准库中的trait
    // pub trait Iterator {
    //     type Item;
    //     fn next(&mut self) -> Option<Self::Item>;
    //     //  + 74 methods
    // }

    // pub trait IntoIterator {
    //     type Item;
    //     type IntoIter: Iterator<Item = Self::Item>;

    //     fn into_iter(self) -> Self::IntoIter;
    // }

    // 特别说明：for 循环以及解糖

    let values = vec![1, 2, 3, 4, 5];
    // 使用 for 循环遍历集合中个每个元素
    for x in values {
        println!("{x}");
    }

    // for 循环解糖后等价如下：
    let v = vec![1, 2, 3, 4, 5];

    // 先将集合转为外部迭代器
    let mut v_iter = v.into_iter();

    // 在 loop 循环中使用next方法循环获取集合中下一个元素，当集合中取不到值时使用break终止 loop循环
    loop {
        match v_iter.next() {
            Some(x) => println!("{}", x),
            None => break,
        }
    }

    // 2 迭代器的使用

    let map = HashMap::from([("rust", 1), ("go", 2), ("python", 3)]);
    let map_iter = map.into_iter();
    let vec: Vec<(&str, i32)> = map_iter.collect();
    println!("{:?}", vec); // [("rust", 1), ("go", 2), ("python", 3)]

    // 3 迭代器、借用和所有权
    let mut v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let iter_mut: IterMut<i32> = v.iter_mut(); // 转为  IterMut 结构体, 可变借用
    let iter: Iter<i32> = v.iter(); // 转为 Iter 结构体， 不可变借用
    let iter_into: IntoIter<i32> = v.into_iter(); // 转为 IntoIter 结构体 ， 获取所有权

    // 4 迭代器适配器
    let vec = vec![1, 2, 3, 4, 5];
    let doubled: Vec<i32> = vec
        .iter()
        .map(|&x| x * 3)
        .take(3)
        .filter(|x| *x > 6)
        .collect();
    println!("{:?}", doubled); // [9]

    // 5 迭代器与迭代器适配器特性：lazy（惰性）

    let v = vec![1, 2, 3, 4, 5];
    v.iter().for_each(|x| println!("{x}"));
    // or
    for x in &v {
        println!("{x}");
    }

    // 一、指针、引用和智能指针

    // 1 引用

    let x = 100;
    let mut y: i64 = 200;
    #[derive(Debug)]
    struct A(i32);
    let a = A(100);

    // 使用 & 获取不变或者可变引用
    let x_pointer = &x;
    let y_pointer = &mut y;
    let a_pointer = &a;

    println!("{:?}", x_pointer); // 100 打印时会自动“解引用”到数据，而不是地址
    println!("{:p}", x_pointer); // 0x7ff7b9bae33c 如果要打印地址的话，改变占位符？为 p

    // let z = &mut y; // 可变借用不能超过1次

    y = *y_pointer + 100; // 解引用后修改

    println!("{:?}", y); //300 本条代码结束后，可变借用才释放
    println!("{:?}", a_pointer); // A(100)

    // 2 裸指针
    let x = 100;
    let mut y: i64 = 200;
    struct B(i32);
    let a = B(100);

    // 裸指针是使用 as *const 从引用转换而来
    let x_raw_pointer = &x as *const i32;
    let y_raw_pointer = &mut y as *const i64;
    let a_raw_pointer = &a as *const B;

    println!("{:?}", x_raw_pointer); // 0x7ff7b763a46c，裸指针打印时不会被“解引用”到数据，而是会直接会打印地址

    unsafe {
        y = *y_raw_pointer + 300; // 裸指针解引用需要使用unsafe 语法块，这里的解引用的安全的

        let z_raw_pointer = &mut y as *const i64; // 第二次生成可变裸指针，unsafe 块绕过了可变借用的次数规则，是不是感觉有点危险？

        y = *z_raw_pointer + 500; // 然后继续改变数据

        println!("{:?}", *y_raw_pointer); // 1000
    }
    println!("{:?}", a_raw_pointer); // 0x7ff7b763a47c
    println!("{:?}", y); // 1000

    // 3.1 智能指针 与 引用

    // Vec 和 String 类型都是智能指针，没想到吧？
    let vec = vec![1, 2, 3, 4];
    let s = "rust".to_string();
    let num = Box::new(100);

    let v1 = vec; // 发生了move语义，现在数据的所有者不再是vec 而是v1，数据没变，拥有者变了

    // println!("{:?}", vec); // 不能再使用 vec，因为它不再拥有数据了

    let v = [1, 2, 3, 4];
    let v = &v1; // 只是借用，v 仍然拥有数据
    println!("{:?}", v); // 所以可以使用 v

    // 3.2 智能智能与结构体、trait

    /*
    pub struct Box<T, A = Global>(_, _)
    where
        A: Allocator,
        T: ?Sized;

    pub struct String {
        vec: Vec<u8>,
    }

    pub struct Vec<T, A: Allocator = Global> {
        buf: RawVec<T, A>,
        len: usize,
    }

    pub struct Rc<T: ?Sized> {
        ptr: NonNull<RcBox<T>>,
        phantom: PhantomData<RcBox<T>>,
    }
    */

    pub trait Deref {
        type Target: ?Sized;

        fn deref(&self) -> &Self::Target;
    }

    pub trait Drop {
        fn drop(&mut self);
    }

    //二 智能指针的使用

    // 2.1 Drop trait

    /*
    unsafe impl<#[may_dangle] T: ?Sized, A: Allocator> Drop for Box<T, A> {
        fn drop(&mut self) {
            // FIXME: Do nothing, drop is currently performed by compiler.
        }
    }
    */

    let mut user = User {
        name: "rust".to_string(),
        age: 12,
    };

    // drop(user);
    // println!("{:?}", user) // 不能再打印，值已经被释放了

    fn main() {
        // 初始化变量,但后面不做调用
        let mut user = User {
            name: "rust".to_string(),
            age: 12,
        };

        // user.drop(); //手动调用也行 因为编译器会自动调用，显式调用二者会冲突

        // 你会在终端发现打印了 “Rust”，成功验证，编译器确实调用了 drop
    }

    // 2.2 Deref trait

    let m = MyBox::new("rust");
    let ref_my_box = *m; // 实现了 Deref trait的智能指针可以使用 * 直接解引用

    /*
    impl ops::Deref for String {
        type Target = str;

        #[inline]
        fn deref(&self) -> &str {
            unsafe { str::from_utf8_unchecked(&self.vec) }
        }
    }

    */

    fn take_ref_string(s: &str) {
        println!("{:?}", s)
    }

    // 将String解引用为str
    // 注意：String这个智能指针包裹的类型是 str，解引用后大小编译器无法确定，所以要再加&（引用）
    take_ref_string(&*"Rust".to_string())
}

// 2.1 Drop trait
#[derive(Debug)]
struct User {
    name: String,
    age: u32,
}

// 我们手动为类型实现了 drop trait
impl Drop for User {
    fn drop(&mut self) {
        println!("{:?}", "rust") // 实现细节只是做了打印
    }
}

// 2.2 Deref trait

use std::ops::Deref;

#[derive(Debug)]
struct MyBox<T>(T);
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> MyBox<T> {
    fn new(t: T) -> Self {
        Self(t)
    }
}
