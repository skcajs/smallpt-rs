use super::tup::Tup;
use super::ray::Ray;

pub fn minkowski(current_ray: &Ray, current_point: Tup) -> Ray {
    Ray {o: current_point, d: current_ray.d}
}

pub fn schwartzchild(current_ray: &Ray, current_point: Tup) -> Ray {
    Ray {o: current_point, d: current_ray.d}
}