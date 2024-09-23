use core::f64;
use std::f64::consts::PI;

use rand::rngs::ThreadRng;
use rand::Rng;

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
                    Tup::zero(),
                    Tup(0.75, 0.25, 0.25),
                    RflType::DIFF,
                ), // Left
                Sphere::new(
                    1e5,
                    Tup(-1e5 + 99., 40.8, 81.6),
                    Tup::zero(),
                    Tup(0.25, 0.25, 0.75),
                    RflType::DIFF,
                ), // Right
                Sphere::new(
                    1e5,
                    Tup(50., 40.8, 1e5),
                    Tup::zero(),
                    Tup(0.75, 0.75, 0.75),
                    RflType::DIFF,
                ), // Back
                Sphere::new(
                    1e5,
                    Tup(50., 40.8, -1e5 + 170.),
                    Tup::zero(),
                    Tup::zero(),
                    RflType::DIFF,
                ), // Front
                Sphere::new(
                    1e5,
                    Tup(50., 1e5, 81.6),
                    Tup::zero(),
                    Tup(0.75, 0.75, 0.75),
                    RflType::DIFF,
                ), // Bottom
                Sphere::new(
                    1e5,
                    Tup(50., -1e5 + 81.6, 81.6),
                    Tup::zero(),
                    Tup(0.75, 0.75, 0.75),
                    RflType::DIFF,
                ), // Top
                Sphere::new(
                    16.5,
                    Tup(27.0, 16.5, 47.0),
                    Tup::zero(),
                    Tup(1., 1., 1.) * 0.999,
                    RflType::SPEC,
                ), // Mirror
                Sphere::new(
                    16.5,
                    Tup(73., 16.5, 78.),
                    Tup::zero(),
                    Tup(1., 1., 1.) * 0.999,
                    RflType::REFR,
                ), // Glass
                Sphere::new(
                    600.,
                    Tup(50., 681.6 - 0.27, 81.6),
                    Tup(12., 12., 12.),
                    Tup::zero(),
                    RflType::DIFF,
                ), // Light
            ],
        }
    }

    pub fn intersect(&self, ray: &Ray, t: &mut f64, id: &mut usize) -> bool {
        let n = self.spheres.len();
        let mut d: f64;
        *t = f64::INFINITY;
        for i in (0..n).rev() {
            d = self.spheres[i].intersect(ray);
            if d > 0.0 && d < *t {
                *t = d;
                *id = i;
            }
        }
        *t < f64::INFINITY
    }

    pub fn radiance(&self, ray: &Ray, mut depth: i32, mut rng: &mut ThreadRng) -> Tup {
        let mut t = f64::INFINITY;
        let mut id: usize = 0;
        if !self.intersect(&ray, &mut t, &mut id) {
            return Tup(0., 0., 0.);
        }
        let obj: &Sphere = &self.spheres[id];
        let x = ray.o + (ray.d * t);
        let n = (x - obj.p).norm();
        let n1 = if n.dot(ray.d) < 0.0 { n } else { n * -1.0 };

        let mut f = obj.c;
        let p = f.0.max(f.1.max(f.2));
        depth += 1;
        if depth > 5 {
            if rng.gen::<f64>() < p {
                f = f * (1.0 / p);
            } else {
                return obj.e;
            }
        }

        match obj.rfl {
            RflType::DIFF => {
                let r1 = 2. * PI * rng.gen::<f64>();
                let r2: f64 = rng.gen();
                let r2s = r2.sqrt();
                let w: Tup = n;
                let u: Tup = if w.0.abs() > 0.1 {
                    Tup(0., 1., 0.).cross(w).norm()
                } else {
                    Tup(1., 0., 0.).cross(w).norm()
                };

                let v = w.cross(u);
                let d: Tup =
                    (u * f64::cos(r1) * r2s + v * f64::sin(r1) * r2s + w * ((1. - r2).sqrt()))
                        .norm();
                return obj.e + f * self.radiance(&Ray { o: x, d }, depth, &mut rng);
            }
            RflType::SPEC => {
                return obj.e
                    + f * self.radiance(
                        &Ray {
                            o: x,
                            d: ray.d - n * 2. * n.dot(ray.d),
                        },
                        depth,
                        &mut rng,
                    );
            }
            RflType::REFR => {
                let rfl_ray = Ray {
                    o: x,
                    d: ray.d - n * 2. * n.dot(ray.d),
                };
                let into = n.dot(n1) > 0.;
                let nc: f64 = 1.;
                let nt: f64 = 1.5;
                let nnt = if into { nc / nt } else { nt / nc };
                let ddn = ray.d.dot(n1);
                let cos2t = 1. - nnt * nnt * (1. - ddn * ddn);
                if cos2t < 0. {
                    return obj.e + f * self.radiance(&rfl_ray, depth, &mut rng);
                }
                let tdir = (ray.d * nnt
                    - n * if into { 1. } else { -1. } * (ddn * nnt + cos2t.sqrt()))
                .norm();
                let a = nt - nc;
                let b = nt + nc;
                let r0 = a * a / (b * b);
                let c = 1. - if into { -ddn } else { tdir.dot(n) };
                let re = r0 + (1. - r0) * c * c * c * c * c;
                let tr = 1. - re;
                let p = 0.25 + 0.5 * re;
                let rp = re / p;
                let tp = tr / (1. - p);

                obj.e
                    + f * (if depth > 2 {
                        if rng.gen_range(0.0..1.0) < p {
                            self.radiance(&rfl_ray, depth, &mut rng) * rp
                        } else {
                            self.radiance(&Ray { o: x, d: tdir }, depth, &mut rng) * tp
                        }
                    } else {
                        self.radiance(&rfl_ray, depth, &mut rng) * re
                            + self.radiance(&Ray { o: x, d: tdir }, depth, &mut rng) * tr
                    })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use rand::thread_rng;

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

    #[test]
    fn ray_intesects_empty_world() {
        let ray = Ray {
            o: Tup(0., 0., 0.),  // Origin
            d: Tup(0., 0., -1.), // Direction pointing away from any spheres
        };
        let world = World { spheres: vec![] };
        let mut rng = thread_rng();

        let result = world.radiance(&ray, 0, &mut rng);
        assert_eq!(result, Tup(0., 0., 0.));
    }

    #[test]
    fn ray_intesects_single_sphere_world() {
        let sphere = Sphere::new(
            1.0,
            Tup(0., 0., -5.),
            Tup(1., 0., 0.),
            Tup(0., 0., 0.),
            RflType::DIFF,
        );
        let world = World {
            spheres: vec![sphere],
        };
        let ray = Ray {
            o: Tup(0., 0., 0.),  // Origin
            d: Tup(0., 0., -1.), // Direction pointing away from any spheres
        };
        let mut rng = thread_rng();

        let result = world.radiance(&ray, 0, &mut rng);
        assert_eq!(result, Tup(1., 0., 0.));
    }
}
