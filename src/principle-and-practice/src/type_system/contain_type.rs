//! 容器类型
//!

/**
### 容器Cell、RefCell、UnsafeCell
### 1. 容器Cell: 通过移进移出值来实现内部可变性
```
   use std::cell::Cell;
   struct Foo {
       x: u32,
       y: Cell<u32>,    // 包裹实现了copy trait的类型
       z: Cell<String>, // 包裹未实现copy trait的类型
   }

   // 初始化一个不可变实例
   let foo = Foo {
       x: 1,
       y: Cell::new(3),
       z: Cell::new("hello".to_string()),
   };

   assert_eq!(1, foo.x);
   assert_eq!(3, foo.y.get());
   // 没有实现Copy的类型无法使用get方法获取内部值,可以看到Cell容器是通过移进移出值来实现内部可变性的
   // assert_eq!("hello".to_string(), foo.z.get());

   // 改变不可变实例
   foo.y.set(100);
   println!("y: {:?}", foo.y.get());
   foo.z.set("world".to_string());
   // 未实现copy的类型不可以使用get获取,但是可以使用into_inner获取
   println!("z: {:?}", foo.z.into_inner());
   // 实现了copy的类型既可以使用get获取,也可以使用into_inner获取
   println!("y: {:?}", foo.y.into_inner());
```
### 2. 容器RefCell: 通过borrow_mut实现可变性
// 主要是应用于一些未实现copy trait类型，通过borrow获取值，有运行时开销
```
 use std::cell::RefCell;
    // 使用vec！宏创建不可变的动态可增长数组
    let vec = vec![1, 2, 3, 4];
    // vec.push(5); // 不能往不可变的数组中增加元素

    let ref_vec = RefCell::new(vec); //包裹变长数组
    println!("{:?}", ref_vec.borrow()); // 不可变借用打印
    ref_vec.borrow_mut().push(5); // 可变借用改变
    println!("{:?}", ref_vec.borrow()) // 不可变借用打印
```
### 3. 容器UnsafeCell 是上述两种容器的底层实现
*/
pub fn contain_type() {}
