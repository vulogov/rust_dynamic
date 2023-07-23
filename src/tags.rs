use crate::value::Value;

impl Value {
    pub fn set_tag<N: AsRef<str> + std::fmt::Display>(&mut self, key: N, value: N)  {
        let _ = self.tags.insert(key.as_ref().to_string(), value.as_ref().to_string());
    }
    pub fn get_tag<N: AsRef<str> + std::fmt::Display>(&mut self, key: N) -> Option<String> {
        self.tags.get(&key.as_ref().to_string()).cloned()
    }
    pub fn has_tag<N: AsRef<str> + std::fmt::Display>(&mut self, key: N) -> bool {
        self.tags.contains_key(&key.as_ref().to_string())
    }
}
