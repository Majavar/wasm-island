use super::{color::lerp, Color, Vec3};

pub fn shade(
    heightmap: &[f64],
    index: usize,
    color: Color,
    width: usize,
    light: &Vec3,
    light_color: Color,
    dark_color: Color,
) -> Color {
    if heightmap[index] > 0.5 {
        let x = index % width;
        let y = index / width;

        let nx = if x == 0 {
            (heightmap[index + 1] - heightmap[index]) * 2.0
        } else if x == width - 1 {
            (heightmap[index] - heightmap[index - 1]) * 2.0
        } else {
            heightmap[index + 1] - heightmap[index - 1]
        };

        let ny = if y == 0 {
            (heightmap[index + width] - heightmap[index]) * 2.0
        } else if y * width > heightmap.len() - width - 1 {
            (heightmap[index] - heightmap[index - width]) * 2.0
        } else {
            heightmap[index + width] - heightmap[index - width]
        };

        let n = (nx * nx + ny * ny + 4.0).sqrt();
        let normal = Vec3([-nx / n, -ny / n, 2.0 / n]);

        let d = light.dot(&normal) * 25.0 + 0.5;

        if d < 0.0 {
            dark_color
        } else if d > 1.0 {
            light_color
        } else if d < 0.5 {
            lerp(dark_color, color, 2.0 * d)
        } else {
            lerp(color, light_color, 2.0 * d - 1.0)
        }
    } else {
        color
    }
}
