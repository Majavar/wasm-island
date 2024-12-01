use super::{
    heightmap::{Heightmap, HeightmapKind},
    interpolation::Interpolation,
    noise::{Noise, NoiseKind, NoiseType},
    shade, Color, ColorRamp, Vec3,
};
use bon::Builder;
use rand::{rngs::StdRng, SeedableRng};
use std::cell::OnceCell;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, strum::Display, strum::EnumIter)]
pub enum GeneratorType {
    #[default]
    Noise,
    Heightmap,
    ColoredMap,
}

#[derive(Builder)]
pub struct Generator {
    seed: u64,
    interpolation: Interpolation,
    noise: NoiseKind,
    width: usize,
    height: usize,
    heightmap: HeightmapKind,
    octave: u64,
    lacunarity: f64,
    persistence: f64,
    color_ramp: ColorRamp,
    flatten: bool,
    use_shading: bool,
    light_color: Color,
    dark_color: Color,
    light_position: Vec3,

    #[builder(skip)]
    current_noise: OnceCell<NoiseType>,
    #[builder(skip)]
    current_heightmap: OnceCell<Vec<f64>>,
    #[builder(skip)]
    current_flattened_map: OnceCell<Vec<f64>>,
}

impl Generator {
    pub fn set_seed(&mut self, seed: u64) {
        self.seed = seed;
        self.current_noise = OnceCell::new();
        self.current_heightmap = OnceCell::new();
        self.current_flattened_map = OnceCell::new();
    }

    pub fn set_interpolation(&mut self, interpolation: Interpolation) {
        self.interpolation = interpolation;

        if self.noise != NoiseKind::Simplex {
            self.current_noise = OnceCell::new();
            self.current_heightmap = OnceCell::new();
            self.current_flattened_map = OnceCell::new();
        }
    }

    pub fn set_noise(&mut self, noise: NoiseKind) {
        self.noise = noise;
        self.current_heightmap = OnceCell::new();
        self.current_flattened_map = OnceCell::new();
    }

    pub fn set_width(&mut self, width: usize) {
        self.width = width;
        self.current_heightmap = OnceCell::new();
        self.current_flattened_map = OnceCell::new();
    }

    pub fn set_height(&mut self, height: usize) {
        self.height = height;
        self.current_heightmap = OnceCell::new();
        self.current_flattened_map = OnceCell::new();
    }

    pub fn set_heightmap(&mut self, heightmap: HeightmapKind) {
        self.heightmap = heightmap;
        self.current_heightmap = OnceCell::new();
        self.current_flattened_map = OnceCell::new();
    }

    pub fn set_octave(&mut self, octave: u64) {
        self.octave = octave;
        if self.heightmap == HeightmapKind::Fractal {
            self.current_heightmap = OnceCell::new();
            self.current_flattened_map = OnceCell::new();
        }
    }

    pub fn set_lacunarity(&mut self, lacunarity: f64) {
        self.lacunarity = lacunarity;
        if self.heightmap == HeightmapKind::Fractal {
            self.current_heightmap = OnceCell::new();
            self.current_flattened_map = OnceCell::new();
        }
    }

    pub fn set_persistence(&mut self, persistence: f64) {
        self.persistence = persistence;
        if self.heightmap == HeightmapKind::Fractal {
            self.current_heightmap = OnceCell::new();
            self.current_flattened_map = OnceCell::new();
        }
    }

    pub fn set_flatten(&mut self, flatten: bool) {
        self.flatten = flatten;
        self.current_flattened_map = OnceCell::new();
        self.current_flattened_map = OnceCell::new();
    }

    pub fn set_use_shading(&mut self, use_shading: bool) {
        self.use_shading = use_shading;
    }

    pub fn generate(&self, generator_type: GeneratorType) -> Vec<u8> {
        let mut rng = StdRng::seed_from_u64(self.seed);

        if generator_type == GeneratorType::Noise {
            let noise = self
                .current_noise
                .get_or_init(move || self.noise.into(&mut rng, self.interpolation));

            (0..self.height)
                .flat_map(|y| {
                    (0..self.width).flat_map(move |x| {
                        let value = noise.noise(x as f64 / 32.0, y as f64 / 32.0);
                        std::iter::repeat((value * 255.0) as u8).take(4)
                    })
                })
                .collect()
        } else {
            let heightmap = self.current_heightmap.get_or_init(|| {
                self.heightmap
                    .into()
                    .noise_cell(&self.current_noise)
                    .get_noise(|r| self.noise.into(r, self.interpolation))
                    .octave(self.octave)
                    .lacunarity(self.lacunarity)
                    .persistance(self.persistence)
                    .rng(&mut rng)
                    .call()
                    .generate(self.width, self.height)
            });

            let flattened_map = self.current_flattened_map.get_or_init(|| {
                if self.flatten {
                    heightmap
                        .iter()
                        .map(|&value| {
                            (value - 0.5) * (value - 0.5) * if value < 0.5 { -2.0 } else { 2.0 }
                                + 0.5
                        })
                        .collect()
                } else {
                    heightmap.clone()
                }
            });

            if generator_type == GeneratorType::Heightmap {
                flattened_map
                    .iter()
                    .flat_map(|&value| std::iter::repeat((value * 255.0) as u8).take(4))
                    .collect()
            } else {
                flattened_map
                    .iter()
                    .enumerate()
                    .flat_map(|(index, &value)| {
                        if self.use_shading {
                            *shade(
                                flattened_map,
                                index,
                                self.color_ramp.get(value),
                                self.width,
                                &self.light_position,
                                self.light_color,
                                self.dark_color,
                            )
                        } else {
                            *self.color_ramp.get(value)
                        }
                    })
                    .collect()
            }
        }
    }
}
