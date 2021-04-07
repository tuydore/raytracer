use crate::Ray;
use crate::Surface;
use std::sync::Arc;

fn trace_one_cycle(rays: &mut Vec<Ray>, surfaces: &[Arc<dyn Surface + Send + Sync>]) {
    todo!()
}
