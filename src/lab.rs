use num::Float;
use rgb::Rgb;
use xyz::Xyz;

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Hash, Default)]
pub struct Lab<T: Float = f32> {
    pub l: T,
    pub a: T,
    pub b: T,
}

impl<T: Float> Lab<T> {
    pub fn new(l: T, a: T, b: T) -> Self {
        Lab { l, a, b }
    }

    pub fn from_rgb(rgb: Rgb) -> Self {
        rgb.to_lab()
    }

    pub fn from_xyz(xyz: Xyz<T>) -> Self {
        xyz.to_lab()
    }

    pub fn to_rgb(self) -> Rgb {
        self.to_xyz().to_rgb()
    }

    pub fn to_xyz(self) -> Xyz<T> {
        let y = (self.l + T::from(16.0).unwrap()) / T::from(116.0).unwrap();
        let x = self.a / T::from(500.0).unwrap() + y;
        let z = y - self.b / T::from(200.0).unwrap();

        let white_ref = [
            T::from(95.047).unwrap(),
            T::from(100.000).unwrap(),
            T::from(108.883).unwrap()
        ];
        let x3 = x * x * x;
        let z3 = z * z * z;

        Xyz {
            data: [
                white_ref[0] * pivot_lab_xyz(x, x3),
                white_ref[1] * pivot_l_lab_xyz(self.l, y),
                white_ref[2] * pivot_lab_xyz(z, z3)
            ]
        }
    }
}

impl<T: Float> From<Rgb> for Lab<T> {
    fn from(rgb: Rgb) -> Self {
        Lab::from_rgb(rgb)
    }
}

impl<T: Float> From<Xyz<T>> for Lab<T> {
    fn from(xyz: Xyz<T>) -> Self {
        Lab::from_xyz(xyz)
    }
}

fn pivot_lab_xyz<T: Float>(n: T, n3: T) -> T {
    let epsilon = T::from(0.008856).unwrap();
    let kappa = T::from(7.787).unwrap();
    if n3 > epsilon {
        n3
    } else {
        (n - T::from(16.0).unwrap() / T::from(116.0).unwrap()) / kappa
    }
}

fn pivot_l_lab_xyz<T: Float>(l: T, y: T) -> T {
    let epsilon = T::from(0.008856).unwrap();
    let kappa = T::from(903.3).unwrap();
    if l > epsilon * kappa {
        y * y * y
    } else {
        l / kappa
    }
}
