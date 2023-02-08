use std::env;

use std::fmt;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::num;
use std::process;
use thiserror::Error;
// use std::error::Error;
use anyhow::{Context, Result};

// 重构 5 使用第三方库thiserror结合anyhow 简化代码

// 自定义错误类型
// ? 操作符号是match的语法糖
// 如果不是error，会展开result拿结果，如果是，返回err

// 使用宏
#[derive(Error, Debug)]
enum CliError {
    #[error("{0}")] // display
    Io(#[from] io::Error),
    #[error("{0}")]
    Parse(#[from] num::ParseIntError), // from
}

// type ParseResult<i32> = Result<i32, CliError>;

fn run(filename: &str) -> Result<i32, anyhow::Error> {
    let mut file = File::open(filename).context(format!("unable to open '{}' ", filename))?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .context(format!("unable to read string '{}' ", filename))?;

    let mut sum = 0;
    for c in contents.lines() {
        // println!("{:?}", c);
        let n: i32 = c.parse::<i32>()?;
        sum += n;
    }
    Ok(sum)
}

fn main() -> Result<(), anyhow::Error> {
    // 读取文件内容
    // 本地文件直接可以使用env传参
    let args: Vec<String> = env::args().collect();
    println!("args: {:?}", args);

    let filename = &args[1];
    println!("in file: {}", filename);
    // println!("in file: {}", filename);
    let err = run(filename)?;
    println!("{:?}", err);
    Ok(())
}
