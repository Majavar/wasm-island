use std::ops::Deref;

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

fn cubic(a: f32, b: f32, t: f32) -> f32 {
    let v = t * t * (3.0 - 2.0 * t);
    lerp(a, b, v)
}

fn quintic(a: f32, b: f32, t: f32) -> f32 {
    let v = t * t * t * (t * (t * 6.0 - 15.0) + 10.0);
    lerp(a, b, v)
}

fn cosine(a: f32, b: f32, t: f32) -> f32 {
    let ft = t * std::f32::consts::PI;
    let f = (1.0 - (ft).cos()) * 0.5;
    lerp(a, b, f)
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, strum::Display, strum::EnumIter)]
pub enum Interpolation {
    #[default]
    Linear,
    Cubic,
    Quintic,
    Cosine,
}

impl Deref for Interpolation {
    type Target = fn(f32, f32, f32) -> f32;

    fn deref(&self) -> &Self::Target {
        match self {
            Interpolation::Linear => &(lerp as fn(f32, f32, f32) -> f32),
            Interpolation::Cubic => &(cubic as fn(f32, f32, f32) -> f32),
            Interpolation::Quintic => &(quintic as fn(f32, f32, f32) -> f32),
            Interpolation::Cosine => &(cosine as fn(f32, f32, f32) -> f32),
        }
    }
}
