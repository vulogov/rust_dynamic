use crate::value::Value;

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Value {

}
