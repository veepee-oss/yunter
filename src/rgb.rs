use num_traits::Float;
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

impl<T: Float> From<Lab<T>> for Rgb {
    fn from(lab: Lab<T>) -> Self {
        Rgb::from_lab(lab)
    }
}

impl<T: Float> From<Xyz<T>> for Rgb {
    fn from(xyz: Xyz<T>) -> Self {
        Rgb::from_xyz(xyz)
    }
}

fn pivot_rgb_xyz<T: Float>(n: T) -> T {
    if n > T::from(0.04045).unwrap() {
        ((n + T::from(0.055).unwrap()) / T::from(1.055).unwrap()).powf(T::from(2.4).unwrap())
    } else {
        n / T::from(12.92).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use lab::Lab;
    use rgb::Rgb;
    use xyz::Xyz;

    #[test]
    fn rgb_to_xyz_simple() {
        let rgb = Rgb { data: [50, 50, 50] };
        let xyz: Xyz = rgb.into();
        assert_eq!(xyz.data[0], 3.0317173);
        assert_eq!(xyz.data[1], 3.1896026);
        assert_eq!(xyz.data[2], 3.4734776);
    }

    #[test]
    fn rgb_to_xyz_difficult() {
        let rgb = Rgb { data: [43, 21, 8] };
        let xyz: Xyz = rgb.into();
        assert_eq!(xyz.data[0], 1.3082554);
        assert_eq!(xyz.data[1], 1.0674537);
        assert_eq!(xyz.data[2], 0.3668146);
    }

    #[test]
    fn rgb_to_lab_simple() {
        let rgb = Rgb { data: [50, 50, 50] };
        let lab: Lab = rgb.into();
        assert_eq!(lab.l, 20.787773);
        assert_eq!(lab.a, 0.0016838312);
        assert_eq!(lab.b, -0.0033080578);
    }

    #[test]
    fn rgb_to_lab_simple2() {
        let rgb = Rgb { data: [39, 17, 4] };
        let lab: Lab = rgb.into();
        assert_eq!(lab.l, 7.596737);
        assert_eq!(lab.a, 9.967141);
        assert_eq!(lab.b, 9.931389);
    }
}
