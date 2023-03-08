//! HashMap 和 HashSet
//!
//!
/**

```
 // 一 HashMap
   // 单线程情况下使用

   use std::collections::HashMap;
   let mut map: HashMap<String, u32> = HashMap::new();
   map.insert("Rust".to_string(), 100);

   use futures::lock::Mutex;
   use std::sync::{Arc, RwLock};

   // 多线程情况下使用
   let mut m: Arc<RwLock<HashMap<&str, i32>>> = Arc::new(RwLock::new(HashMap::new()));
   let mut m = m.write().unwrap();
   m.insert("Rust", 1);

   let mut l: Arc<Mutex<HashMap<&str, i32>>> = Arc::new(Mutex::new(HashMap::new()));

   let mut x = l.try_lock().unwrap();
   x.insert("Rust", 1);

   // 二 HashSet

   let mut map: HashSet<String> = HashSet::new();
   map.insert("Rust".to_string());

   let mut m_set: Arc<RwLock<HashSet<&str>>> = Arc::new(RwLock::new(HashSet::new()));
   let mut m_set = m_set.write().unwrap();
   m_set.insert("Rust");

   // ....

```
*/

pub fn map() {}
