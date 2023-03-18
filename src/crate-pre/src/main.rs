fn main() {
    // 1 函数签名中显式声明参数类型
    fn sum(a: u32, b: u32) -> u32 {
        a + b
    }
    sum(32, 32);

    let a = 32_u64;
    // sum(a, 32); // a 是u64 类型，不满足函数约定

    // 2 使用Option枚举来包裹可能为空的类型，方便后续处理

    let v = vec![Some(1), Some(2), Some(3), Some(4), None];

    let mut v_iter = v.iter();

    loop {
        if let Some(v) = v_iter.next() {
            match v {
                Some(v) => println!("{}", v),
                None => break,
            }
        }
    }

    // 3 使用Result处理错误,自定义错误类型

    use std::error::Error;
    use std::fmt;
    use std::fs::File;
    use std::io::prelude::*;

    /*

    #[derive(Debug)]
    enum MyError {
        OpenFileError(String),
        ParseFileError(String),
        ReadFileError(String),
        // ...
    }

    impl Error for MyError {}

    impl fmt::Display for MyError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                MyError::OpenFileError(s) => write!(f, "{}", s),
                MyError::ParseFileError(s) => write!(f, "{}", s),
                MyError::ReadFileError(s) => write!(f, "{}", s),
                // ...
            }
        }
    }
    */

    // 使用宏实现 自定义的错误类型

    use thiserror::Error;
    #[derive(Error, Debug)] // Error 宏
    #[non_exhaustive]
    pub enum MyError {
        #[error("{0}")] // display 属性宏
        OpenFileError(String),
        #[error("{0}")] // display 属性宏
        ParseFileError(String),
        #[error("{0}")] // display 属性宏
        ReadFileError(String),
    }

    fn get_content(path: &str) -> Result<String, MyError> {
        let file = File::open(path);

        match file {
            Ok(mut file) => {
                let mut string = String::new();

                file.read_to_string(&mut string).unwrap();

                println!("{:?}", string);

                Ok(".".to_string())
            }
            Err(_) => Err(MyError::OpenFileError("failed to open file".to_string())),
        }
    }

    // 这里故意填了未创建的文件名，一定会出错
    // if let Err(e) = get_content("filename.text") {
    //     println!("{:?}", e) // OpenFileError("failed to open file") 打印出的错误类型
    // }

    // 4 恐慌和断言

    match get_content("filename.text") {
        Ok(filename) => {
            assert!(filename == "".to_string()); // 断言判断返回值是否为空字符串
        }
        Err(e) => panic!("{:?}", e), // 返回错误时程序奔溃
    }

    // 5 真实环境中的 自定义Error类型

    /*

    #[derive(Debug, Error)]
    pub enum KvError {
        #[error("Not found for table: {0},key: {1}")]
        NotFound(String, String),
        #[error("Cannot parse command: `{0}`")]
        InvalidCommand(String),
        #[error("Cannot convert value {:0} to {1}")]
        ConvertError(Value, &'static str),
        #[error("Cannot process command {0} with table: {1}, key: {2}. Error: {}")]
        StorageError(&'static str, String, String, String),

        #[error("I/O error")]
        IoError(#[from] std::io::Error),

        //使用第三发库的具体Error类型
        #[error("Failed to encode protobuf message")]
        EncodeError(#[from] prost::EncodeError),
        #[error("Failed to decode protobuf message")]
        DecodeError(#[from] prost::DecodeError),

        #[error("Internal error: {0}")]
        Internal(String),

        #[error("Invalid command error")]
        FmtError(#[from] std::fmt::Error),

        #[error("frame error")]
        FrameError,
        #[error("Failed to access sled db")]
        SledError(#[from] sled::Error),

        // #[error("I/O error")]
        // IoError(#[from] std::io::Error),
        #[error("certificate parse error server: {0}, cert: {1}")]
        CertificateParseError(&'static str, &'static str),

        #[error("TLS error")]
        TlsError(#[from] tokio_rustls::rustls::TLSError),

        // #[error("Yamux Connection error")]
        // YamuxConnectionError(#[from] yamux::ConnectionError),
        #[error("Parse config error")]
        ConfigError(#[from] toml::de::Error),
    }

    */
}
