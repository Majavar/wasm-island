use crate::map_generator::{
    Color, ColorRampStep, GeneratorType, HeightmapKind, Interpolation, NoiseKind, Vec3,
};

pub const DEFAULT_GENERATOR_TYPE: GeneratorType = GeneratorType::ColoredMap;
pub const DEFAULT_SEED: u64 = 5;
pub const DEFAULT_INTERPOLATION: Interpolation = Interpolation::Cubic;
pub const DEFAULT_HEIGHTMAP: HeightmapKind = HeightmapKind::Fractal;
pub const DEFAULT_NOISE: NoiseKind = NoiseKind::Gradient;
pub const DEFAULT_WIDTH: usize = 512;
pub const DEFAULT_HEIGHT: usize = 512;
pub const DEFAULT_OCTAVE: u64 = 8;
pub const DEFAULT_LACUNARITY: f64 = 2.0;
pub const DEFAULT_PERSISTENCE: f64 = 0.5;
pub const DEFAULT_COLOR_RAMP: [ColorRampStep; 9] = [
    ColorRampStep {
        color: Color([2, 43, 68, 255]),
        position: 0.0,
    }, // very dark blue: deep water
    ColorRampStep {
        color: Color([9, 62, 92, 255]),
        position: 0.25,
    }, // dark blue: water
    ColorRampStep {
        color: Color([17, 82, 112, 255]),
        position: 0.49,
    }, // blue: shallow water
    ColorRampStep {
        color: Color([69, 108, 118, 255]),
        position: 0.50,
    }, //light blue: shore
    ColorRampStep {
        color: Color([42, 102, 41, 255]),
        position: 0.51,
    }, // green: grass
    ColorRampStep {
        color: Color([115, 128, 77, 255]),
        position: 0.75,
    }, // light green: veld
    ColorRampStep {
        color: Color([153, 143, 92, 255]),
        position: 0.85,
    }, // brown: tundra
    ColorRampStep {
        color: Color([179, 179, 179, 255]),
        position: 0.95,
    }, // grey: rocks
    ColorRampStep {
        color: Color([255, 255, 255, 255]),
        position: 1.0,
    }, // white: snow
];
pub const DEFAULT_FLATTEN: bool = true;
pub const DEFAULT_USE_SHADING: bool = true;
pub const DEFAULT_LIGHT: Color = Color([255, 255, 204, 255]);
pub const DEFAULT_DARK: Color = Color([51, 17, 51, 255]);
pub const DEFAULT_LIGHT_POSITION: Vec3 = Vec3([-1.0, -1.0, 0.0]);
