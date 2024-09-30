use std::f64::consts::PI;

use rand::{rngs::ThreadRng, Rng};

use crate::{
    ray::Ray,
    sphere::{RflType, Sphere},
    tup::Tup,
    world::World,
};

#[derive(Default)]
pub enum IntegrationType {
    #[default]
    Iterative,
    #[allow(dead_code)]
    Recursive,
}

pub fn integrate(
    world: &World,
    ray: Ray,
    depth: i32,
    rng: &mut ThreadRng,
    int_type: IntegrationType,
) -> Tup {
    match int_type {
        IntegrationType::Iterative => radiance_iter(world, ray, depth, rng),
        IntegrationType::Recursive => radiance(world, &ray, depth, rng),
    }
}

pub fn radiance(world: &World, ray: &Ray, mut depth: i32, mut rng: &mut ThreadRng) -> Tup {
    let mut t = f64::INFINITY;
    let mut id: usize = 0;
    if !world.intersect(&ray, &mut t, &mut id) {
        return Tup(0., 0., 0.);
    }
    let obj: &Sphere = &world.spheres[id];
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
            let w: Tup = n1;
            let u: Tup = if w.0.abs() > 0.1 {
                Tup(0., 1., 0.).cross(w).norm()
            } else {
                Tup(1., 0., 0.).cross(w).norm()
            };

            let v = w.cross(u);
            let d: Tup =
                (u * f64::cos(r1) * r2s + v * f64::sin(r1) * r2s + w * ((1. - r2).sqrt())).norm();
            return obj.e + f * radiance(world, &Ray { o: x, d }, depth, &mut rng);
        }
        RflType::SPEC => {
            return obj.e
                + f * radiance(
                    world,
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
                return obj.e + f * radiance(world, &rfl_ray, depth, &mut rng);
            }
            let tdir =
                (ray.d * nnt - n * if into { 1. } else { -1. } * (ddn * nnt + cos2t.sqrt())).norm();
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
                    if rng.gen::<f64>() < p {
                        radiance(world, &rfl_ray, depth, &mut rng) * rp
                    } else {
                        radiance(world, &Ray { o: x, d: tdir }, depth, &mut rng) * tp
                    }
                } else {
                    radiance(world, &rfl_ray, depth, &mut rng) * re
                        + radiance(world, &Ray { o: x, d: tdir }, depth, &mut rng) * tr
                })
        }
    }
}

pub fn radiance_iter(world: &World, mut ray: Ray, mut depth: i32, rng: &mut ThreadRng) -> Tup {
    let mut result = Tup::zeros();
    let mut throughput = Tup::ones();

    loop {
        let mut t = f64::INFINITY;
        let mut id: usize = 0;
        if !world.intersect(&ray, &mut t, &mut id) {
            return result;
        }

        let obj: &Sphere = &world.spheres[id];
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
                result += throughput * obj.e;
                break;
            }
        }

        result += throughput * obj.e;
        throughput = throughput * f;

        match obj.rfl {
            RflType::DIFF => {
                let r1 = 2. * PI * rng.gen::<f64>();
                let r2: f64 = rng.gen();
                let r2s = r2.sqrt();
                let w: Tup = n1;
                let u: Tup = if w.0.abs() > 0.1 {
                    Tup(0., 1., 0.).cross(w).norm()
                } else {
                    Tup(1., 0., 0.).cross(w).norm()
                };
                let v = w.cross(u);
                let d: Tup =
                    (u * f64::cos(r1) * r2s + v * f64::sin(r1) * r2s + w * ((1. - r2).sqrt()))
                        .norm();

                ray = Ray { o: x, d };
            }
            RflType::SPEC => {
                ray = Ray {
                    o: x,
                    d: ray.d - n * 2. * n.dot(ray.d),
                };
            }
            RflType::REFR => {
                let into = n.dot(n1) > 0.;
                let nc: f64 = 1.;
                let nt: f64 = 1.5;
                let nnt = if into { nc / nt } else { nt / nc };
                let ddn = ray.d.dot(n1);
                let cos2t = 1. - nnt * nnt * (1. - ddn * ddn);

                if cos2t < 0. {
                    ray = Ray {
                        o: x,
                        d: ray.d - n * 2. * n.dot(ray.d),
                    };
                    continue;
                }

                let tdir = (ray.d * nnt
                    - n * if into { 1. } else { -1. } * (ddn * nnt + cos2t.sqrt()))
                .norm();

                let a = nt - nc;
                let b = nt + nc;
                let r0 = (a * a) / (b * b);
                let c = 1. - if into { -ddn } else { tdir.dot(n) };
                let re = r0 + (1. - r0) * c * c * c * c * c;
                let tr = 1. - re;
                let p = 0.25 + 0.5 * re;
                let rp = re / p;
                let tp = tr / (1. - p);

                if rng.gen::<f64>() < p {
                    ray = Ray {
                        o: x,
                        d: ray.d - n * 2. * n.dot(ray.d),
                    };
                    throughput = throughput * rp;
                } else {
                    ray = Ray { o: x, d: tdir };
                    throughput = throughput * tp;
                }

                // if depth > 2 {
                //     if rng.gen::<f64>() < p {
                //         ray = Ray {
                //             o: x,
                //             d: ray.d - n * 2. * n.dot(ray.d),
                //         };
                //         throughput = throughput * rp;
                //     } else {
                //         ray = Ray { o: x, d: tdir };
                //         throughput = throughput * tp;
                //     }
                // } else {
                //     let reflected_dir = ray.d - n * 2. * n.dot(ray.d);
                //     let reflected_part = throughput * re;
                //     let refracted_part = throughput * tr;

                //     result += reflected_part
                //         * radiance(
                //             world,
                //             &Ray {
                //                 o: x,
                //                 d: reflected_dir,
                //             },
                //             depth,
                //             &mut rng,
                //         );
                //     result +=
                //         refracted_part * radiance(world, &Ray { o: x, d: tdir }, depth, &mut rng);
                //     break;
                // }
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {

    use rand::thread_rng;

    use super::*;
    #[test]
    fn ray_intesects_empty_world() {
        let ray = Ray {
            o: Tup(0., 0., 0.),  // Origin
            d: Tup(0., 0., -1.), // Direction pointing away from any spheres
        };
        let world = World { spheres: vec![] };
        let mut rng = thread_rng();

        let result = radiance(&world, &ray, 0, &mut rng);
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

        let result = radiance(&world, &ray, 0, &mut rng);
        assert_eq!(result, Tup(1., 0., 0.));
    }
}
