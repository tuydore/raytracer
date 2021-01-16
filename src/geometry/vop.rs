#[derive(Debug, PartialEq, Clone, Copy)]
pub struct VOP {
    pub index_of_refraction: f64,
}

impl VOP {
    pub fn new(index_of_refraction: f64) -> Self {
        Self {
            index_of_refraction,
        }
    }
}
