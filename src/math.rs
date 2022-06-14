use std::f64;

// Gamma function implementation taken from statrs (https://crates.io/crates/statrs), as it was
// easyer than importing the whole crate just to use it

const TWO_SQRT_E_OVER_PI: f64 = 1.8603827342052657173362492472666631120594218414085755;

const GAMMA_R: f64 = 10.900511;
const GAMMA_DK: &[f64] = &[
    2.48574089138753565546e-5,
    1.05142378581721974210,
    -3.45687097222016235469,
    4.51227709466894823700,
    -2.98285225323576655721,
    1.05639711577126713077,
    -1.95428773191645869583e-1,
    1.70970543404441224307e-2,
    -5.71926117404305781283e-4,
    4.63399473359905636708e-6,
    -2.71994908488607703910e-9,
];

fn gamma(x: f64) -> f64 {
    if x < 0.5 {
        let s = GAMMA_DK
            .iter()
            .enumerate()
            .skip(1)
            .fold(GAMMA_DK[0], |s, t| s + t.1 / (t.0 as f64 - x));

        f64::consts::PI
            / ((f64::consts::PI * x).sin()
                * s
                * TWO_SQRT_E_OVER_PI
                * ((0.5 - x + GAMMA_R) / f64::consts::E).powf(0.5 - x))
    } else {
        let s = GAMMA_DK
            .iter()
            .enumerate()
            .skip(1)
            .fold(GAMMA_DK[0], |s, t| s + t.1 / (x + t.0 as f64 - 1.0));

        s * TWO_SQRT_E_OVER_PI * ((x - 0.5 + GAMMA_R) / f64::consts::E).powf(x - 0.5)
    }
}

fn fact_int(x: u64) -> f64 {
    let mut count: f64 = 1.;

    if x == 0 || x == 1 {
        return 1.;
    } else if x > 170 {
        return f64::INFINITY;
    }

    for i in 2..x + 1 {
        count *= i as f64;
    }

    count
}

pub fn fact(x: f64) -> f64 {
    return if x < 0. {
        f64::NAN
    } else if x - x.trunc() < std::f64::EPSILON {
        fact_int(x.round() as u64)
    } else {
        gamma(x + 1.)
    };
}
