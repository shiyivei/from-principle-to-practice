use anyhow::{anyhow, Result};
use dialect::TyrDialect;
use polars::prelude::*;
use polars::prelude::{CsvWriter, DataFrame};
use sqlparser::parser::Parser;
use std::ops::{Deref, DerefMut};

use crate::fetcher::retrieve_data;

mod convert;
mod dialect;
mod fetcher;
mod loader;
use convert::Sql;
use loader::detect_content;

// 定义自己的数据结构包裹别人的结构，然后实现一些自定义的方法

#[derive(Debug)]
pub struct DataSet(DataFrame);

// 注意deref 和derefmut 的实现方式

impl Deref for DataSet {
    // 把类型变为一个智能指针，指定解引用的目标类型
    type Target = DataFrame;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for DataSet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl DataSet {
    // 从 DataSet 转为 csv
    pub fn to_csv(&self) -> Result<String> {
        let mut buf = Vec::new();

        let writer = CsvWriter::new(&mut buf);

        writer.finish(self)?;

        Ok(String::from_utf8(buf)?)
    }
}

// 从 from 获取数据，从 filter 过滤数据

pub async fn query<T: AsRef<str>>(sql: T) -> Result<DataSet> {
    let ast = Parser::parse_sql(&TyrDialect::default(), sql.as_ref())?;

    if ast.len() != 1 {
        return Err(anyhow!("Only support single sql at the moment"));
    }

    let sql = &ast[0];

    let Sql {
        source,
        condition,
        selection,
        offset,
        limit,
        order_by,
    } = sql.try_into()?;

    let ds = detect_content(retrieve_data(source).await?).load()?;

    let mut filtered = match condition {
        Some(expr) => ds.0.lazy().filter(expr),
        None => ds.0.lazy(),
    };

    filtered = order_by
        .into_iter()
        .fold(filtered, |acc, (col, desc)| acc.sort(&col, desc));

    if offset.is_some() || limit.is_some() {
        filtered = filtered.slice(offset.unwrap_or(0), limit.unwrap_or(usize::MAX));
    }

    Ok(DataSet(filtered.select(selection).collect()?))
}
