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

    pub fn intersect(ray: &Ray, t: &mut f32, id: &mut isize) -> bool {
        true
    }

    pub fn radiance() -> Tup {}
}
