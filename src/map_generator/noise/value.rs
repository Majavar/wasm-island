use super::Noise;
use crate::map_generator::Interpolation;
use rand::{
    distributions::{Distribution, Uniform},
    seq::SliceRandom,
    Rng,
};

pub struct Value {
    permutation: [u8; 256],
    values: [f64; 256],
    interpolation: fn(f64, f64, f64) -> f64,
}

impl Value {
    pub fn new<R: Rng>(rng: &mut R, interpolation: Interpolation) -> Self {
        let mut permutation = [0; 256];
        for (i, x) in permutation.iter_mut().enumerate() {
            *x = i as u8;
        }
        permutation.shuffle(rng);

        let distribution = Uniform::from(0.0..=1.0);
        let mut values = [0.0; 256];
        for i in values.iter_mut() {
            *i = distribution.sample(rng);
        }

        Self {
            permutation,
            values,
            interpolation: *interpolation,
        }
    }

    fn index(&self, x: usize, y: usize) -> usize {
        self.permutation[(self.permutation[x & 0xFF] as usize + y) & 0xFF] as usize
    }
}

impl Noise for Value {
    fn noise(&self, x: f64, y: f64) -> f64 {
        let xint = x as usize;
        let yint = y as usize;

        let nw = self.values[self.index(xint, yint)];
        let ne = self.values[self.index(xint + 1, yint)];
        let sw = self.values[self.index(xint, yint + 1)];
        let se = self.values[self.index(xint + 1, yint + 1)];

        let xf = x.fract();
        let yf = y.fract();

        let n = (self.interpolation)(nw, ne, xf);
        let s = (self.interpolation)(sw, se, xf);

        (self.interpolation)(n, s, yf)
    }
}
