use anyhow::{anyhow, Result};
use async_trait::async_trait;
use tokio::fs;

#[async_trait] // rust 中的异步 trait还未稳定，在此使用 async_trait宏
pub trait Fetch {
    type Error;
    async fn fetch(&self) -> Result<String, Self::Error>;
}

//从网路或者源文件获取数据
pub async fn retrieve_data(source: impl AsRef<str>) -> Result<String> {
    let name = source.as_ref();

    match &name[..4] {
        "http" => UrlFetcher(name).fetch().await,
        // 处理 file://<filename>
        "file" => FileFetcher(name).fetch().await,
        _ => return Err(anyhow!("We only support http/https/file at the moment")),
    }
}

struct UrlFetcher<'a>(pub(crate) &'a str);
struct FileFetcher<'a>(pub(crate) &'a str);

#[async_trait] // 使用的时候也要使用
impl<'a> Fetch for UrlFetcher<'a> {
    type Error = anyhow::Error;
    async fn fetch(&self) -> Result<String, Self::Error> {
        Ok(reqwest::get(self.0).await?.text().await?)
    }
}
#[async_trait]
impl<'a> Fetch for FileFetcher<'a> {
    type Error = anyhow::Error;
    async fn fetch(&self) -> Result<String, Self::Error> {
        Ok(fs::read_to_string(&self.0[7..]).await?)
    }
}

// 如果想要再增加一个源，那再增加结构体 + 实现trait + 匹配
