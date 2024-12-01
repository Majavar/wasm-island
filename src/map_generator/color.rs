use std::ops::Deref;

#[derive(Debug, Clone, Copy)]
pub struct Color(pub [u8; 4]);

impl Color {
    fn r(&self) -> f64 {
        self.0[0] as f64
    }

    fn g(&self) -> f64 {
        self.0[1] as f64
    }

    fn b(&self) -> f64 {
        self.0[2] as f64
    }

    fn a(&self) -> f64 {
        self.0[3] as f64
    }
}

impl Deref for Color {
    type Target = [u8; 4];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn lerp(a: Color, b: Color, t: f64) -> Color {
    if t <= 0.0 {
        a
    } else if t >= 1.0 {
        b
    } else {
        Color([
            (a.r() + (b.r() - a.r()) * t) as u8,
            (a.g() + (b.g() - a.g()) * t) as u8,
            (a.b() + (b.b() - a.b()) * t) as u8,
            (a.a() + (b.a() - a.a()) * t) as u8,
        ])
    }
}

#[derive(Debug, Clone)]
pub struct ColorRampStep {
    pub color: Color,
    pub position: f64,
}

#[derive(Debug, Clone)]
pub struct ColorRamp {
    steps: Vec<ColorRampStep>,
}

impl From<Vec<ColorRampStep>> for ColorRamp {
    fn from(mut steps: Vec<ColorRampStep>) -> Self {
        ColorRamp {
            steps: {
                steps.sort_by(|a, b| a.position.partial_cmp(&b.position).unwrap());
                steps
            },
        }
    }
}

impl ColorRamp {
    pub fn get(&self, position: f64) -> Color {
        match self.steps.iter().position(|x| x.position >= position) {
            Some(0) => self.steps[0].color,
            Some(i) => {
                let a = &self.steps[i - 1];
                let b = &self.steps[i];
                let t = (position - a.position) / (b.position - a.position);
                lerp(a.color, b.color, t)
            }
            None => self.steps.last().unwrap().color,
        }
    }
}
