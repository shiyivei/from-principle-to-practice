//! Rust中的trait
//!

/**

```

 // 一、trait 是什么

   // 1. 没有方法的trait,只起约束作用
   pub trait Copy: Clone {} // 大多存放在栈上的类型都实现了 Copy trait

   // 2. 有方法，但未实现的 trait
   pub trait From<T>: Sized {
       // From trait 常用于做类型转换 比如从 Vec<u8> 转到 String
       fn from(value: T) -> Self;
   }
   // 3. 有实现的方法，也有未实现方法的 trait
   // 并且有关联类型，当方法的返回值是一个泛型时，可将关联类型作为返回值类型
   // pub trait Iterator {
   //     // Iterator trait主要用在集合类型上，实现了该trait的集合可以迭代获取下一个元素等
   //     type Item; // 关联类型

   //     // 定义的未实现的方法
   //     fn next(&mut self) -> Option<Self::Item>;

   //     // 默认实现的方法 如map 、 collect等
   //     fn map<B, F>(self, f: F) -> Map<Self, F>
   //     where
   //         Self: Sized,
   //         F: FnMut(Self::Item) -> B,
   //     {
   //         Map::new(self, f)
   //     }

   //     fn collect<B: FromIterator<Self::Item>>(self) -> B
   //     where
   //         Self: Sized,
   //     {
   //         FromIterator::from_iter(self)
   //     }

   //     // 还有其他 73种 默认实现已经实现了的方法
   // }

   // 二、 trait 怎么用

   // 2.1.1 通过 派生宏来实现（直接放在数据结构上）
   use serde::{Deserialize, Serialize};

   #[derive(Eq, PartialEq, Debug, Serialize, Deserialize, Default)]
   pub struct Customer {
       id: u64,
       age: Option<u32>,
       name: Option<String>,
       gender: Option<Gender>,
   }

   #[derive(Eq, PartialEq, Debug, Serialize, Deserialize)]
   pub enum Gender {
       Male,
       Women,
       Other,
   }

   // 使用trait 提供的方法

   let c = Customer {
       age: Some(26),
       ..Default::default()
   };

   let c_string = serde_json::to_string(&c).unwrap();

   println!("c to string {:?}", c_string); // "{\"age\":26,\"name\":null,\"gender\":null}"

   let customer: Customer = serde_json::from_str(&c_string).unwrap();

   println!("string to Customer {:?}", customer); // Customer { age: Some(26), name: None, gender: None }

   // 使用函数实现
   use rand::{thread_rng, Rng}; // id 随机生成
   fn new_customer(name: String) -> Customer {
       Customer {
           id: thread_rng().gen::<u64>(),
           name: Some(name),
           ..Default::default()
       }
   }

   // 调用函数
   println!("customer {:?}", new_customer("rust".to_string()));

   // 2.1.2 通过实现 Into<Customer> trait 实现数据转换

   impl Into<Customer> for String {
       fn into(self) -> Customer {
           Customer {
               id: thread_rng().gen::<u64>(),
               name: Some(self),
               ..Default::default()
           }
       }
   }

   let new_customer: Customer = "rust".to_string().into();

   println!("new customer {:?}", new_customer);

   // 2.2 trait 作为约束
   // 同一种约束的不同写法，bound_func1 等价于 bound_func2
   trait Consume {}

   fn bound_func1(c: impl Consume) {
       // do something
   }

   fn bound_func2<T: Consume>(c: T) {}

   // 2.3 trait 对象
   impl Consume for Customer {}
   // 使用Box是因为编译时大小未知
   fn use_trait_object(c: Customer) -> Box<dyn Consume> {
       Box::new(c)
   }
   // 目前不支持在函数返回值中使用 impl trait
   // fn use_trait_object(c: Customer) -> impl Consume {
   //     c
   // }

```
*/

pub fn traits() {}
