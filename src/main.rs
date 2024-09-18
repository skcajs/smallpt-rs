mod ray;
mod sphere;
mod tup;
mod world;

use std::fs::File;
use std::io::Write;

use rand::{thread_rng, Rng};

use num_traits::ToPrimitive;
use ray::Ray;
use tup::Tup;
use world::World;

fn clamp(x: f32) -> f32 {
    if x < 0. {
        return 0.;
    } else if x > 1. {
        return 1.;
    }
    x
}

fn to_int(x: f32) -> i32 {
    (clamp(x).powf(1. / 2.2) * 255. + 0.5).to_i32().unwrap()
}

fn main() {
    let w = 640;
    let h = 480;
    let samps = 20;
    let cam = Ray {
        o: Tup(50., 52., 295.6),
        d: Tup(0., -0.42612, 01.).norm(),
    };
    let cx = Tup(w.to_f32().unwrap() * 0.513 / h.to_f32().unwrap(), 0., 0.);
    let cy = (cx.cross(cam.d)).norm() * 0.5135;
    let mut c = vec![Tup(0., 0., 0.); w * h];
    let world = World::new();
    for y in 0..h {
        let mut rng = thread_rng();
        for x in 0..w {
            for sy in 0..2 {
                let i = (h - y - 1) * w + x;
                for sx in 0..2 {
                    let mut rad = Tup(0., 0., 0.);
                    for _ in 0..samps {
                        let r1: f32 = 2. * rng.gen::<f32>();
                        let dx: f32;
                        if r1 < 1. {
                            dx = r1.sqrt() - 1.;
                        } else {
                            dx = 1. - (2. - r1).sqrt();
                        }
                        let r2: f32 = 2. * rng.gen::<f32>();
                        let dy: f32;
                        if r2 < 1. {
                            dy = r2.sqrt() - 1.;
                        } else {
                            dy = 1. - (2. - r2).sqrt();
                        }
                        let d = cx
                            * (((sx.to_f32().unwrap() + 0.5 + dx) / 2. + x.to_f32().unwrap())
                                / w.to_f32().unwrap()
                                - 0.5)
                            + cy * (((sy.to_f32().unwrap() + 0.5 + dy) / 2. + y.to_f32().unwrap())
                                / h.to_f32().unwrap()
                                - 0.5);
                        rad = rad
                            + world.radiance(
                                &Ray {
                                    o: cam.o + d * 140.to_f32().unwrap(),
                                    d: d.norm(),
                                },
                                0,
                                &mut rng,
                            ) * (1. / samps.to_f32().unwrap());
                    }

                    c[i] = c[i] + Tup(clamp(rad.0), clamp(rad.1), clamp(rad.2)) * 0.25;
                }
            }
        }
    }

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

    // let mut ppm = vec![format!("P3\n{} {}\n{}\n", w, h, 255)];
    // // for i in c {
    // //     ppm.push(format!("{} {} {}", to_int(i.0), to_int(i.1), to_int(i.2),));
    // // }
    // let encoded_ppm = bincode::serialize(&ppm).expect("Could not encode vector");
    // fs::write("image.txt", encoded_ppm).expect("could not write ppm file");
}
