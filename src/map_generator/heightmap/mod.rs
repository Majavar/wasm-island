use super::noise::{Noise, NoiseType};
use bon::bon;
use enum_dispatch::enum_dispatch;
use rand::Rng;
use std::cell::OnceCell;

mod diamond;
mod fractal;
mod midpoint;
mod utils;

#[enum_dispatch]
pub trait Heightmap {
    fn generate(&mut self, width: usize, height: usize) -> Vec<f64>;
}

#[enum_dispatch(Heightmap)]
pub enum HeightmapType<'a, R, F>
where
    R: Rng,
    F: FnMut(&mut R) -> NoiseType,
{
    Midpoint(midpoint::Midpoint<'a, R>),
    Diamond(diamond::Diamond<'a, R>),
    Fractal(fractal::Fractal<'a, R, F>),
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, strum::Display, strum::EnumIter)]
pub enum HeightmapKind {
    #[default]
    Midpoint,
    Diamond,
    Fractal,
}

#[bon]
impl HeightmapKind {
    #[builder]
    pub fn into<'a, R, F>(
        self,
        noise_cell: &'a OnceCell<NoiseType>,
        get_noise: F,
        octave: u64,
        lacunarity: f64,
        persistance: f64,
        rng: &'a mut R,
    ) -> HeightmapType<'a, R, F>
    where
        R: Rng,
        F: FnMut(&mut R) -> NoiseType,
    {
        match self {
            HeightmapKind::Midpoint => midpoint::Midpoint { rng }.into(),
            HeightmapKind::Diamond => diamond::Diamond { rng }.into(),
            HeightmapKind::Fractal => fractal::Fractal {
                noise_cell,
                get_noise,
                octave,
                lacunarity,
                persistance,
                rng,
            }
            .into(),
        }
    }
}
