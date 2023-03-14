use anyhow::{anyhow, Result};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, SaltString},
    Argon2, Params, PasswordVerifier, Version,
};
use lazy_static::lazy_static;
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use std::env;
// argon2 hash 使用的密码
const ARGON_SECRET: &[u8] = b"deadbeef";
lazy_static! {
    static ref ARGON2: Argon2<'static> = Argon2::new_with_secret(
        ARGON_SECRET,
        argon2::Algorithm::default(),
        argon2::Version::default(),
        argon2::Params::default()
    )
    .unwrap();
}

// user对应的数据结构，处理login/register
pub struct UserDb {
    pool: SqlitePool,
}

// 使用FromRow宏把数据库中读出来的内容转换为User结构
#[derive(Debug, sqlx::FromRow)]
pub struct User {
    id: i64,
    email: String,
    hashed_password: String,
}

impl UserDb {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    // 注册
    pub async fn register(&self, email: &str, password: &str) -> Result<i64> {
        let hashed_password = generate_password_hash(password)?;
        let id = sqlx::query("INSERT INTO users(email,hashed_password) VALUES (?,?)")
            .bind(email)
            .bind(hashed_password)
            .execute(&self.pool)
            .await?
            .last_insert_rowid();

        Ok(id)
    }

    // 用户登录：从user表中获取用户信息，并验证用户密码
    pub async fn login(&self, email: &str, password: &str) -> Result<String> {
        let user: User = sqlx::query_as("SELECT * from users WHERE email =?")
            .bind(email)
            .fetch_one(&self.pool)
            .await?;

        println!("find user: {:?}", user);

        if let Err(_) = verify_password(password, &user.hashed_password) {
            return Err(anyhow!("Failed to login"));
        }

        Ok("awesome token".into())
    }
}

// 创建Users表

async fn recreate_table(pool: &SqlitePool) -> Result<()> {
    // 丢掉表重新创建
    sqlx::query("DROP TABLE IF EXISTS users")
        .execute(pool)
        .await?;

    sqlx::query(
        r#"CREATE TABLE IF NOT EXISTS users (
          id INTEGER PRIMARY KEY NOT NULL, 
          email VARCHAR UNIQUE NOT NULL, 
          hashed_password VARCHAR NOT NULL    
     )"#,
    )
    .execute(pool)
    .await?;

    Ok(())
}

// 创建安全的密码哈希

fn generate_password_hash(password: &str) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);

    Ok(ARGON2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| anyhow!("failed to hash password"))?
        .to_string())
}

// 验证密码和哈希
fn verify_password(password: &str, password_hash: &str) -> Result<()> {
    let parsed_hash =
        PasswordHash::new(password_hash).map_err(|_| anyhow!("failed to parse hashed password"))?;

    ARGON2
        .verify_password(password.as_bytes(), &parsed_hash)
        .map_err(|_| anyhow!("failed to verify hashed password"))?;

    Ok(())
}

#[tokio::main]

async fn main() -> Result<()> {
    let url = env::var("DATABASE_URL").unwrap_or("sqlite://./data/example.db".into());

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await?;

    recreate_table(&pool).await?;

    let user_db = UserDb::new(pool.clone());
    let email = "shiyivei@outlook.com";
    let password = "shiyivei";

    let id = user_db.register(email, password).await?;
    println!("Registered id {}", id);

    //登录

    let token = user_db.login(email, password).await?;
    println!("Login token {}", token);

    // 登录失败

    let result = user_db.login(email, "badpass").await;
    println!("Login should fail with bad password {:?}", result);

    Ok(())
}
