use anyhow::Result;
use polars::prelude::*;
use queryer::query;
use std::io::Cursor;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let url = "https://raw.githubusercontent.com/owid/covid-19-data/master/public/data/latest/owid-covid-latest.csv";

    // let data = reqwest::get(url).await?.text().await?;

    // // 直接把string转为df
    // let data_frame = CsvReader::new(Cursor::new(data))
    //     .infer_schema(Some(16))
    //     .finish()?;

    // let filtered = data_frame.filter(&data_frame["new_deaths"].gt(500)?)?;

    // println!(
    //     "{:?}",
    //     filtered.select([
    //         "location",
    //         "total_cases",
    //         "new_cases",
    //         "total_deaths",
    //         "new_deaths"
    //     ])
    // );

    // 以前需要查询

    // let filtered = data_frame.filter(&data_frame["new_deaths"].gt(500))?;

    // println!(
    //     "{:?}",
    //     filtered.select((
    //         "location",
    //         "total_cases",
    //         "new_cases",
    //         "total_deaths",
    //         "new_deaths"
    //     ))
    // );

    // 现在直接拿 sql 语句查询

    let url = "file://owid-covid-latest.csv";

    let sql = format!("SELECT location name, total_cases, new_cases, total_deaths, new_deaths FROM {} where new_deaths >= 500 ORDER BY new_cases DESC", url);

    let df1 = query(sql).await?;

    println!("query result {:?}", df1);

    Ok(())
}
