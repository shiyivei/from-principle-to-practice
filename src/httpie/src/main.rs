use anyhow::{anyhow, Ok, Result};
use clap::Parser;
use colored::Colorize;

use mime::Mime;
use reqwest::{header, Client, Response, Url};
use std::{collections::HashMap, io::Error, str::FromStr};
use syntect::{
    easy::HighlightLines,
    highlighting::{Style, ThemeSet},
    parsing::SyntaxSet,
    util::{as_24_bit_terminal_escaped, LinesWithEndings},
};
use tokio;

mod tests;

// 操作命令,包括一系列子命令
#[derive(Parser, Debug)]
#[clap(version = "1.0", author = "shiyivei <shiyivei@outlook.com>")]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

// 子命令，使用枚举

#[derive(Parser, Debug)]
enum SubCommand {
    Get(Get),
    Post(Post),
}

// 每个命令详细情况
#[derive(Debug, Parser)]
struct Get {
    // 不再通过属性宏的方式指定工具函数
    // #[(parse(try_from_str = parse_url))]
    url: String,
}

#[derive(Debug, Parser)]
struct Post {
    // 不再通过属性宏的方式指定工具函数,自动选择
    // #[clap(parse(try_from_str = parse_url))]
    url: String,
    // 不再通过属性宏的方式指定工具函数
    // #[clap(parse(try_from_str = parse_kv_pair))]
    body: Vec<KvPair>,
}

// 类型嵌套

#[derive(Debug, PartialEq, Clone)]
struct KvPair {
    k: String,
    v: String,
}

impl FromStr for KvPair {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split("=");
        let err = || anyhow!(format!("Failed to parse {}", s));

        Ok(Self {
            k: (split.next().ok_or_else(err)?).to_string(),
            v: (split.next().ok_or_else(err)?).to_string(),
        })
    }
}

// 解析结构体的方法

fn parse_kv_pair(s: &str) -> Result<KvPair> {
    s.parse()
}

// 解析url的方法

fn parse_url(s: &str) -> Result<String> {
    let _url: Url = s.parse()?;

    Ok(s.into())
}

// 拿到解析后的参数，并发送get请求

async fn get(client: Client, args: &Get) -> Result<()> {
    let resp = client.get(&args.url).send().await?;
    // 使用
    Ok(print_resp(resp).await?)
}

// 拿到解析后的参数，并发送get请求

async fn post(client: Client, args: &Post) -> Result<()> {
    let mut body = HashMap::new();
    for pair in args.body.iter() {
        body.insert(&pair.k, &pair.v);
    }

    let resp = client.post(&args.url).json(&body).send().await?;
    Ok(print_resp(resp).await?)
}

// 打印响应状态

fn print_status(resp: &Response) {
    let status = format!("{:?} {}", resp.version(), resp.status()).blue();
}
// 打印响应头

fn print_header(resp: &Response) {
    for (name, value) in resp.headers() {
        println!("{} {:?}", name.to_string().green(), value)
    }

    println!()
}

// 打印响应体

fn print_body(m: Option<Mime>, body: &str) {
    match m {
        Some(v) if v == mime::APPLICATION_JSON => print_syntect(body, "json"),
        Some(v) if v == mime::TEXT_HTML => print_syntect(body, "html"),

        _ => println!("{}", body),
    }
}
// 打印响应
async fn print_resp(resp: Response) -> Result<()> {
    print_status(&resp);
    print_header(&resp);
    let mime = get_content_type(&resp);
    let body = resp.text().await?;

    print_body(mime, &body);
    Ok(())
}

// 打印响应类型

fn get_content_type(resp: &Response) -> Option<Mime> {
    resp.headers()
        .get(header::CONTENT_TYPE)
        .map(|v| v.to_str().unwrap().parse().unwrap())
}

#[tokio::main]
async fn main() -> Result<()> {
    let opts = Opts::parse();

    // 响应头是两对key value
    let mut headers = header::HeaderMap::new();
    headers.insert("X-POWERED_BY", "Rust".parse()?);
    headers.insert(header::USER_AGENT, "Rust Httpie".parse()?);

    // 构建客户端
    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    // 根据命令发送请求，并对获得的异步结果进行打印
    let result = match opts.subcmd {
        SubCommand::Get(ref args) => get(client, args).await?,
        SubCommand::Post(ref args) => post(client, args).await?,
    };

    Ok(result)
}

// 漂亮打印

fn print_syntect(s: &str, ext: &str) {
    let ps = SyntaxSet::load_defaults_newlines();

    let ts = ThemeSet::load_defaults();

    let syntax = ps.find_syntax_by_extension(ext).unwrap();
    let mut h = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);
    for line in LinesWithEndings::from(s) {
        // let ranges: Vec<(Style, &str)> = h.highlight(line, &ps);
        let ranges = h.highlight_line(line, &ps);
        match ranges {
            core::result::Result::Ok(vecs) => {
                let escaped = as_24_bit_terminal_escaped(&vecs[..], true);
                print!("{}", escaped);
            }
            Err(e) => println!(""),
        }
    }
}
