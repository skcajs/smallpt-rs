use super::tup::Tup;
use super::ray::Ray;

pub fn minkowski(ray: &Ray, previous_point: Tup, time_step: f64) -> Ray {
    let current_point = previous_point + ray.d * time_step;

    Ray {o: current_point, d: (current_point - previous_point).norm()}
}

pub fn schwartzchild(current_ray: &Ray, current_point: Tup) -> Ray {
    Ray {o: current_point, d: current_ray.d}
}