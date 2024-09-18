mod ray;
mod sphere;
mod tup;
mod world;

use rand::Rng;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

use num_traits::ToPrimitive;
use ray::Ray;
use tup::Tup;
use world::World;

fn main() {
    let w = 1024;
    let h = 768;
    let samps = 50;
    let cam = Ray {
        o: Tup(50., 52., 295.6),
        d: Tup(0., -0.42612, 01.).norm(),
    };
    let cx = Tup(w.to_f32().unwrap() * 0.513 / h.to_f32().unwrap(), 0., 0.);
    let cy = (cx.cross(cam.d)).norm() * 0.5135;
    let ray: Ray;
    let mut c: Vec<Tup> = vec![];
    let world = World::new();
    for y in 0..h {
        for x in 0..w {
            let seed: [u8; 32] = [1; 32];
            let mut rng = ChaCha8Rng::from_seed(seed);
            for sy in 0..2 {
                let i = (h - y - 1) * w * x;
                for sx in 0..2 {
                    let mut rad = Tup(0., 0., 0.);
                    for s in 0..samps {
                        let r1: f32 = 2. * rng.gen_range(0.0..1.);
                        let dx: f32;
                        if r1 < 1. {
                            dx = r1.sqrt() - 1.;
                        } else {
                            dx = 1. - (2. - r1).sqrt();
                        }
                        let r2: f32 = 2. * rng.gen_range(0.0..1.);
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
                                seed,
                            );
                    }
                }
            }
        }
    }
}
