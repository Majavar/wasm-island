use super::{utils::normalize, Heightmap, Noise, NoiseType};
use rand::Rng;
use std::cell::OnceCell;

pub struct Fractal<'a, R, F>
where
    R: Rng,
    F: FnMut(&mut R) -> NoiseType,
{
    pub noise_cell: &'a OnceCell<NoiseType>,
    pub get_noise: F,
    pub octave: u64,
    pub lacunarity: f64,
    pub persistance: f64,
    pub rng: &'a mut R,
}

impl<'a, R, F> Fractal<'a, R, F>
where
    R: Rng,
    F: FnMut(&mut R) -> NoiseType,
{
    fn get(&self, x: f64, y: f64, noise: &NoiseType) -> f64 {
        let mut value = 0.0;
        let mut frequency = 1.0;
        let mut amplitude = 1.0;

        for _ in 0..self.octave {
            value += noise.noise(x * frequency, y * frequency) * amplitude;
            frequency *= self.lacunarity;
            amplitude *= self.persistance;
        }

        value
    }
}
impl<'a, R, F> Heightmap for Fractal<'a, R, F>
where
    R: Rng,
    F: FnMut(&mut R) -> NoiseType,
{
    fn generate(&mut self, width: usize, height: usize) -> Vec<f64> {
        let wf = width as f64;
        let hf = height as f64;
        let x_ratio = (wf / hf).max(1.0);
        let y_ratio = (hf / wf).max(1.0);
        let noise = self.noise_cell.get_or_init(|| (self.get_noise)(self.rng));

        normalize(
            (0..height)
                .flat_map(|y| (0..width).map(move |x| (x, y)))
                .map(|(x, y)| self.get((x as f64 / wf) * x_ratio, (y as f64 / hf) * y_ratio, noise))
                .collect(),
        )
    }
}
