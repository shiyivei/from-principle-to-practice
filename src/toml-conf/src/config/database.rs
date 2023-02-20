#[derive(Debug)]
pub struct Database {
    pub(crate) adapter: String,
    pub(crate) db_name: String,
    pub(crate) pool: i64,
}
