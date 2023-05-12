#[derive(Clone, Debug, Hash)]
#[derive(PartialEq, PartialOrd)]
pub struct Number {
    value: String,
}

impl Number {
    pub fn new(v: f64) -> Number {
        Number { value: v.to_string() }
    }

    pub fn to_value(&self) -> f64 {
        let ret = self.value.parse::<f64>();
        match ret {
            Ok(v) => {
                return v;
            }
            Err(_e) => {
                panic!();
            }
        }
    }
}