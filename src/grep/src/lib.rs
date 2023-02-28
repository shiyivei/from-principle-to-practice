use clap::Parser;

use colored::*;
use itertools::Itertools;

use rayon::iter::{IntoParallelIterator, ParallelIterator};
use regex::Regex;

use std::{
    fs::File,
    io::{self, BufRead, BufReader, Read, Stdout, Write},
    ops::Range,
    path::Path,
};

mod error;
pub use error::GrepError;

// 使用 type alias可以重定义类型 ，后续书写更加简单（也将函数签名定义为类型别名）
pub type StrategyFn<W, R> = fn(&Path, BufReader<R>, &Regex, &mut W) -> Result<(), GrepError>;

#[derive(Debug, Parser)]
#[clap(version = "1.0", author = "shiyivei@outlook.com")]
pub struct GrepConfig {
    pattern: String,
    glob: String,
}

impl GrepConfig {
    pub fn match_with_default_strategy(&self) -> Result<(), GrepError> {
        self.match_with(default_strategy)
    }
    // 使用策略查找匹配
    pub fn match_with(&self, strategy: StrategyFn<Stdout, File>) -> Result<(), GrepError> {
        let regex = Regex::new(&self.pattern)?;

        // 生成所有符合通配符的文件列表
        let files: Vec<_> = glob::glob(&self.glob)?.collect();

        // 并行处理文件
        files.into_par_iter().for_each(|v| {
            if let Ok(filename) = v {
                if let Ok(file) = File::open(&filename) {
                    let reader = BufReader::new(file);
                    let mut stdout = io::stdout();

                    if let Err(e) = strategy(filename.as_path(), reader, &regex, &mut stdout) {
                        println!("Internal error: {:?}", e)
                    }
                }
            }
        });

        Ok(())
    }
}

// 默认策略
pub fn default_strategy<W: Write, R: Read>(
    path: &Path,
    reader: BufReader<R>,
    pattern: &Regex,
    writer: &mut W,
) -> Result<(), GrepError> {
    let matches: String = reader
        .lines()
        .enumerate()
        .map(|(lineno, line)| {
            line.ok()
                .map(|line| {
                    pattern
                        .find(&line)
                        .map(|m| format_line(&line, lineno + 1, m.range()))
                })
                .flatten()
        })
        .filter_map(|v| v.ok_or(()).ok())
        .join("\n");

    if !matches.is_empty() {
        writer.write(path.display().to_string().green().as_bytes())?;
        writer.write(b"\n");
        writer.write(matches.as_bytes())?;
        writer.write(b"\n");
    }
    Ok(())
}

/// 格式化输出匹配的行，包含行号、列号和带有高亮的第一个匹配项目

pub fn format_line(line: &str, lineno: usize, range: Range<usize>) -> String {
    let Range { start, end } = range;
    let prefix = &line[..start];

    format!(
        "{0: >6}:{1: <3} {2}{3}{4}",
        lineno.to_string().blue(),
        (prefix.chars().count() + 1).to_string().cyan(),
        prefix,
        &line[start..end].red(),
        &line[end..]
    )
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn format_line_should_work() {
        let result = format_line("Hello, Shi~", 1000, 7..10);
        let expected = format!(
            "{0: >6}:{1: <3} Hello, {2}~",
            "1000".blue(),
            "7".cyan(),
            "Shi".red()
        );

        assert_eq!(result, expected)
    }

    #[test]
    fn default_strategy_should_work() {
        let path = Path::new("src/main.rs");
        let input = b"hello world!\nhey yivei!";

        let reader = BufReader::new(&input[..]);
        let pattern = Regex::new(r"he\\w+").unwrap();

        let mut writer = Vec::new();
        default_strategy(path, reader, &pattern, &mut writer);

        let result = String::from_utf8(writer).unwrap();

        let expected = [
            String::from("src/main.rs"),
            format_line("hello world!", 1, 0..5),
            format_line("hello yivei!\n!", 2, 0..3),
        ];

        assert_eq!(result, expected.join("\n"));
    }
}
