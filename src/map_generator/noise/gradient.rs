use core::f64;

use super::Noise;
use crate::map_generator::Interpolation;
use rand::{
    distributions::{Distribution, Uniform},
    seq::SliceRandom,
    Rng,
};

#[derive(Debug, Clone, Copy)]
struct Vector {
    x: f64,
    y: f64,
}

impl Vector {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    fn zero() -> Self {
        Self::new(0.0, 0.0)
    }

    fn dot(&self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y
    }
}

pub struct Gradient {
    permutation: [u8; 256],
    gradients: [Vector; 256],
    interpolation: fn(f64, f64, f64) -> f64,
}

impl Gradient {
    pub fn new<R: Rng>(rng: &mut R, interpolation: Interpolation) -> Self {
        let mut permutation = [0; 256];
        for (i, x) in permutation.iter_mut().enumerate() {
            *x = i as u8;
        }
        permutation.shuffle(rng);

        let distribution = Uniform::from(0.0..f64::consts::PI * 2.0);
        let mut gradients = [Vector::zero(); 256];
        for g in gradients.iter_mut() {
            let (s, c) = distribution.sample(rng).sin_cos();
            *g = Vector::new(c, s);
        }

        Self {
            permutation,
            gradients,
            interpolation: *interpolation,
        }
    }

    fn index(&self, x: usize, y: usize) -> usize {
        self.permutation[(self.permutation[x & 0xFF] as usize + y) & 0xFF] as usize
    }
}

impl Noise for Gradient {
    fn noise(&self, x: f64, y: f64) -> f64 {
        let xint = x as usize;
        let yint = y as usize;

        let xf = x.fract();
        let yf = y.fract();

        let nw = self.gradients[self.index(xint, yint)].dot(Vector::new(xf, yf));
        let ne = self.gradients[self.index(xint + 1, yint)].dot(Vector::new(xf - 1.0, yf));
        let sw = self.gradients[self.index(xint, yint + 1)].dot(Vector::new(xf, yf - 1.0));
        let se =
            self.gradients[self.index(xint + 1, yint + 1)].dot(Vector::new(xf - 1.0, yf - 1.0));

        let n = (self.interpolation)(nw, ne, xf);
        let s = (self.interpolation)(sw, se, xf);

        (self.interpolation)(n, s, yf) / f64::consts::SQRT_2 + 0.5
    }
}
