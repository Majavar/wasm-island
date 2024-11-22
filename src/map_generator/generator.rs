use super::{
    interpolation::Interpolation,
    noise::{Noise, NoiseKind, NoiseType},
};
use rand::{rngs::StdRng, SeedableRng};

pub struct Generator {
    seed: u32,
    interpolation: Interpolation,
    noise: NoiseType,
}

impl Generator {
    pub fn build() -> Builder {
        Builder::default()
    }

    pub fn update_seed(&mut self, seed: u32) {
        self.seed = seed;
        self.reseed();
        
    }

    pub fn update_interpolation(&mut self, interpolation: Interpolation) {
        self.interpolation = interpolation;
        self.reseed();
    }

    pub fn update_noise(&mut self, noise: NoiseKind) {
        let mut rng = StdRng::seed_from_u64(self.seed as u64);
        self.noise = noise.into(&mut rng, self.interpolation);
    }

    pub fn generate(&self, width: usize, height: usize) -> Vec<u8> {
        let mut image = Vec::with_capacity(4 * (width * height));

        for y in 0..height {
            for x in 0..width {
                let value = self.noise.noise(x as f32 / 32.0, y as f32 / 32.0);
                for _ in 0..4 {
                    image.push((value * 255.0) as u8);
                }
            }
        }

        image
    }

    fn reseed(&mut self) {
        let mut rng = StdRng::seed_from_u64(self.seed as u64);
        self.noise.reseed(&mut rng, self.interpolation);
    }
}

pub struct Builder {
    seed: u32,
    interpolation: Interpolation,
    noise: NoiseKind,
}

impl Default for Builder {
    fn default() -> Self {
        Self {
            seed: u32::default(),
            interpolation: Interpolation::default(),
            noise: NoiseKind::default(),
        }
    }
}

impl Builder {
    pub fn seed(self, seed: u32) -> Self {
        Self { seed, ..self }
    }

    pub fn interpolation(self, interpolation: Interpolation) -> Self {
        Self {
            interpolation,
            ..self
        }
    }

    pub fn noise(self, noise: NoiseKind) -> Self {
        Self { noise, ..self }
    }

    pub fn build(self) -> Generator {
        let mut rng = StdRng::seed_from_u64(self.seed as u64);

        Generator {
            seed: self.seed,
            interpolation: self.interpolation,
            noise: self.noise.into(&mut rng, self.interpolation),
        }
    }
}
