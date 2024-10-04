use rand::{rngs::ThreadRng, thread_rng, Rng};

pub struct Sampler {
    rng: ThreadRng
}

impl Sampler {
    pub fn new() -> Self {
        Sampler {
            rng: thread_rng()
        }
    }

    pub fn next(&mut self) -> f64 {
        self.rng.gen::<f64>()
    }

    pub fn next_2d(&mut self) -> (f64, f64) {
        (self.rng.gen::<f64>(), self.rng.gen::<f64>())
    }
}