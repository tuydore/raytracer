use serde::Deserialize;

#[derive(Debug, PartialEq, Clone, Copy, Deserialize)]
pub struct VOP {
    pub ior: f64,
    pub abs: [f64; 3],
}

impl Default for VOP {
    fn default() -> Self {
        Self {
            ior: 1.0,
            abs: [0.0; 3],
        }
    }
}
