// 创建storage trait

// 注意 storage 需要并发安全访问，所以要用到 Arc以及读写锁 RwLock

// crate代表当前 lib
use crate::storage::MemTable;
use crate::{KvError, Kvpair, Value};

// 定义一个 Storage 约束所有对Storage的操作行为,增删改查
// 有接口就知道类型有哪些方法可以操作了

pub trait Storage {
    fn get(&self, table: &str, key: &str) -> Result<Option<Value>, KvError>;
    fn set(&self, table: &str, key: String, value: Value) -> Result<Option<Value>, KvError>; // 返回前值
    fn contains(&self, table: &str, key: &str) -> Result<bool, KvError>;
    fn del(&self, table: &str, key: &str) -> Result<Option<Value>, KvError>;
    fn get_all(&self, table: &str) -> Result<Vec<Kvpair>, KvError>;
    fn get_iter(&self, table: &str) -> Result<Box<dyn Iterator<Item = Kvpair>>, KvError>;
}

pub struct StorageIter<T> {
    data: T,
}

impl<T> StorageIter<T> {
    pub fn new(data: T) -> Self {
        Self { data }
    }
}

impl<T> Iterator for StorageIter<T>
where
    T: Iterator,
    T::Item: Into<Kvpair>,
{
    type Item = Kvpair;
    fn next(&mut self) -> Option<Self::Item> {
        self.data.next().map(|v| v.into())
    }
}

#[cfg(test)]
// 单元测试写在 实现之前，是标准的TDD(Test-Driven Deployment)
mod tests {

    use super::*;

    #[test]
    fn memtable_basic_interface_should_work() {
        let store = MemTable::new();
        test_basi_interface(store)
    }

    #[test]
    fn mem_get_all_should_work() {
        let store = MemTable::new();
        test_get_all(store)
    }

    #[test]
    fn memtable_iter_should_work() {
        let store = MemTable::new();
        test_get_iter(store);
    }

    // 如果测试函数中内容太多的话，需要收敛到新的函数中
    // 测试驱动开发
    fn test_basi_interface(store: impl Storage) {
        // 第一次 set 会创建 table，插入 key 并返回 None（之前没值）
        let v = store.set("t1", "k1".into(), "v".into());
        assert!(v.unwrap().is_none());
        // 再次 set 同样的 key 会更新，并返回之前的值
        let v1 = store.set("t1", "k1".into(), "v1".into());
        assert_eq!(v1, Ok(Some("v".into())));
        // get 存在的 key 会得到最新的值
        let v = store.get("t1", "k1");
        assert_eq!(v, Ok(Some("v1".into())));

        // get 不存在的key or 不存在的 table
        assert_eq!(store.get("t1", "k2"), Ok(None));
        assert!(store.get("t2", "k1").unwrap().is_none());
        // contains 纯在的 key 返回 true，否则 false
        assert_eq!(store.contains("t1", "k1"), Ok(true));
        assert_eq!(store.contains("t1", "k2"), Ok(false));
        assert_eq!(store.contains("t2", "k1"), Ok(false));
        // del 存在的 key 返回之前的值
        let v = store.del("t1", "k1");
        assert_eq!(v, Ok(Some("v1".into())));
        // del 存在的 key 返回之前的值
        let v = store.del("t1", "hello");
        assert_eq!(v, Ok(None));
        // del 不存在的 key 或 table 返回 None
        assert_eq!(Ok(None), store.del("t1", "k1"));
        assert_eq!(Ok(None), store.del("t2", "k"));
    }

    fn test_get_all(store: impl Storage) {
        store.set("t2", "k1".into(), "v1".into()).unwrap();
        store.set("t2", "k2".into(), "v2".into()).unwrap();

        let mut data = store.get_all("t2").unwrap();
        data.sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert_eq!(
            data,
            vec![
                Kvpair::new("k1", "v1".into()),
                Kvpair::new("k2", "v2".into())
            ]
        );
    }

    fn test_get_iter(store: impl Storage) {
        store.set("t2", "k1".into(), "v1".into()).unwrap();
        store.set("t2", "k2".into(), "v2".into()).unwrap();

        let mut data: Vec<_> = store.get_iter("t2").unwrap().collect();
        data.sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert_eq!(
            data,
            vec![
                Kvpair::new("k1", "v1".into()),
                Kvpair::new("k2", "v2".into())
            ]
        );
    }
}
