extern crate num_traits;

mod xyz;
mod lab;
mod rgb;

pub use xyz::Xyz;
pub use lab::Lab;
pub use rgb::Rgb;

fn pivot_rgb_xyz(n: f32) -> f32 {
    if n > 0.04045 {
        ((n + 0.055) / 1.055).powf(2.4)
    } else {
        (n / 12.92)
    }
}

fn pivot_xyz_rgb(n: f32) -> f32 {
    if n > 0.0031308 {
        1.055 * n.powf(1.0 / 2.4) - 0.055
    } else {
        n * 12.92
    }
}

fn cubic_root(n: f32) -> f32 {
    n.powf(1.0 / 3.0)
}

fn pivot_xyz_lab(n: f32) -> f32 {
    let epsilon = 0.008856;
    let kappa = 903.3;
    if n > epsilon {
        cubic_root(n)
    } else {
        (kappa * n + 16.0) / 116.0
    }
}

fn pivot_lab_xyz(n: f32, n3: f32) -> f32 {
    let epsilon = 0.008856;
    let kappa = 7.787;
    if n3 > epsilon {
        n3
    } else {
        (n - 16.0 / 116.0) / kappa
    }
}

fn pivot_l_lab_xyz(l: f32, y: f32) -> f32 {
    let epsilon = 0.008856;
    let kappa = 903.3;
    if l > epsilon * kappa {
        y * y * y
    } else {
        l / kappa
    }
}

fn clamp_0_255(n: f32) -> u8 {
    let n = n * 255.0;
    if n > 255.0 {
        255
    } else if n < 0.0 {
        0
    } else {
        n as u8
    }
}

pub fn rgb_to_xyz(rgb: Rgb) -> Xyz {
    let r = pivot_rgb_xyz(rgb.data[0] as f32 / 255.0) * 100.0;
    let g = pivot_rgb_xyz(rgb.data[1] as f32 / 255.0) * 100.0;
    let b = pivot_rgb_xyz(rgb.data[2] as f32 / 255.0) * 100.0;

    // (Observer = 2°, Illuminant = D65)
    Xyz {
        data: [
            r * 0.4124 + g * 0.3576 + b * 0.1805,
            r * 0.2126 + g * 0.7152 + b * 0.0722,
            r * 0.0193 + g * 0.1192 + b * 0.9505
        ]
    }
}

pub fn xyz_to_rgb(xyz: Xyz) -> Rgb {
    let x = xyz.data[0] / 100.0;
    let y = xyz.data[1] / 100.0;
    let z = xyz.data[2] / 100.0;

    let r = x * 3.2406 + y * -1.5372 + z * -0.4986;
    let g = x * -0.9689 + y * 1.8758 + z * 0.0415;
    let b = x * 0.0557 + y * -0.2040 + z * 1.0570;

    let r = pivot_xyz_rgb(r);
    let g = pivot_xyz_rgb(g);
    let b = pivot_xyz_rgb(b);

    Rgb { data: [clamp_0_255(r), clamp_0_255(g), clamp_0_255(b)] }
}

pub fn xyz_to_lab(xyz: Xyz) -> Lab {
    let white_ref: [f32; 3] = [95.047, 100.000, 108.883];
    let x = pivot_xyz_lab(xyz.data[0] / white_ref[0]);
    let y = pivot_xyz_lab(xyz.data[1] / white_ref[1]);
    let z = pivot_xyz_lab(xyz.data[2] / white_ref[2]);

    Lab {
        l: (116.0 * y - 16.0).max(0.0),
        a: 500.0 * (x - y),
        b: 200.0 * (y - z)
    }
}

pub fn lab_to_xyz(lab: Lab) -> Xyz {
    let y = (lab.l + 16.0) / 116.0;
    let x = lab.a / 500.0 + y;
    let z = y - lab.b / 200.0;

    let white_ref: [f32; 3] = [95.047, 100.000, 108.883];
    let x3 = x * x * x;
    let z3 = z * z * z;

    Xyz {
        data: [
            white_ref[0] * pivot_lab_xyz(x, x3),
            white_ref[1] * pivot_l_lab_xyz(lab.l, y),
            white_ref[2] * pivot_lab_xyz(z, z3)
        ]
    }
}

pub fn rgb_to_lab(rgb: Rgb) -> Lab {
    xyz_to_lab(rgb_to_xyz(rgb))
}

pub fn lab_to_rgb(lab: Lab) -> Rgb {
    xyz_to_rgb(lab_to_xyz(lab))
}

#[cfg(test)]
mod tests {
    use {rgb_to_xyz, xyz_to_lab, lab_to_xyz, xyz_to_rgb, rgb_to_lab, lab_to_rgb};
    use {Xyz, Lab, Rgb};

    #[test]
    fn rgb_to_xyz_simple() {
        let rgb = Rgb { data: [50, 50, 50] };
        let xyz = rgb_to_xyz(rgb);
        assert_eq!(xyz.data[0], 3.0317173);
        assert_eq!(xyz.data[1], 3.1896026);
        assert_eq!(xyz.data[2], 3.4734776);
    }

    #[test]
    fn rgb_to_xyz_difficult() {
        let rgb = Rgb { data: [43, 21, 8] };
        let xyz = rgb_to_xyz(rgb);
        assert_eq!(xyz.data[0], 1.3082554);
        assert_eq!(xyz.data[1], 1.0674537);
        assert_eq!(xyz.data[2], 0.3668146);
    }

    #[test]
    fn xyz_to_rgb_simple() {
        let xyz = Xyz { data: [33.113681223365006, 15.997065707552856, 50.057654344067586] };
        let rgb = xyz_to_rgb(xyz);
        assert_eq!(rgb.data[0], 200);
        assert_eq!(rgb.data[1], 0);
        assert_eq!(rgb.data[2], 190);
    }

    #[test]
    fn xyz_to_lab_simple() {
        let xyz = Xyz { data: [33.113681223365006, 15.997065707552856, 50.057654344067586] };
        let lab = xyz_to_lab(xyz);
        assert_eq!(lab.l, 46.97064);
        assert_eq!(lab.a, 80.399574);
        assert_eq!(lab.b, -45.789467);
    }

    #[test]
    fn xyz_to_lab_simple2() {
        let xyz = Xyz { data: [1.0590637931500604, 0.840998318832299, 0.22137415400510363] };
        let lab = xyz_to_lab(xyz);
        assert_eq!(lab.l, 7.596737);
        assert_eq!(lab.a, 9.967141);
        assert_eq!(lab.b, 9.931389);
    }

    #[test]
    fn lab_to_xyz_simple() {
        let lab = Lab { l: 46.97063877033799,
                        a: 80.39955582611775,
                        b: -45.78947040551107 };
        let xyz = lab_to_xyz(lab);
        assert_eq!(xyz.data[0], 33.113674);
        assert_eq!(xyz.data[1], 15.997065);
        assert_eq!(xyz.data[2], 50.057648);
    }

    #[test]
    fn lab_to_xyz_simple3() {
        let lab = Lab { l: 7.417381,
                        a: 1.6188575,
                        b: 5.2350793 };
        let xyz = lab_to_xyz(lab);
        assert_eq!(xyz.data[0], 0.8199973);
        assert_eq!(xyz.data[1], 0.82114255);
        assert_eq!(xyz.data[2], 0.52809083);
    }

    #[test]
    fn rgb_to_lab_simple() {
        let rgb = Rgb { data: [50, 50, 50] };
        let lab = rgb_to_lab(rgb);
        assert_eq!(lab.l, 20.787773);
        assert_eq!(lab.a, 0.0016838312);
        assert_eq!(lab.b, -0.0033080578);
    }

    #[test]
    fn rgb_to_lab_simple2() {
        let rgb = Rgb { data: [39, 17, 4] };
        let lab = rgb_to_lab(rgb);
        assert_eq!(lab.l, 7.596737);
        assert_eq!(lab.a, 9.967141);
        assert_eq!(lab.b, 9.931389);
    }

    #[test]
    fn lab_to_rgb_simple() {
        let lab = Lab { l: 46.97063877033799, a: 80.39955582611775, b: -45.78947040551107 };
        let rgb = lab_to_rgb(lab);
        assert_eq!(rgb.data[0], 200);
        assert_eq!(rgb.data[1], 0);
        assert_eq!(rgb.data[2], 190);
    }

    #[test]
    fn lab_to_rgb_simple2() {
        let lab = Lab { l: 8.112699908516632,
                        a: 12.557465203893239,
                        b: 0.05391983803255673 };
        let rgb = lab_to_rgb(lab);
        assert_eq!(rgb.data[0], 39);
        assert_eq!(rgb.data[1], 17);
        assert_eq!(rgb.data[2], 24);
    }

    #[test]
    fn lab_to_rgb_simple3() {
        let lab = Lab { l: 7.417381,
                        a: 1.6188575,
                        b: 5.2350793 };
        let rgb = lab_to_rgb(lab);
        assert_eq!(rgb.data[0], 27);
        assert_eq!(rgb.data[1], 21);
        assert_eq!(rgb.data[2], 13);
    }
}
