use crate::value::Value;

impl Value {
    pub fn calc_q(&mut self, other: Value) -> &mut Self {
        self.q = (self.get_q() + other.get_q())/2.0;
        self
    }
    pub fn set_q(&mut self, q: f64) -> &mut Self {
        self.q = q;
        self
    }
    pub fn get_q(&self) -> f64 {
        self.q
    }
}
