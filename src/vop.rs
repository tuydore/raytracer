use serde::Deserialize;

#[derive(Debug, PartialEq, Clone, Copy, Deserialize)]
pub struct VOP {
    pub ior: f64,
}
