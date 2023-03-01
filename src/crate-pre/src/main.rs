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

pub fn main() {}
