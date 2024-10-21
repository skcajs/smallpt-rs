use core::f64;

use super::ray::Ray;
use super::sphere::{RflType, Sphere};
use super::tup::Tup;

pub struct World {
    pub spheres: Vec<Sphere>,
}

impl World {
    pub fn new() -> Self {
        World {
            spheres: vec![
                // Scene: radius, position, emission, color, material
                Sphere::new(
                    1e5,
                    Tup(1e5 + 1.0, 40.8, 81.6),
                    Tup::zeros(),
                    Tup(0.75, 0.25, 0.25),
                    RflType::DIFF,
                ), // Left
                Sphere::new(
                    1e5,
                    Tup(-1e5 + 99., 40.8, 81.6),
                    Tup::zeros(),
                    Tup(0.25, 0.25, 0.75),
                    RflType::DIFF,
                ), // Right
                Sphere::new(
                    1e5,
                    Tup(50., 40.8, 1e5),
                    Tup::zeros(),
                    Tup(0.75, 0.75, 0.75),
                    RflType::DIFF,
                ), // Back
                Sphere::new(
                    1e5,
                    Tup(50., 40.8, -1e5 + 170.),
                    Tup::zeros(),
                    Tup::zeros(),
                    RflType::DIFF,
                ), // Front
                Sphere::new(
                    1e5,
                    Tup(50., 1e5, 81.6),
                    Tup::zeros(),
                    Tup(0.75, 0.75, 0.75),
                    RflType::DIFF,
                ), // Bottom
                Sphere::new(
                    1e5,
                    Tup(50., -1e5 + 81.6, 81.6),
                    Tup::zeros(),
                    Tup(0.75, 0.75, 0.75),
                    RflType::DIFF,
                ), // Top
                Sphere::new(
                    16.5,
                    Tup(27.0, 16.5, 47.0),
                    Tup::zeros(),
                    Tup(1., 1., 1.) * 0.999,
                    RflType::SPEC,
                ), // Mirror
                Sphere::new(
                    16.5,
                    Tup(73., 16.5, 78.),
                    Tup::zeros(),
                    Tup(1., 1., 1.) * 0.999,
                    RflType::REFR,
                ), // Glass
                Sphere::new(
                    600.,
                    Tup(50., 681.6 - 0.27, 81.6),
                    Tup(12., 12., 12.),
                    Tup::zeros(),
                    RflType::DIFF,
                ), // Light
            ],
        }
    }

    pub fn intersect(&self, ray: &Ray, t: &mut f64, id: &mut usize) -> bool {
        *t = f64::INFINITY;
        for i in (0..self.spheres.len()).rev() {
            let d = self.spheres[i].intersect(ray);
            if d != 0.0 && d < *t {
                *t = d;
                *id = i;
            }
        }
        *t < f64::INFINITY
    }

    pub fn trace_geodesic(&self, ray: &Ray, t: &mut f64, id: &mut usize) -> bool {
        let max_iter = 1000;
        let mut dt = 10.0;
        let sigma = 0.1;

        let mut current_ray = ray.clone();

        for _ in 0..max_iter {   
            *t = f64::INFINITY;
            for i in (0..self.spheres.len()).rev() {
                let d = self.spheres[i].intersect(&current_ray);
                if d != 0.0 && d < *t {
                    *t = d;
                    *id = i;
                }
            }

            let next_ray = self.trace_step(&current_ray, dt); // This is currently linear, soon will be curved
            if self.has_intersected(&current_ray, &next_ray, t, id) {
                if *t < sigma {
                    return true;
                }
                dt /= 2.0;
                continue;
            }

            current_ray = next_ray;
        }
        false
    }

    fn trace_step(&self, ray: & Ray, dt: f64) -> Ray {
        let o = ray.o + ray.d * dt;
        Ray {
            o,
            d: o - ray.o
        }
    }

    fn has_intersected(&self, current_ray: &Ray, next_ray: &Ray, t: &f64,  id: &usize) -> bool {
        let n = self.spheres[*id].normal_at(current_ray.o + current_ray.d * *t);
        let current_ray_dot_n = current_ray.o.dot(n).signum();
        let next_ray_dot_n = next_ray.o.dot(n).signum();
        current_ray_dot_n != next_ray_dot_n
    }

    // pub fn trace_geodesic(&self, ray: &Ray, t: &mut f64, id: &mut usize) -> bool {
    //     let max_distance: f64 = 4000.0;
    //     let step_size: f64 = 0.5;
    //     let mut current_distance = 0.0;
    //     let sigma = 0.01;

    //     while current_distance < max_distance {
    //         let current_point = ray.o + ray.d * current_distance;

    //         *t = f64::INFINITY;
    //         for i in (0..self.spheres.len()).rev() {
    //             let d = self.spheres[i].intersect(&Ray { o: current_point, d: ray.d });
    //             if d != 0.0 && d < *t {
    //                 *t = d;
    //                 *id = i;
    //             }
    //         }

            
    //     }
        

    //     *t < f64::INFINITY
    // }

    // pub fn trace_geodesic(&self, ray: &Ray, t: &mut f64, id: &mut usize) -> bool {
    //     *t = f64::INFINITY;
    //     let max_distance: f64 = 4000.0; 
    //     let step_size: f64 = 0.5;
    //     let mut current_distance = 0.0;
    
    //     while current_distance < max_distance {
    //         // Compute the current point along the ray
    //         let current_point = ray.o + ray.d * current_distance;
    
    //         // Check for intersections with each sphere
    //         for i in (0..self.spheres.len()).rev() {
    //             let d = self.spheres[i].intersect(&Ray { o: current_point, d: ray.d });
    //             if d != 0.0 && d < step_size {
    //                 *t = current_distance;
    //                 *id = i;
    //                 return true; // Intersection found
    //             }
    //         }
    
    //         // Increment the current distance by the step size
    //         current_distance += step_size;
    //     }
    
    //     *t = f64::INFINITY;
    //     false // No intersection found within max_distance
    // }

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn new_world() {
        let sphere = Sphere {
            r: 1e5,
            p: Tup(1e5 + 1.0, 40.8, 81.6),
            e: Tup(0., 0., 0.),
            c: Tup(0.75, 0.25, 0.25),
            rfl: RflType::DIFF,
        };
        let world = World::new();
        assert_eq!(world.spheres.len(), 9);
        assert_eq!(world.spheres[0], sphere)
    }

    #[test]
    fn ray_intersects() {
        let world = World::new();
        let ray = Ray {
            o: Tup(0.0, 0.0, -5.0),
            d: Tup(0.0, 0.0, 1.0),
        };
        let mut t = f64::INFINITY;
        let mut id = 0;
        let its = world.intersect(&ray, &mut t, &mut id);
        assert!(its);
    }

    #[test]
    fn ray_does_not_intersect() {
        let world = World::new();
        let ray = Ray {
            o: Tup(0.0, 0.0, -200000.0),
            d: Tup(0.0, 0.0, 0.0),
        };
        let mut t = f64::INFINITY;
        let mut id = 0;
        let its = world.intersect(&ray, &mut t, &mut id);
        assert!(!its);
    }
}
