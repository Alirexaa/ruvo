use dashmap::DashMap;

pub struct StringMemoryStorage {
    inner: DashMap<String, String, ahash::RandomState>,
}

impl StringMemoryStorage {
    pub fn new() -> Self {
        StringMemoryStorage {
            inner: DashMap::with_hasher(ahash::RandomState::new()),
        }
    }

    pub fn insert(&self, key: String, value: String) -> Option<String> {
        self.inner.insert(key, value)
    }

    pub fn get(&self, key: String) -> Option<String> {
        match self.inner.get(&key) {
            Some(v) => Some(v.clone()),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::StringMemoryStorage;

    #[test]
    fn test_insert() {
        let st = StringMemoryStorage::new();
        let old_value = st.insert("key".to_string(), "value".to_string());
        assert!(old_value.is_none());
        let old_value = st.insert("key".to_string(), "value2".to_string());
        assert_eq!(Some("value".to_string()), old_value);
    }

    #[test]
    fn test_get() {
        let st = StringMemoryStorage::new();
        let value = st.get("key".to_string());
        assert!(value.is_none());

        let old_value = st.insert("key".to_string(), "value".to_string());
        assert!(old_value.is_none());
        let value = st.get("key".to_string());
        assert_eq!(Some("value".to_string()), value);

        let old = st.insert("key".to_string(), "value2".to_string());
        assert_eq!(Some("value".to_string()), old);
        let value = st.get("key".to_string());
        assert_eq!(Some("value2".to_string()), value);
    }
}
