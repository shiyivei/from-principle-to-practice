#![allow(unused)]
use self::ConfigError::*;
use crate::*;

// 定义错误
#[derive(Debug)]
pub enum ConfigError {
    NotFound,
    IoError,
    BadFilePath(PathBuf, &'static str),
    BadEnv(String),
    BadEntry(String, PathBuf),
    BadType(String, &'static str, &'static str, Option<PathBuf>),
    ParseError(String, PathBuf, String, Option<(usize, usize)>),
}

// 为错误实现trait
impl Error for ConfigError {
    fn description(&self) -> &str {
        match *self {
            NotFound => "config file was not found",
            IoError => "there was I/O error while reading config file",
            BadFilePath(..) => "the config file path is invalid",
            BadEnv(..) => "the environment specified in `ROCKET_ENV` is invalid",
            BadEntry(..) => "the environment specified as `[environment]` is invalid",
            BadType(..) => "a key was specified with a value of the wrong type",
            ParseError(..) => "the config file contains invalid TOML",
        }
    }
}

// 打印

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            NotFound => write!(f, "config file was not found"),
            IoError => write!(f, "there was I/O error while reading config file"),
            BadFilePath(ref p, _) => write!(f, "{:?} is not a valid config path file", p),
            BadEnv(ref e) => write!(f, "{:?} is not a valid `ROCKET_ENV` val", e),
            BadEntry(ref e, _) => write!(f, "{:?} is not a valid `[environment]` entry", e),
            BadType(ref n, e, a, _) => {
                write!(f, "type mismatch for {}. expected {}, found {}", n, e, a)
            }
            ParseError(..) => write!(f, "the config file contains invalid TOML"),
        }
    }
}
