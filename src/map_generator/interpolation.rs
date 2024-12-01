use std::ops::Deref;

fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t
}

fn cubic(a: f64, b: f64, t: f64) -> f64 {
    let v = t * t * (3.0 - 2.0 * t);
    lerp(a, b, v)
}

fn quintic(a: f64, b: f64, t: f64) -> f64 {
    let v = t * t * t * (t * (t * 6.0 - 15.0) + 10.0);
    lerp(a, b, v)
}

fn cosine(a: f64, b: f64, t: f64) -> f64 {
    let ft = t * std::f64::consts::PI;
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
    type Target = fn(f64, f64, f64) -> f64;

    fn deref(&self) -> &Self::Target {
        match self {
            Interpolation::Linear => &(lerp as fn(f64, f64, f64) -> f64),
            Interpolation::Cubic => &(cubic as fn(f64, f64, f64) -> f64),
            Interpolation::Quintic => &(quintic as fn(f64, f64, f64) -> f64),
            Interpolation::Cosine => &(cosine as fn(f64, f64, f64) -> f64),
        }
    }
}
