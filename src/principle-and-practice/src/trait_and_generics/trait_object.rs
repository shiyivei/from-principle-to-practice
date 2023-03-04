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

### trait object 三种应用场景


```
use std::future::Future;
use std::pin::Pin;
use std::{fmt::Write, io::Read, marker::PhantomData};

// 1. 在函数中的使用
use std::{error::Error, process::Command};

// 在rust中，我们如果想要在函数中返回一个实现了某个trait的值,无法使用 impl trait 语法，只能使用 trait对象 dyn trait （但是参数两种语法都可以使用）
pub type BoxedError = Box<dyn Error + Send + Sync>; // 分配在堆上
pub type E = dyn Error + Send + Sync; // 分配在栈上

// 返回值是一个trait对象
pub trait Executor {
    fn run(&self) -> Result<Option<i32>, BoxedError>;
}

// 带有泛型生命周期参数的类型
pub struct Shell<'a, 'b> {
    cmd: &'a str,
    args: &'b [&'a str],
}

// 实现方法
impl<'a, 'b> Shell<'a, 'b> {
    pub fn new(cmd: &'a str, args: &'b [&'a str]) -> Self {
        Self { cmd, args }
    }
}

// 实现trait

impl<'a, 'b> Executor for Shell<'a, 'b> {
    fn run(&self) -> Result<Option<i32>, BoxedError> {
        let output = Command::new(self.cmd).args(self.args).output()?;
        Ok(output.status.code())
    }
}

// 在函数返回值中使用 trait object
pub fn execute_generics(cmd: &impl Executor) -> Result<Option<i32>, BoxedError> {
    cmd.run()
}
// 在函数参数和返回值中使用 trait object
pub fn execute_trait_object(cmd: &dyn Executor) -> Result<Option<i32>, BoxedError> {
    cmd.run()
}
// 在函数参数和返回值中使用 trait object
pub fn execute_boxed_trait_object(cmd: Box<dyn Executor>) -> Result<Option<i32>, BoxedError> {
    cmd.run()
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]
    fn shell_should_work() {
        let cmd = Shell::new("ls", &[]);
        let result = cmd.run().unwrap();

        assert_eq!(result, Some(0))
    }

    #[test]
    fn execute_shell_should_work() {
        let cmd = Shell::new("ls", &[]);
        let result = execute_generics(&cmd).unwrap();
        assert_eq!(result, Some(0));

        let result = execute_trait_object(&cmd).unwrap();
        assert_eq!(result, Some(0));

        let result = execute_boxed_trait_object(Box::new(cmd)).unwrap();
        assert_eq!(result, Some(0));
    }
}
pub struct HeaderValue;
pub struct Url {
    url: String,
}

pub trait CookiesStore: Send + Sync {
    fn set_cookies(&self, cookie_headers: &mut dyn Iterator<Item = &HeaderValue>, url: &Url);
    fn cookies(&self, url: &Url) -> Option<HeaderValue>;
}

// 2. 返回值中使用

pub struct Kvpair {
    key: String,
    value: String,
}

pub enum KvError {
    InvalidKey,
}

pub trait Storage: Send + Sync + 'static {
    /// 遍历 HashTable，返回 kv pair 的 Iterator
    fn get_iter(&self, table: &str) -> Result<Box<dyn Iterator<Item = Kvpair>>, KvError>;
}

// 目前，Rust并不支持在trait中使用 async fn，所以使用async_trait 宏来增加这个功能
use async_trait;
#[async_trait]
pub trait Fetch {
    type Error;
    async fn fetch(&self) -> Result<String, Self::Error>;
}

// 宏展开相当于
pub trait Fetch {
    type Error;
    fn fetch<'a>(
        &'a self,
    ) -> Result<Pin<Box<dyn Future<Output = String> + Send + 'a>>, Self::Error>;
}
// 它使用了 trait object 作为返回值
// 这样，不管 fetch() 的实现，返回什么样的 Future 类型
// 都可以被 trait object 统一起来
// 调用者只需要按照正常 Future 的接口使用即可

pub trait CryptoResolver {
    fn resolve_rng(&self) -> Option<Box<dyn Random>>;
    fn resolve_dh(&self, choice: &HDChoice) -> Option<Box<dyn Dh>>;
    fn resolve_hash(&self, choice: &HashChoice) -> Option<Box<dyn Hash>>;
    fn resolve_cipher(&self, choice: &CipherChoice) -> Option<Box<dyn Cipher>>;
    #[cfg(futures = "hfs")]
    fn resolve_kem(&self, choice: &KemChoice) -> Option<Box<dyn Kem>> {
        None
    }
}

pub fn generate_keypair(&self) -> Result<Keypair, Error> {
    // 拿到当前的随机数生成算法
    let mut rng = self.resolver.resolve_rng().ok_or(InitStage::GetRngImpl)?;
    // 拿到当前的 DH 算法
    let mut dh = self
        .resolver
        .resolve_dh(&self.params.dh)
        .ok_or(InitStage::GetDhImpl)?;
    let mut private = vec![0u8; dh.priv_len()];
    let mut public = vec![0u8; dh.pub_len()];
    // 使用随机数生成器 和 DH 生成密钥对
    dh.generate(&mut *rng);

    private.copy_from_slice(dh.privkey());
    public.copy_from_slice(dh.pubkey());

    Ok(Keypair { private, public })
}

// 3. 在数据结构中使用 trait object
pub struct HandShakeState<A, B, C, D>
where
    A: Add,
    B: Borrow,
    C: Copy,
    D: Drop, {}

type HandShakeStateAlias = dyn Add + Borrow + Copy + Drop;

// 4. 在闭包中的使用

pub struct Input<'a, T> {
    prompt: String,
    default: Option<T>,
    show_default: bool,
    initial_text: Option<String>,
    theme: &'a dyn Theme,
    permit_empty: bool,
    validator: Option<Box<dyn FnMut(&T) -> Option<String> + 'a>>,
    #[cfg(feature = "history")]
    history: Option<&'a mut dyn History<T>>,
}

```
*/

pub fn trait_object() {}
