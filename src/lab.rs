use num_traits::Float;
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

#[cfg(test)]
mod tests {
    use lab::Lab;
    use rgb::Rgb;
    use xyz::Xyz;

    #[test]
    fn lab_to_xyz_simple() {
        let lab = Lab { l: 46.97063877033799,
                        a: 80.39955582611775,
                        b: -45.78947040551107 };
        let xyz: Xyz = lab.into();
        assert_eq!(xyz.data[0], 33.113674);
        assert_eq!(xyz.data[1], 15.997065);
        assert_eq!(xyz.data[2], 50.057648);
    }

    #[test]
    fn lab_to_xyz_simple3() {
        let lab = Lab { l: 7.417381,
                        a: 1.6188575,
                        b: 5.2350793 };
        let xyz: Xyz = lab.into();
        assert_eq!(xyz.data[0], 0.8199973);
        assert_eq!(xyz.data[1], 0.82114255);
        assert_eq!(xyz.data[2], 0.52809083);
    }

    #[test]
    fn lab_to_rgb_simple() {
        let lab = Lab { l: 46.97063877033799, a: 80.39955582611775, b: -45.78947040551107 };
        let rgb: Rgb = lab.into();
        assert_eq!(rgb.data[0], 200);
        assert_eq!(rgb.data[1], 0);
        assert_eq!(rgb.data[2], 190);
    }

    #[test]
    fn lab_to_rgb_simple2() {
        let lab = Lab { l: 8.112699908516632,
                        a: 12.557465203893239,
                        b: 0.05391983803255673 };
        let rgb: Rgb = lab.into();
        assert_eq!(rgb.data[0], 39);
        assert_eq!(rgb.data[1], 17);
        assert_eq!(rgb.data[2], 24);
    }

    #[test]
    fn lab_to_rgb_simple3() {
        let lab = Lab { l: 7.417381,
                        a: 1.6188575,
                        b: 5.2350793 };
        let rgb: Rgb = lab.into();
        assert_eq!(rgb.data[0], 27);
        assert_eq!(rgb.data[1], 21);
        assert_eq!(rgb.data[2], 13);
    }
}
