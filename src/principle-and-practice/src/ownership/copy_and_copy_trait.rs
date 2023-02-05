//! 1 Copy 和 Copy Trait
//!
/**
 ### 数据Copy和按位复制
 ### 基础数据类型
```
// 当位置表达式处在值上下文中时，代表把位置表达式当做值传递，默认会发生内存位置的转移
   // 但是其实现了Copy，所以发生了值的复制
   let a = 42;
   let b = a;

   let a: &'static str = "42";

   let a = "42".to_string();
   // &String 实现了 deref trait
   // 可以自动解引用到 &str
   let b = &a;
   let c: &str = &a;

   // 元组是否copy 取决于其内部的元素
```
### 自定义类型

```
 // 元组是否copy 取决于其内部的元素

    // 结构体不会实现Copy，需要手动通过派生宏实现，并且同时需要Clone trait
    #[derive(Debug, Clone, Copy)]
    struct A;

    #[derive(Debug, Clone, Copy)]
    struct Point();

    #[derive(Debug, Clone, Copy)]
    struct Member {
        name: &'static str,
        age: u32,
    }

    // #[derive(Debug, Clone, Copy)]
    struct Person {
        name: String,
        age: u32,
    }

    let a = A; // 零大小类型，编译器会直接优化，相当于占位
    let b = a;
    println!("{:?}", a);

    #[derive(Debug)]
    // 定义一个结构体
    struct B;
    // 再为结构体手动实现 Clone
    impl Clone for B {
        fn clone(&self) -> Self {
            println!("from Custom Copy:Clone");
            *self
        }
    }

    // 先为结构体实现 Copy
    impl Copy for B {}

    let b = B;
    let c = b; // 这里隐式调用 b.clone(),并且是编译器的默认实现
    println!("{:?}", b);

    let d = b.clone(); // 自定义的clone需要显式调用

```
### 按位复制

```
// 按位复制 示例 1
    #[derive(Debug, Clone, Copy)]
    struct A(i8, i32);
    let a = A(1, 2);
    let b = a; // 按位复制，b和a安全相同，包括内存对齐填充的padding部分

    let c = A(a.0, a.1); //逐成员复制，非按位复制，c和a padding的部分不一定相同

    // 按位复制 示例 2
    #[derive(Debug, Copy, Clone)]
    struct B {
        a: u16,
        b: u8,
        c: bool,
    };

    let b = unsound_b();

    let some_b = Some(b);

    println!("b: {:#?}", b);
    println!("some_b: {:#?}", some_b);

    fn unsound_b() -> B {
        #[derive(Debug, Copy, Clone)]
        struct C {
            a: u16,
            b: u8,
            c: u8,
        }

        let c = C { a: 1, b: 1, c: 2 };

        // *mut T 与 *const T 两个原生指针

        // &c 借用
        // 转换：借用先转 C裸指针 再转为B裸指针
        // 1 代表true 0 代表false
        // 通过unsafe rust 按位复制
        unsafe { *(&c as *const C as *const B) }
    }

    // 按位复制 示例 3
    // 使用ptr 以及里面的函数格外注意
    // 裸指针不能drop

    use std::{mem, ptr};

    let mut d = String::from("cccc");
    let d_len = d.len();

    {
        // 在堆上开辟了一块内存，为了在栈上获取指向堆的指针
        let mut c = String::with_capacity(d_len);

        // unsafe {
        //     // 把栈上引用多复制了一份，现在有两个指针指向了同一块堆数据
        //     ptr::copy(&d, &mut c, 1);

        //     //pub const unsafe fn copy<T>(src: *const T, dst: *mut T, count: usize) {}
        // };

        println!("c pointer: {:?}", c.as_ptr());

        // unsafe { ptr::drop_in_place(c.as_mut_ptr()) }
        // mem::drop(c);
    }

    // c 和 d 指向了同一片内存，因为上面指针进行了按位复制
    println!("d pointer: {:?}", d.as_ptr());
    d.push_str("c");
    println!("d: {}", d);

    // 按位复制 示例 4
    // 大多数是栈复制，但也不一定
    // 如下也是按位复制

    use std::cell::RefCell;

    let a = Box::new(RefCell::new(1));
    let b = Box::new(RefCell::new(2));

    *b.borrow_mut() = *a.borrow_mut();

    println!("a = {}", a.borrow_mut());
    println!("b = {}", b.borrow_mut());
```

*/

pub fn copy_and_copy_trait() {}
