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
                //Scene: radius, position, emission, color, material
                Sphere {
                    r: 1e5,
                    p: Tup(1e5 + 1.0, 40.8, 81.6),
                    e: Tup(0., 0., 0.),
                    c: Tup(0.75, 0.25, 0.25),
                    rfl: RflType::DIFF,
                }, //Left
                Sphere {
                    r: 1e5,
                    p: Tup(-1e5 + 99., 40.8, 81.6),
                    e: Tup(0., 0., 0.),
                    c: Tup(0.25, 0.25, 0.75),
                    rfl: RflType::DIFF,
                }, //Rght
                Sphere {
                    r: 1e5,
                    p: Tup(50., 40.8, 1e5),
                    e: Tup(0., 0., 0.),
                    c: Tup(0.75, 0.75, 0.75),
                    rfl: RflType::DIFF,
                }, //Back
                Sphere {
                    r: 1e5,
                    p: Tup(50., 40.8, -1e5 + 170.),
                    e: Tup(0., 0., 0.),
                    c: Tup(0., 0., 0.),
                    rfl: RflType::DIFF,
                }, //Frnt
                Sphere {
                    r: 1e5,
                    p: Tup(50., 1e5, 81.6),
                    e: Tup(0., 0., 0.),
                    c: Tup(0.75, 0.75, 0.75),
                    rfl: RflType::DIFF,
                }, //Botm
                Sphere {
                    r: 1e5,
                    p: Tup(50., -1e5 + 81.6, 81.6),
                    e: Tup(0., 0., 0.),
                    c: Tup(0.75, 0.75, 0.75),
                    rfl: RflType::DIFF,
                }, //Top
                Sphere {
                    r: 16.5,
                    p: Tup(27.0, 16.5, 47.0),
                    e: Tup(0., 0., 0.),
                    c: Tup(1., 1., 1.) * 0.999,
                    rfl: RflType::SPEC,
                }, //Mirr
                Sphere {
                    r: 16.5,
                    p: Tup(73., 16.5, 78.),
                    e: Tup(0., 0., 0.),
                    c: Tup(1., 1., 1.) * 0.999,
                    rfl: RflType::REFR,
                }, //Glas
                Sphere {
                    r: 600.,
                    p: Tup(50., 681.6 - 0.27, 81.6),
                    e: Tup(12., 12., 12.),
                    c: Tup(0., 0., 0.),
                    rfl: RflType::DIFF,
                }, //Lite
            ],
        }
    }

    pub fn intersect(&self, ray: &Ray, t: &mut f32, id: &mut usize) -> bool {
        let n = self.spheres.len();
        let mut d: f32;
        *t = f32::INFINITY;
        for i in (0..n).rev() {
            d = self.spheres[i].intersect(ray);
            if d > 0.0 && d < *t {
                *t = d;
                *id = i;
            }
        }
        *t < f32::INFINITY
    }

    // pub fn radiance() -> Tup {}
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
    fn ray_intersects_world() {
        let world = World::new();
        let ray = Ray {
            o: Tup(0.0, 0.0, -5.0),
            d: Tup(0.0, 0.0, 1.0),
        };
        let mut t = f32::INFINITY;
        let mut id = 0;
        let its = world.intersect(&ray, &mut t, &mut id);
        assert!(its);
    }

    #[test]
    fn ray_does_not_intersect_world() {
        let world = World::new();
        let ray = Ray {
            o: Tup(0.0, 0.0, -200000.0),
            d: Tup(0.0, 0.0, 0.0),
        };
        let mut t = f32::INFINITY;
        let mut id = 0;
        let its = world.intersect(&ray, &mut t, &mut id);
        assert!(!its);
    }
}
