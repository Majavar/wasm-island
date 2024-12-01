use super::{
    utils::{normalize, submap},
    Heightmap,
};
use rand::{
    distributions::{Distribution, Uniform},
    Rng,
};
use std::cmp::max;

pub struct Diamond<'a, R>
where
    R: Rng,
{
    pub rng: &'a mut R,
}

impl<'a, R> Diamond<'a, R>
where
    R: Rng,
{
    fn square<D: Distribution<f64>>(
        &mut self,
        data: &mut [f64],
        x: usize,
        y: usize,
        size: usize,
        d: usize,
        sampler: D,
    ) {
        let idx = |x, y| x + y * size;

        let tl = data[idx(x - d, y - d)];
        let tr = data[idx(x - d, y + d)];
        let bl = data[idx(x + d, y - d)];
        let br = data[idx(x + d, y + d)];

        data[idx(x, y)] = (tl + tr + bl + br) / 4.0 + sampler.sample(self.rng);
    }

    fn diamond<D: Distribution<f64>>(
        &mut self,
        data: &mut [f64],
        x: usize,
        y: usize,
        size: usize,
        d: usize,
        sampler: D,
    ) {
        let mut sum = 0.0;
        let mut count = 0.0;
        let idx = |x, y| x + y * size;

        if x > 0 {
            sum += data[idx(x - d, y)];
            count += 1.0;
        }
        if x < size - 1 {
            sum += data[idx(x + d, y)];
            count += 1.0;
        }
        if y > 0 {
            sum += data[idx(x, y - d)];
            count += 1.0;
        }
        if y < size - 1 {
            sum += data[idx(x, y + d)];
            count += 1.0;
        }

        data[idx(x, y)] = sum / count + sampler.sample(self.rng);
    }
}

impl<'a, R> Heightmap for Diamond<'a, R>
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
                    self.square(&mut data, x, y, size, half, sampler);
                }
            }

            for x in (half..size).step_by(d) {
                for y in (half..size).step_by(d) {
                    self.diamond(&mut data, x - half, y, size, half, sampler);
                    self.diamond(&mut data, x + half, y, size, half, sampler);
                    self.diamond(&mut data, x, y - half, size, half, sampler);
                    self.diamond(&mut data, x, y + half, size, half, sampler);
                }
            }

            d = half;
            delta /= 2.0;
        }

        normalize(submap(size, size, width, height, data))
    }
}
