use thiserror::Error;

// 使用thiserror定义错误类型
// 1. 将其它类型的错误转换为自定义的错误，相当于重命名

#[derive(Debug, Error)]
pub enum GrepError {
    #[error("Glob pattern error")]
    GlobPatternError(#[from] glob::PatternError),
    #[error("Regex pattern error")]
    RegexPatternError(#[from] regex::Error),
    #[error("I/O pattern error")]
    IoError(#[from] std::io::Error),
}

// 2. 直接定义错误

// #[derive(Debug, Error, PartialEq)]
// pub enum KvError {
//     // 使用字段属性定义错误内容
//     #[error("Not found for table: {},key: {1}")]
//     NotFound(String, String),
//     #[error("Cannot parse command: `{0}`")]
//     InvalidCommand(String),
//     #[error("Cannot convert value {:0} to {1}")]
//     ConvertError(Value, &'static str),
//     #[error("Cannot process command {0} with table: {1}, key: {2}. Error: {}")]
//     StorageError(&'static str, String, String, String),

//     //使用第三发库的具体Error类型
//     #[error("Failed to encode protobuf message")]
//     EncodeError(#[from] prost::EncodeError),
//     #[error("Failed to decode protobuf message")]
//     DecodeError(#[from] prost::DecodeError),
//     #[error("Internal error: {0}")]
//     Internal(String),
// }
