//! trait对象
//!
/**
 ###
 ```
 // trait 作为泛型限定
    use std::string::ToString;

    fn print<T: ToString>(v: T) {
        println!("{}", v.to_string());
    }

    let c = 'a';
    let s = "hello world";

    print::<char>(c);
    print::<&'static str>(s);

    // 静态分发：impl trait

    use std::fmt::Display;

    // 返回一个实现了 Display trait 的类型
    fn make_value<T: Display>(index: usize) -> impl Display {
        match index {
            0 => "Hello,World",
            1 => "Hello,world (1)",
            _ => panic!(),
        }
    }

    println!("{}", make_value::<&'static str>(0));
    println!("{}", make_value::<&'static str>(1));

    // trait 与生命周期
    //     fn make_debug<T>(_: T) -> impl std::fmt::Debug {
    //         42u8
    //     }

    // late bound
    fn make_debug<'a, T: 'static>(_: &'a T) -> impl std::fmt::Debug {
        42u8
    }

    fn test() -> impl std::fmt::Debug {
        let value = "value".to_string();
        make_debug(&value)
    }

    use core::any::{Any, TypeId};
    use std::sync::Arc;

    // 模拟类
    // 类的实例相当于trait 对象
    struct Class {
        name: String,
        type_id: TypeId,
    }

    impl Class {
        fn new<T: 'static>() -> Self {
            Class {
                name: std::any::type_name::<T>().to_string(),
                type_id: TypeId::of::<T>(),
            }
        }
    }

    struct Instance {
        inner: Arc<dyn Any>, //相当于 Box<T>
    }

    impl Instance {
        fn new(obj: impl Any) -> Self {
            Self {
                inner: Arc::new(obj),
            }
        }

        fn instance_of(&self, class: &Class) -> bool {
            self.inner.as_ref().type_id() == class.type_id
        }
    }

    struct Foo {};
    struct Bar {};

    let foo_class = Class::new::<Foo>();
    let bar_class = Class::new::<Bar>();

    let foo_instance = Instance::new(Foo {});

    assert!(foo_instance.instance_of(&foo_class));
    assert!(!foo_instance.instance_of(&bar_class));

 ```
 ### trait 对象的本质


```
// 不仅能够绕过对象安全规则，还可以提升性能
trait StarkFamily {
        fn last_name(&self) -> &'static str;
        fn totem(&self) -> &'static str;
    }

    trait TullyFamily {
        fn territory(&self) -> &'static str;
    }

    trait Children {
        fn new(first_name: &'static str) -> Self
        where
            Self: Sized;

        fn first_name(&self) -> &'static str;
    }

    impl StarkFamily for dyn Children {
        fn last_name(&self) -> &'static str {
            "Stark"
        }

        fn totem(&self) -> &'static str {
            "Wolf"
        }
    }

    impl TullyFamily for dyn Children {
        fn territory(&self) -> &'static str {
            "Riverrun City"
        }
    }

    struct People {
        first_name: &'static str,
    }

    impl Children for People {
        fn new(first_name: &'static str) -> Self
        where
            Self: Sized,
        {
            println!("hello,{:?} Stark", first_name);
            People {
                first_name: first_name,
            }
        }
        fn first_name(&self) -> &'static str {
            self.first_name
        }
    }

    fn fully_name(person: Box<dyn Children>) {
        println!(
            "--- Winter is coming, the lone {:?} dies, the packs lives ---",
            person.totem()
        );

        let full = format!("{} {}", person.first_name(), person.last_name());
        println!("I'm {:?}", full);

        println!("My mother come from {:?}", person.territory());
    }

    let sansa = People::new("Sansa");
    let aray = People::new("Aray");

    let starks = Box::new(sansa);
    fully_name(starks);

    let starks = Box::new(aray);
    fully_name(starks);


    //另一种实现方法
    use core::ops::Add;
    // 类型不同，行为相同，通过trait实现
    trait KnobControl<T: Add + Add<Output = T> + Copy> {
        fn set_position(&mut self, value: T);
        fn get_value(&self, p: T) -> T;
    }

    struct LinearKnob<T: Add + Add<Output = T> + Copy> {
        position: T,
    }

    struct LogarithmicKnob<T: Add + Add<Output = T> + Copy> {
        position: T,
    }

    impl<T: Add + Add<Output = T> + Copy> KnobControl<T> for LinearKnob<T> {
        fn set_position(&mut self, value: T) {
            self.position = value
        }
        fn get_value(&self, p: T) -> T {
            self.position
        }
    }

    impl<T: Add + Add<Output = T> + Copy> KnobControl<T> for LogarithmicKnob<T> {
        fn set_position(&mut self, value: T) {
            self.position = value
        }

        fn get_value(&self, p: T) -> T {
            self.position + p
        }
    }

    // 通过enum实现
    // 将类型抽象到枚举体中

    enum Knob<T: Add + Add<Output = T> + Copy> {
        Linear(LinearKnob<T>),
        Logarithmic(LogarithmicKnob<T>),
    }

    impl<T: Add + Add<Output = T> + Copy> KnobControl<T> for Knob<T> {
        fn set_position(&mut self, value: T) {
            match self {
                Knob::Linear(inner_knob) => inner_knob.set_position(value),
                Knob::Logarithmic(inner_knob) => inner_knob.set_position(value),
            }
        }

        fn get_value(&self, value: T) -> T {
            match self {
                Knob::Linear(inner_knob) => inner_knob.get_value(value),
                Knob::Logarithmic(inner_knob) => inner_knob.get_value(value),
            }
        }
    }

```

### 使用枚举代替 trait对象

```

    // 类型不同，行为相同，通过trait实现
    trait KnobControl {
        fn set_position(&mut self, value: f64);
        fn get_value(&self) -> f64;
    }

    struct LinearKnob {
        position: f64,
    }

    struct LogarithmicKnob {
        position: f64,
    }

    impl KnobControl for LinearKnob {
        fn set_position(&mut self, value: f64) {
            self.position = value
        }
        fn get_value(&self) -> f64 {
            self.position
        }
    }

    impl KnobControl for LogarithmicKnob {
        fn set_position(&mut self, value: f64) {
            self.position = value
        }

        fn get_value(&self) -> f64 {
            (self.position + 1.).log2()
        }
    }

    // 通过enum实现
    // 将类型抽象到枚举体中

    enum Knob {
        Linear(LinearKnob),
        Logarithmic(LogarithmicKnob),
    }

    impl KnobControl for Knob {
        fn set_position(&mut self, value: f64) {
            match self {
                Knob::Linear(inner_knob) => inner_knob.set_position(value),
                Knob::Logarithmic(inner_knob) => inner_knob.set_position(value),
            }
        }

        fn get_value(&self) -> f64 {
            match self {
                Knob::Linear(inner_knob) => inner_knob.get_value(),
                Knob::Logarithmic(inner_knob) => inner_knob.get_value(),
            }
        }
    }

```
### 使用trait对象重载函数
```

 // 不能将trait中的方法为相同的类型做不同实现
    // 一个能实现此目标的方案

    trait Blanket<T> {
        fn blanket(&self) -> &'static str;
    }

    impl Blanket<u8> for u8 {
        fn blanket(&self) -> &'static str {
            "u8"
        }
    }

    // trait对象
    impl<T: ToString> Blanket<&dyn ToString> for T {
        fn blanket(&self) -> &'static str {
            "ToString"
        }
    }

    trait CloneBlanket {}

    impl<T: Clone> Blanket<&dyn CloneBlanket> for T {
        fn blanket(&self) -> &'static str {
            "Clone"
        }
    }

    trait TryIntoBlanket<T> {
        type Error;
    }

    impl<T, E, U> Blanket<&dyn TryIntoBlanket<T, Error = E>> for U
    where
        U: TryInto<T, Error = E>,
    {
        fn blanket(&self) -> &'static str {
            "to_string"
        }
    }

    impl<T: AsRef<U>, U: ?Sized> Blanket<&dyn AsRef<U>> for T {
        fn blanket(&self) -> &'static str {
            "as_ref"
        }
    }
    ```
### trait 对象与 Self：Sized
    ```
     // trait 中有默认实现时
    // 并且默认实现的函数体中包含Self
    trait WithConstructor {
        fn build(param: usize) -> Self
        where
            Self: Sized;
        fn new(param: usize) -> Self
        where
            Self: Sized,
        {
            Self::build(0)
        }

        fn t(&self);
    }

    struct A;

    impl WithConstructor for A {
        fn t(&self) {
            println!("hello");
        }
        fn build(param: usize) -> Self
        where
            Self: Sized,
        {
            A
        }
    }

    let a = &A;
    a.t()

    ```
     trait Test {
        fn foo(&self);

        fn works(self: Box<Self>) {
            println!("hello");
        }

        fn fails(self: Box<Self>)
        // where
        //     Self: Sized, //限定了被调用,关闭；？Sized 在类型声明时使用
        {
            self.foo();
        }
    }

    struct Concrete;

    impl Concrete {
        fn hello(&self) {
            println!("hello");
        }
    }

    impl Test for Concrete {
        fn foo(&self) {
            ()
        }
        fn works(self: Box<Self>) {
            self.hello();
        }
        // 没有实现fails
    }

    let concrete: Box<dyn Test> = Box::new(Concrete);
    // concrete.fails();
    concrete.works();

    ```



*/

pub fn trait_object() {}
