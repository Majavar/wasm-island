use crate::map_generator::Interpolation;
use enum_dispatch::enum_dispatch;
use rand::Rng;

mod gradient;
mod simplex;
mod value;

#[enum_dispatch]
pub trait Noise {
    fn noise(&self, x: f64, y: f64) -> f64;
}

#[enum_dispatch(Noise)]
pub enum NoiseType {
    Value(value::Value),
    Gradient(gradient::Gradient),
    Simplex(simplex::Simplex),
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, strum::Display, strum::EnumIter)]
pub enum NoiseKind {
    #[default]
    Value,
    Gradient,
    Simplex,
}

impl NoiseKind {
    pub fn into<R: Rng>(self, rng: &mut R, interpolation: Interpolation) -> NoiseType {
        match self {
            NoiseKind::Value => value::Value::new(rng, interpolation).into(),
            NoiseKind::Gradient => gradient::Gradient::new(rng, interpolation).into(),
            NoiseKind::Simplex => simplex::Simplex::new(rng).into(),
        }
    }
}
