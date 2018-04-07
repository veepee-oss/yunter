use num_traits::{Float, clamp};
use rgb::Rgb;
use lab::Lab;

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Hash, Default)]
pub struct Xyz<T: Float = f32> {
    pub data: [T; 3]
}

impl<T: Float> Xyz<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Xyz { data: [x, y, z] }
    }

    pub fn from_rgb(rgb: Rgb) -> Self {
        rgb.to_xyz()
    }

    pub fn from_lab(lab: Lab<T>) -> Self {
        lab.to_xyz()
    }

    pub fn to_rgb(self) -> Rgb {
        let one_hundred = T::from(100.0).unwrap();

        let x = self.data[0] / one_hundred;
        let y = self.data[1] / one_hundred;
        let z = self.data[2] / one_hundred;

        let r = x * T::from(3.2406).unwrap() + y * T::from(-1.5372).unwrap() + z * T::from(-0.4986).unwrap();
        let g = x * T::from(-0.9689).unwrap() + y * T::from(1.8758).unwrap() + z * T::from(0.0415).unwrap();
        let b = x * T::from(0.0557).unwrap() + y * T::from(-0.2040).unwrap() + z * T::from(1.0570).unwrap();

        let r = pivot_xyz_rgb(r);
        let g = pivot_xyz_rgb(g);
        let b = pivot_xyz_rgb(b);

        let zero = T::zero();
        let max_u8 = T::from(255.0).unwrap();

        Rgb {
            data: [
                clamp(r * max_u8, zero, max_u8).to_u8().unwrap(),
                clamp(g * max_u8, zero, max_u8).to_u8().unwrap(),
                clamp(b * max_u8, zero, max_u8).to_u8().unwrap(),
            ]
        }
    }

    pub fn to_lab(self) -> Lab<T> {
        let white_ref = [
            T::from(95.047).unwrap(),
            T::from(100.000).unwrap(),
            T::from(108.883).unwrap(),
        ];
        let x = pivot_xyz_lab(self.data[0] / white_ref[0]);
        let y = pivot_xyz_lab(self.data[1] / white_ref[1]);
        let z = pivot_xyz_lab(self.data[2] / white_ref[2]);

        Lab {
            l: (T::from(116.0).unwrap() * y - T::from(16.0).unwrap()).max(T::from(0.0).unwrap()),
            a: T::from(500.0).unwrap() * (x - y),
            b: T::from(200.0).unwrap() * (y - z)
        }
    }
}

impl<T: Float> From<Lab<T>> for Xyz<T> {
    fn from(lab: Lab<T>) -> Self {
        Xyz::from_lab(lab)
    }
}

impl<T: Float> From<Rgb> for Xyz<T> {
    fn from(rgb: Rgb) -> Self {
        Xyz::from_rgb(rgb)
    }
}

fn pivot_xyz_rgb<T: Float>(n: T) -> T {
    if n > T::from(0.0031308).unwrap() {
        T::from(1.055).unwrap() * n.powf(T::from(1.0).unwrap() / T::from(2.4).unwrap()) - T::from(0.055).unwrap()
    } else {
        n * T::from(12.92).unwrap()
    }
}

fn cubic_root<T: Float>(n: T) -> T {
    n.powf(T::from(1.0).unwrap() / T::from(3.0).unwrap())
}

fn pivot_xyz_lab<T: Float>(n: T) -> T {
    let epsilon = T::from(0.008856).unwrap();
    let kappa = T::from(903.3).unwrap();
    if n > epsilon {
        cubic_root(n)
    } else {
        (kappa * n + T::from(16.0).unwrap()) / T::from(116.0).unwrap()
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
    fn xyz_to_rgb_simple() {
        let xyz = Xyz { data: [33.113681223365006, 15.997065707552856, 50.057654344067586] };
        let rgb: Rgb = xyz.into();
        assert_eq!(rgb.data[0], 200);
        assert_eq!(rgb.data[1], 0);
        assert_eq!(rgb.data[2], 190);
    }

    #[test]
    fn xyz_to_lab_simple() {
        let xyz = Xyz { data: [33.113681223365006, 15.997065707552856, 50.057654344067586] };
        let lab: Lab = xyz.into();
        assert_eq!(lab.l, 46.97064);
        assert_eq!(lab.a, 80.399574);
        assert_eq!(lab.b, -45.789467);
    }

    #[test]
    fn xyz_to_lab_simple2() {
        let xyz = Xyz { data: [1.0590637931500604, 0.840998318832299, 0.22137415400510363] };
        let lab: Lab = xyz.into();
        assert_eq!(lab.l, 7.596737);
        assert_eq!(lab.a, 9.967141);
        assert_eq!(lab.b, 9.931389);
    }
}
