use std::hash::{Hasher, Hash};
use crate::value::Value;

impl Hash for Value {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.id.hash(hasher);
    }
}
