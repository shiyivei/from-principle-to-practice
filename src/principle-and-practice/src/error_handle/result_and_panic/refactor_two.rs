use std::env;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::num;
use std::process;

// 重构 2

// 自定义错误类型
// ? 操作符号是match的语法糖
// 如果不是error，会展开result拿结果，如果是，返回err

#[derive(Debug)]
enum CliError {
    Io(io::Error),
    Parse(num::ParseIntError),
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CliError::Io(ref err) => write!(f, "IO error: {}", err),
            CliError::Parse(ref err) => write!(f, "Parse error: {}", err),
        }
    }
}

impl Error for CliError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            CliError::Io(ref err) => Some(err),
            CliError::Parse(ref err) => Some(err),
        }
    }
}

impl From<io::Error> for CliError {
    fn from(err: io::Error) -> CliError {
        CliError::Io(err)
    }
}

impl From<num::ParseIntError> for CliError {
    fn from(err: num::ParseIntError) -> CliError {
        CliError::Parse(err)
    }
}

type ParseResult<i32> = Result<i32, CliError>;

fn run(filename: Option<String>) -> ParseResult<i32> {
    let mut file = File::open(filename.unwrap())?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut sum = 0;

    for c in contents.lines() {
        let n: i32 = c.parse::<i32>()?;
        sum += n;
    }
    Ok(sum)
}

fn main() {
    // 读取文件内容
    // 本地文件直接可以使用env传参

    let filename = env::args().nth(1);
    // println!("in file: {}", filename);
    match run(filename) {
        Ok(n) => {
            println!("{:?}", n);
        }
        Err(e) => {
            println!("main error: {}", e);
            process::exit(1);
        }
    }
}
