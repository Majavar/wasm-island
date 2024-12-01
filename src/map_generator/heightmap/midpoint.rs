use super::{
    utils::{normalize, submap},
    Heightmap,
};
use rand::{
    distributions::{Distribution, Uniform},
    Rng,
};
use std::cmp::max;

pub struct Midpoint<'a, R>
where
    R: Rng,
{
    pub rng: &'a mut R,
}

impl<'a, R> Heightmap for Midpoint<'a, R>
where
    R: Rng,
{
    fn generate(&mut self, width: usize, height: usize) -> Vec<f64> {
        let sampler = Uniform::from(0.0..=1.0);
        let size = (max(width, height) - 1).next_power_of_two() + 1;
        let mut data = vec![0.0; size * size];
        let idx = |x, y| x + y * size;

        data[idx(0, 0)] = sampler.sample(self.rng);
        data[idx(size - 1, 0)] = sampler.sample(self.rng);
        data[idx(0, size - 1)] = sampler.sample(self.rng);
        data[idx(size - 1, size - 1)] = sampler.sample(self.rng);

        let mut d = size - 1;
        let mut delta = 0.5;
        while d > 1 {
            let half = d / 2;
            let sampler = Uniform::from(-delta..=delta);

            for x in (half..size).step_by(d) {
                for y in (half..size).step_by(d) {
                    let tl = data[idx(x - half, y - half)];
                    let tr = data[idx(x - half, y + half)];
                    let bl = data[idx(x + half, y - half)];
                    let br = data[idx(x + half, y + half)];

                    data[idx(x, y)] = (tl + tr + bl + br) / 4.0 + sampler.sample(self.rng);
                    data[idx(x - half, y)] = (tl + tr) / 2.0 + sampler.sample(self.rng);
                    data[idx(x + half, y)] = (bl + br) / 2.0 + sampler.sample(self.rng);
                    data[idx(x, y - half)] = (tl + bl) / 2.0 + sampler.sample(self.rng);
                    data[idx(x, y + half)] = (tr + br) / 2.0 + sampler.sample(self.rng);
                }
            }

            d = half;
            delta /= 2.0;
        }

        normalize(submap(size, size, width, height, data))
    }
}
