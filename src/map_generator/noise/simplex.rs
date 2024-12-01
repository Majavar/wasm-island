use super::Noise;
use rand::{seq::SliceRandom, Rng};

fn grad(hash: u8, x: f64, y: f64) -> f64 {
    match hash & 0b111 {
        0b000 => x + y,
        0b001 => x,
        0b010 => x - y,
        0b011 => y,
        0b100 => -y,
        0b101 => -x + y,
        0b110 => -x,
        0b111 => -x - y,
        _ => unreachable!(),
    }
}

pub struct Simplex {
    permutation: [u8; 256],
}

impl Simplex {
    pub fn new<R: Rng>(rng: &mut R) -> Self {
        let mut permutation = [0; 256];
        for (i, x) in permutation.iter_mut().enumerate() {
            *x = i as u8;
        }
        permutation.shuffle(rng);

        Self { permutation }
    }

    fn index(&self, x: usize, y: usize) -> u8 {
        self.permutation[(self.permutation[x & 0xFF] as usize + y) & 0xFF]
    }
}

impl Noise for Simplex {
    fn noise(&self, x: f64, y: f64) -> f64 {
        let f2 = 0.5 * (f64::sqrt(3.0) - 1.0);
        let g2 = (3.0 - f64::sqrt(3.0)) / 6.0;

        let s = (x + y) * f2;
        let i = (x + s) as usize;
        let j = (y + s) as usize;

        let t = ((i + j) as f64) * g2;
        let x0 = x - (i as f64) + t;
        let y0 = y - (j as f64) + t;

        let (i1, j1) = if x0 > y0 { (1, 0) } else { (0, 1) };

        let x1 = x0 - (i1 as f64) + g2;
        let y1 = y0 - (j1 as f64) + g2;

        let v = g2.mul_add(2.0, -1.0);
        let x2 = x0 + v;
        let y2 = y0 + v;

        let t0 = 0.5 - x0 * x0 - y0 * y0;
        let t1 = 0.5 - x1 * x1 - y1 * y1;
        let t2 = 0.5 - x2 * x2 - y2 * y2;

        let n0 = if t0.is_sign_positive() {
            let gi0 = self.index(i, j);
            let d0 = t0 * t0;
            d0 * d0 * grad(gi0, x0, y0)
        } else {
            0.0
        };

        let n1 = if t1.is_sign_positive() {
            let gi1 = self.index(i + i1, j + j1);
            let d1 = t1 * t1;
            d1 * d1 * grad(gi1, x1, y1)
        } else {
            0.0
        };

        let n2 = if t2.is_sign_positive() {
            let gi2 = self.index(i + 1, j + 1);
            let d2 = t2 * t2;
            d2 * d2 * grad(gi2, x2, y2)
        } else {
            0.0
        };

        35.0 * (n0 + n1 + n2) + 0.5
    }
}
