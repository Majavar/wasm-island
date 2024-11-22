use crate::map_generator::Interpolation;
use enum_dispatch::enum_dispatch;
use rand::Rng;

mod gradient;
mod value;

#[enum_dispatch]
pub trait Noise {
    fn noise(&self, x: f32, y: f32) -> f32;
}

#[enum_dispatch(Noise)]
pub enum NoiseType {
    Value(value::Value),
    Gradient(gradient::Gradient),
}

impl NoiseType {
    pub fn reseed<R: Rng>(&mut self, rng: &mut R, interpolation: Interpolation) {
        match self {
            NoiseType::Value(_) => *self = value::Value::new(rng, interpolation).into(),
            NoiseType::Gradient(_) => *self = gradient::Gradient::new(rng, interpolation).into(),
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, strum::Display, strum::EnumIter)]
pub enum NoiseKind {
    #[default]
    Value,
    Gradient,
}

impl NoiseKind {
    pub fn into<R: Rng>(self, rng: &mut R, interpolation: Interpolation) -> NoiseType {
        match self {
            NoiseKind::Value => value::Value::new(rng, interpolation).into(),
            NoiseKind::Gradient => gradient::Gradient::new(rng, interpolation).into(),
        }
    }
}
