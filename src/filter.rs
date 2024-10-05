
use crate::sampler::Sampler;

pub fn tent_filter(sampler: &mut Sampler) -> (f64, f64) {
    let (r1, r2) = sampler.next_2d();

    let dx = if r1 < 0.5 { -1.0 } else { 1.0 } * (2.0 - 2.0 * r1).sqrt();
    let dy = if r2 < 0.5 { -1.0 } else { 1.0 } * (2.0 - 2.0 * r2).sqrt();

    (dx, dy)
}