pub struct NoSQLClient {
    connection: String,
    data: std::collections::HashMap<String, String>,
    lists: std::collections::HashMap<String, Vec<String>>,
}

impl NoSQLClient {
    pub fn new(connection: &str) -> Self {
        Self {
            connection: connection.to_string(),
            data: std::collections::HashMap::new(),
            lists: std::collections::HashMap::new(),
        }
    }

    pub fn set(&mut self, key: &str, value: &str) -> Result<(), String> {
        self.data.insert(key.to_string(), value.to_string());
        Ok(())
    }

    pub fn get(&self, key: &str) -> Option<String> {
        self.data.get(key).cloned()
    }

    pub fn del(&mut self, key: &str) -> Result<(), String> {
        self.data.remove(key);
        self.lists.remove(key);
        Ok(())
    }

    pub fn lpush(&mut self, key: &str, value: &str) -> Result<(), String> {
        let list = self.lists.entry(key.to_string()).or_insert_with(Vec::new);
        list.push(value.to_string());
        Ok(())
    }

    pub fn lrange(&self, key: &str) -> Option<&Vec<String>> {
        self.lists.get(key)
    }

    pub fn keys(&self) -> Vec<String> {
        let mut all_keys: Vec<String> = self.data.keys().cloned().collect();
        all_keys.extend(self.lists.keys().cloned());
        all_keys.sort();
        all_keys.dedup();
        all_keys
    }

    pub fn mset(&mut self, pairs: &[(&str, &str)]) -> Result<(), String> {
        for (key, value) in pairs {
            self.data.insert(key.to_string(), value.to_string());
        }
        Ok(())
    }

    pub fn mget(&self, keys: &[&str]) -> Vec<Option<String>> {
        keys.iter()
            .map(|key| self.data.get(*key).cloned())
            .collect()
    }
}