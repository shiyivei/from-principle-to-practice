use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::process;

// 重构 1

//自定义错误结果,把所有实现了Error trait的都转为 trait 对象
type ParseResult<i32> = Result<i32, Box<dyn Error>>;

fn run(filename: &str) -> ParseResult<i32> {
    File::open(filename)
        .map_err(|e| e.into())
        .and_then(|mut f| {
            let mut contents = String::new();
            f.read_to_string(&mut contents)
                .map_err(|e| e.into())
                .map(|_| contents)
        })
        .and_then(|contents| {
            let mut sum = 0;
            for c in contents.lines() {
                match c.parse::<i32>() {
                    Ok(n) => sum += n,
                    Err(err) => {
                        let err: Box<dyn Error> = err.into();
                        println!("error info: {},cause: {:?}", err.to_string(), err.source())
                    }
                }
            }
            Ok(sum)
        })
}

fn main() {
    // 读取文件内容
    // 本地文件直接可以使用env传参
    let args: Vec<String> = env::args().collect();
    println!("args: {:?}", args);

    let filename = &args[1];
    println!("in file: {}", filename);
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
