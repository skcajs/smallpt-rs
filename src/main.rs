mod ray;
mod sphere;
mod tup;
mod world;

use std::fs::File;
use std::io::Write;

use rand::{thread_rng, Rng};
// use rayon::prelude::*;

use ray::Ray;
use tup::Tup;
use world::World;

fn clamp(x: f64) -> f64 {
    if x < 0. {
        return 0.;
    } else if x > 1. {
        return 1.;
    }
    x
}

fn to_int(x: f64) -> i32 {
    (clamp(x).powf(1. / 2.2) * 255. + 0.5) as i32
}

fn main() {
    let w = 640;
    let h = 480;
    let samps: isize = 50;
    let cam = Ray {
        o: Tup(50., 52., 295.6),
        d: Tup(0., -0.042612, -1.).norm(),
    };

    let cx = Tup(w as f64 * 0.5135 / h as f64, 0.0, 0.0);
    let cy = (cx.cross(cam.d)).norm() * 0.5135;
    let mut c = vec![Tup(0.0, 0.0, 0.0); (w * h) as usize];

    let world = World::new();
    let mut rng = thread_rng();

    (0..h).into_iter().for_each(|y| {
        (0..w).into_iter().for_each(|x| {
            let i = (h - y - 1) * w + x;
            for sy in 0..2 {
                for sx in 0..2 {
                    let mut rad = Tup(0., 0., 0.);
                    for _ in 0..samps {
                        let r1: f64 = 2. * rng.gen::<f64>();
                        let dx = if r1 < 1. {
                            r1.sqrt() - 1.
                        } else {
                            1. - (2. - r1).sqrt()
                        };

                        let r2: f64 = 2. * rng.gen::<f64>();
                        let dy = if r2 < 1. {
                            r2.sqrt() - 1.
                        } else {
                            1. - (2. - r2).sqrt()
                        };

                        let d = cx * (((sx as f64 + 0.5 + dx) / 2. + x as f64) / w as f64 - 0.5)
                            + cy * (((sy as f64 + 0.5 + dy) / 2. + y as f64) / h as f64 - 0.5)
                            + cam.d;

                        // rad = rad
                        //     + world.radiance(
                        //         &Ray {
                        //             o: cam.o + d * 140.,
                        //             d: d.norm(),
                        //         },
                        //         0,
                        //         &mut rng,
                        //     ) * (1. / samps as f64);

                        rad = rad
                            + world.radiance_iter(
                                Ray {
                                    o: cam.o + d * 140.,
                                    d: d.norm(),
                                },
                                0,
                                &mut rng,
                            ) * (1. / samps as f64);
                    }

                    c[i] = c[i] + Tup(clamp(rad.0), clamp(rad.1), clamp(rad.2)) * 0.25;
                }
            }
        });
    });

    let mut f = File::create("image.ppm").unwrap();
    writeln!(f, "P3\n{} {}\n255", w, h).unwrap();
    for i in 0..w * h {
        writeln!(
            f,
            "{} {} {}",
            to_int(c[i].0),
            to_int(c[i].1),
            to_int(c[i].2)
        )
        .unwrap();
    }
}
