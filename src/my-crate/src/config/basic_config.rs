#![allow(unused)]
use super::error::ConfigError;
use crate::config::database::Database;
use crate::environment::Environment;
use crate::environment::Environment::*;
use crate::*;

pub type Result<T> = ::std::result::Result<T, ConfigError>;

#[derive(Debug)]
pub struct BasicConfig {
    pub environment: Environment,
    address: String,
    port: u16,
    database: Option<Database>,
    workers: Option<u16>,
    pub(crate) config_file_path: Option<PathBuf>,
    pub(crate) root_path: Option<PathBuf>,
}

impl PartialEq for BasicConfig {
    fn eq(&self, other: &BasicConfig) -> bool {
        self.address == other.address && self.port == other.port && self.workers == other.workers
    }
}

impl BasicConfig {
    // 读取的传入参数
    pub fn new(env: Environment, config_table: &Map<String, Value>) -> Self {
        Self::default(env, config_table)
    }

    // 抽象配置过程
    pub(crate) fn default(env: Environment, config_table: &Map<String, Value>) -> Self {
        let default_workers = (num_cpus::get() * 2) as u16;

        // println!("{:?}", config_table);
        let config: Option<&Value>;

        match env {
            Environment::Development => config = config_table.get("development"),
            Environment::Staging => config = config_table.get("staging"),
            Environment::Production => config = config_table.get("production"),
        }
        let mut database = Database {
            adapter: "".to_string(),
            db_name: "".to_string(),
            pool: 0,
        };

        if let Some(v) = config {
            let db = v.get("database");
            if let Some(db) = db {
                let adapter = db.get("adapter");

                match adapter {
                    Some(value) => match value.as_str() {
                        Some(name) => {
                            database.adapter = name.to_string();
                            // println!("{:#?}", name)
                        }
                        None => println!(""),
                    },

                    None => println!(""),
                }
                let db_name = db.get("db_name");

                match db_name {
                    Some(value) => match value.as_str() {
                        Some(name) => {
                            database.db_name = name.to_string();
                            // println!("{:#?}", name)
                        }
                        None => println!(""),
                    },

                    None => println!(""),
                }
                let pool = db.get("pool");

                match pool {
                    Some(value) => match value.as_integer() {
                        Some(name) => {
                            // println!("pool {:#?}", name);
                            database.pool = name;

                            // println!("pool {:#?}", name)
                        }
                        None => println!(""),
                    },

                    None => println!(""),
                }
            }
        }

        let mut base = BasicConfig {
            environment: Environment::Development,
            address: "0.0.0.0".to_string(),
            port: 9000,
            database: Some(database),
            workers: Some(default_workers),
            config_file_path: None,
            root_path: None,
        };

        match env {
            Development => BasicConfig {
                environment: Development,
                address: "localhost".to_string(),
                port: 8000,
                ..base
            },
            Staging => BasicConfig {
                environment: Staging,
                ..base
            },
            production => BasicConfig {
                environment: Environment::Production,
                ..base
            },
        }
    }

    // 把路径解析为引用类型
    pub fn set_root<P: AsRef<Path>>(&mut self, path: P) {
        self.root_path = Some(path.as_ref().into());
    }

    // 从对应的路径中解析
    pub fn default_from<P>(
        env: Environment,
        path: P,
        config_table: &Map<String, Value>,
    ) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let mut config = BasicConfig::new(env, config_table);

        let config_file_path = path.as_ref().to_path_buf();
        if let Some(parent) = config_file_path.parent() {
            config.set_root(parent);
        } else {
            let msg = "Configuration files must be rooted in a directory";
            return Err(ConfigError::BadFilePath(config_file_path.clone(), msg));
        }

        config.config_file_path = Some(config_file_path);
        Ok(config)
    }
}
