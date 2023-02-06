//! 2 move 和 Drop trait
//!

/**
 ### move 语义的本质：初始化变量
```
   let mut a = "42".to_string();
   let b = a;

   // 上面两个表达式的共同作用等价于将a重制为未出始化状态
   // 而不是立即调用drop（析构函数）丢弃
   // 当函数调用结束时才会被都去丢弃
   let mut a: String;

   // 重新给a赋值
   a = "a".to_string();

   // 可以继续使用
   //     println!("{:?}", a);

   ```
### 析构顺序
   ```
   // 析构顺序 案例 1
   // drop： 是Rust自动管理内存的机制，也是智能指针的机制
   struct PrintDrop(&'static str);
   impl Drop for PrintDrop {
       fn drop(&mut self) {
           println!("Dropping {:?}", self.0);
       }
   }

   // 先丢弃y,后丢弃x
   let a = PrintDrop("x");
   let a = PrintDrop("y");

   // 析构顺序 案例 2
   // 元组析构 元组内部按照定义顺序析构
   let tup1 = (PrintDrop("x"), PrintDrop("y"), PrintDrop("z"));
   let tup2 = (PrintDrop("1"), PrintDrop("z"), PrintDrop("3"));

   // 析构顺序 案例 3
   let tup3 = (PrintDrop("a"), PrintDrop("b"), PrintDrop("c"));
   // panic会导致整个tuple最后被析构
   // 内部析构顺序也会变
   // 并且导致后面的析构函数不会调用
   //     let tup4 = (PrintDrop("x"), PrintDrop("y"), panic!());

   // 析构顺序 案例 4
   // 先drop 自身，然后再按照内存布局drop
   struct Foo {
       bar: PrintDrop,
       baz: PrintDrop,
   }

   impl Drop for Foo {
       fn drop(&mut self) {
           println!("Dropping Foo")
       }
   }

   let foo = Foo {
       bar: PrintDrop("bar"),
       baz: PrintDrop("baz"),
   };

   // 析构顺序 案例 5 闭包 捕获未修改
   // 按照捕获顺序
   let z = PrintDrop("z");
   let x = PrintDrop("x");
   let y = PrintDrop("y");

   let closure = move || {
       y;
       z;
       x
   };
   //等价于：生成的匿名结构体
   //所以按照内存布局析构
   struct Closure {
       y: PrintDrop,
       z: PrintDrop,
       x: PrintDrop,
   }

   // 析构顺序 案例 5 闭包 先定义的后析构
   // 先析构子区域的，必须符合安全逻辑

   let z = PrintDrop("z");
   let x = PrintDrop("x");
   let y = PrintDrop("y");

   let closure = move || {
       {
           let z_ref = &z;
       }
       x;
       y;
       z;
   };

   // 使用forget / ManuallyDrop / unsafe rust避免自动drop
   // ManuallyDrop<T>
   // 内存泄漏在Rust中不是不安全，顶多占用内存
   // 弱引用和所有权没关系，不能调用析构函数，使用forget忘掉所有权，就不会调用析构函数
   use std::sync::Arc;
   let arc = Arc::new(PrintDrop("arc"));

   /*
   pub fn new(data: T) -> Arc<T> {
       //Start the weak pointer count as 1 which is the weak pointer that's
       //held by all the strong pointers (kinda), see std/rc.rs for more info
       let x: Box<_> = Box::new(ArcInner {
           strong: atomic::AtomicUsize::new(1),
           weak: atomic::AtomicUsize::new(1),
           data,
       });
       unsafe { Self::from_inner(Box::leak(x).into()) }
   }

    pub const fn leak<'a>(b: Self) -> &'a mut T
   where
       A: 'a,
   {
       unsafe { &mut *mem::ManuallyDrop::new(b).0.as_ptr() }
   }

   pub fn into_raw(self) -> *const T {
       let result = self.as_ptr();
       mem::forget(self);
       result
   }
    */
    ```
*/

pub fn move_and_drop_trait() {}
