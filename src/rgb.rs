use num::Float;
use lab::Lab;
use xyz::Xyz;

#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Default)]
pub struct Rgb {
    pub data: [u8; 3]
}

impl Rgb {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Rgb { data: [r, g, b] }
    }

    pub fn from_lab<T: Float>(lab: Lab<T>) -> Self {
        lab.to_rgb()
    }

    pub fn from_xyz<T: Float>(xyz: Xyz<T>) -> Self {
        xyz.to_rgb()
    }

    pub fn to_lab<T: Float>(self) -> Lab<T> {
        self.to_xyz().to_lab()
    }

    pub fn to_xyz<T: Float>(self) -> Xyz<T> {
        let one_hundred = T::from(100.0).unwrap();
        let max_u8 = T::from(255.0).unwrap();

        let r = pivot_rgb_xyz(T::from(self.data[0]).unwrap() / max_u8) * one_hundred;
        let g = pivot_rgb_xyz(T::from(self.data[1]).unwrap() / max_u8) * one_hundred;
        let b = pivot_rgb_xyz(T::from(self.data[2]).unwrap() / max_u8) * one_hundred;

        // (Observer = 2°, Illuminant = D65)
        Xyz {
            data: [
                r * T::from(0.4124).unwrap() + g * T::from(0.3576).unwrap() + b * T::from(0.1805).unwrap(),
                r * T::from(0.2126).unwrap() + g * T::from(0.7152).unwrap() + b * T::from(0.0722).unwrap(),
                r * T::from(0.0193).unwrap() + g * T::from(0.1192).unwrap() + b * T::from(0.9505).unwrap(),
            ]
        }
    }
}

fn pivot_rgb_xyz<T: Float>(n: T) -> T {
    if n > T::from(0.04045).unwrap() {
        ((n + T::from(0.055).unwrap()) / T::from(1.055).unwrap()).powf(T::from(2.4).unwrap())
    } else {
        n / T::from(12.92).unwrap()
    }
}
