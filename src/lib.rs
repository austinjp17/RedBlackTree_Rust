
pub mod operations;
pub mod structures;

use operations::rotations;

// Properties
// 1) Root is black
// 2) Every leaf (NULL) is black
// 3) No adjacent red nodes
// 4) Every path from a node to Null desendents has the same number of black nodes
// If Node red -> Both children black


// TEMP
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Hash)]
pub struct DollarValue {
    pub integer: u32,
    pub fractional: u32,
}
impl DollarValue {
    pub fn new(price: f64, precision: u8) -> Self {        
        DollarValue  {
            integer: price.floor() as u32,
            fractional: (price.fract() * 10_f64.powf(precision as f64)) as u32,
        }
    }
}

impl Ord for DollarValue {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.integer == other.integer {
            return self.fractional.cmp(&other.fractional);
        }
        self.integer.cmp(&other.integer)
    }
}




