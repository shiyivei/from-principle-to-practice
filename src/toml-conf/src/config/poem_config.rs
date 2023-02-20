#![allow(unused)]
use super::error::ConfigError;
use crate::config::database::Database;
use crate::environment::Environment;
use crate::environment::Environment::*;
use crate::*;
use toml::de::Error;
trait DoubleOption {
    fn double_usize(&self) -> Option<(usize, usize)>;
}

impl DoubleOption for Error {
    fn double_usize(&self) -> Option<(usize, usize)> {
        return Some((1, 1));
    }
}

// 定义常量,配置文件
const CONFIG_FILENAME: &str = "conf/Poem.toml";
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

#[doc(hidden)]
#[derive(Debug, PartialEq)]

// 配置类型，用于匹配toml文件中的配置项
pub struct PoemConfig {
    pub active_env: Environment,
    config: HashMap<Environment, BasicConfig>,
}

// 实现读配置方法
impl PoemConfig {
    pub fn read_config() -> Result<PoemConfig> {
        // 获得文件名称（带路径）
        let file = PoemConfig::find()?;

        // println!();
        // println!(" 1. get file path: {:?}", file);
        // println!();

        // 打开文件
        let mut handle = File::open(&file).map_err(|_| ConfigError::IoError)?;
        // 建一个内容缓存
        let mut contents = String::new();

        // 读取内容
        handle
            .read_to_string(&mut contents)
            .map_err(|_| ConfigError::IoError)?;

        // 解析内容
        PoemConfig::parse(contents, &file)
    }

    fn find() -> Result<PathBuf> {
        // 获取crate根目录
        // Result 都可以用map_err处理
        // 函数必须是
        let cwd = env::current_dir().map_err(|_| ConfigError::NotFound)?;

        let mut current = cwd.as_path();

        loop {
            // 配置加上配置文件目录
            let manifest = current.join(CONFIG_FILENAME);

            // 从文件中获取元数据，注意元数据不是数据本身
            if fs::metadata(&manifest).is_ok() {
                return Ok(manifest);
            }
            // 获取根目录的父路径
            match current.parent() {
                Some(p) => current = p,
                None => break,
            }
        }
        Err(ConfigError::NotFound)
    }
    fn get_mut(&mut self, env: Environment) -> &mut BasicConfig {
        match self.config.get_mut(&env) {
            Some(config) => config,
            None => panic!("set(): {} config is missing.", env),
        }
    }

    pub fn active_default_from(
        filename: Option<&Path>,
        config_table: &Map<String, Value>,
    ) -> Result<PoemConfig> {
        // 新建哈希表
        let mut defaults = HashMap::new();
        // 通过默认把几个基本配置装入map
        if let Some(path) = filename {
            // 并设置了路径
            defaults.insert(
                Development,
                BasicConfig::default_from(Development, &path, config_table)?,
            );
            defaults.insert(
                Staging,
                BasicConfig::default_from(Staging, &path, config_table)?,
            );
            defaults.insert(
                Production,
                BasicConfig::default_from(Production, &path, config_table)?,
            );
        } else {
            // 只装入了map
            defaults.insert(Development, BasicConfig::default(Development, config_table));
            defaults.insert(Staging, BasicConfig::default(Staging, config_table));
            defaults.insert(Production, BasicConfig::default(Production, config_table));
        }

        let mut config = PoemConfig {
            active_env: Environment::active()?,
            config: defaults,
        };

        Ok(config)
    }

    pub fn active(config_table: &Map<String, Value>) -> Result<BasicConfig> {
        Ok(BasicConfig::new(Environment::active()?, config_table))
    }

    fn parse<P: AsRef<Path>>(src: String, filename: P) -> Result<PoemConfig> {
        // 拿到文件路径
        let path = filename.as_ref().to_path_buf();

        // 从String解析toml文件内容
        let table = match src.parse::<toml::Value>() {
            Ok(toml::Value::Table(table)) => table,
            Ok(value) => {
                let err = format!("expected a table, found {}", value.type_str());
                return Err(ConfigError::ParseError(src, path, err, Some((1, 1))));
            }

            Err(e) => {
                return Err(ConfigError::ParseError(
                    src,
                    path,
                    e.to_string(),
                    e.double_usize(),
                ))
            }
        };

        let mut config = PoemConfig::active_default_from(Some(filename.as_ref()), &table)?;

        for (entry, value) in table {
            let kv_pairs = match value.as_table() {
                Some(table) => table,
                None => {
                    return Err(ConfigError::BadType(
                        entry,
                        "a table",
                        value.type_str(),
                        Some(path.clone()),
                    ))
                }
            };
        }

        Ok(config)
    }
}
