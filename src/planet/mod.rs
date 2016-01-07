#[macro_use]
pub mod mercury;

#[macro_use]
pub mod venus;

#[macro_use]
pub mod earth;

#[macro_use]
pub mod mars;

#[macro_use]
pub mod jupiter;

#[macro_use]
pub mod saturn;

#[macro_use]
pub mod uranus;

#[macro_use]
pub mod neptune;

use angle;
use time;

/// Represents different planets
pub enum Planet {
    /// Mercury
    Mercury,
    /// Venus
    Venus,
    /// Earth
    Earth,
    /// Mars
    Mars,
    /// Jupiter
    Jupiter,
    /// Saturn
    Saturn,
    /// Uranus
    Uranus,
    /// Neptune
    Neptune,
}

pub fn HeliocentricCoords(planet: Planet, JD: f64) -> (f64, f64, f64) {/*
    let terms = match planet {
        Planet::Mercury => VSOP87_Mercury_Terms!(),
        Planet::Venus => VSOP87_Mercury_Terms!(),
        Planet::Earth => VSOP87_Mercury_Terms!(),
        Planet::Mars => VSOP87_Mercury_Terms!(),
        Planet::Jupiter => VSOP87_Mercury_Terms!(),
        Planet::Saturn => VSOP87_Mercury_Terms!(),
        Planet::Uranus => VSOP87_Mercury_Terms!(),
        Planet::Neptune => VSOP87_Mercury_Terms!(),
    };

    let JM = time::JulianMillenium(JD);

    let (L0_terms, L1_terms, L2_terms, L3_terms, L4_terms, L5_terms) = terms.0;
    let L = VSOP87Coordinate(JM, &L0_terms, &L1_terms, &L2_terms, &L3_terms, &L4_terms, &L5_terms);

    let (B0_terms, B1_terms, B2_terms, B3_terms, B4_terms, B5_terms) = terms.1;
    let B = VSOP87Coordinate(JM, &B0_terms, &B1_terms, &B2_terms, &B3_terms, &B4_terms, &B5_terms);

    let (R0_terms, R1_terms, R2_terms, R3_terms, R4_terms, R5_terms) = terms.2;
    let R = VSOP87Coordinate(JM, &R0_terms, &R1_terms, &R2_terms, &R3_terms, &R4_terms, &R5_terms);

    (L, B, R)*/(0.0, 0.0, 0.0)
}

fn VSOP87Coordinate(t: f64, a: &[[f64; 3]], b: &[[f64; 3]], c: &[[f64; 3]], d: &[[f64; 3]], e: &[[f64; 3]], f: &[[f64; 3]]) -> f64 {
    let mut T0 = 0.0; for &i in a.iter() { T0 += VSOP87Term(t, &i); }
    let mut T1 = 0.0; for &i in b.iter() { T1 += VSOP87Term(t, &i); }
    let mut T2 = 0.0; for &i in c.iter() { T2 += VSOP87Term(t, &i); }
    let mut T3 = 0.0; for &i in d.iter() { T3 += VSOP87Term(t, &i); }
    let mut T4 = 0.0; for &i in e.iter() { T4 += VSOP87Term(t, &i); }
    let mut T5 = 0.0; for &i in f.iter() { T5 += VSOP87Term(t, &i); }

    T0 +
    t * (T1 +
    t * (T2 +
    t * (T3 +
    t * (T4 +
    t * T5 ))))
}

fn VSOP87Term(t: f64, array: &[f64]) -> f64 {
    array[0] * (array[1] + t*array[2]).cos()
}

fn EffectOfLightTime(x: f64, y: f64, z: f64) -> f64 {
    0.0057755183 * (x*x + y*y + z*z).sqrt()
}

pub fn GeocentricEclipticalCoords(L: f64, B: f64, R: f64, L0: f64, B0: f64, R0: f64) -> (f64, f64, f64) {
    let x = R*B.cos()*L.cos() - R0*B0.cos()*L0.cos();
    let y = R*B.cos()*L.sin() - R0*B0.cos()*L0.sin();
    let z = R*B.sin() - R0*B0.sin();

    (y.atan2(x),
     z/(x*x + y*y).sqrt(),
     EffectOfLightTime(x, y, z))
}

pub fn GeocentricEquatorialCoords(X: f64, Y: f64, Z: f64, semimaj_axis: f64, e: f64, i: f64, w: f64, sigma: f64, n: f64,
            oblq_eclip: f64, M: f64, E: f64, v: f64, r: f64) -> (f64, f64, f64) {

    let F = sigma.cos();
    let G = sigma.sin() * oblq_eclip.cos();
    let H = sigma.sin() * oblq_eclip.sin();

    let P = - sigma.sin() * i.cos();
    let Q =   sigma.cos() * i.cos() * oblq_eclip.cos()
            - i.sin()     * oblq_eclip.sin();
    let R =   sigma.cos() * i.cos() * oblq_eclip.sin()
            + i.sin()     * oblq_eclip.cos();

    let A = F.atan2(P);
    let B = G.atan2(Q);
    let C = H.atan2(R);
    let a = (F*F + P*P).sqrt();
    let b = (G*G + Q*Q).sqrt();
    let c = (H*H + R*R).sqrt();

    let x = r * a * (A + w + v);
    let y = r * b * (B + w + v);
    let z = r * c * (C + w + v);

    let xi = X + x;
    let nu = Y + y;
    let et = Z + z;

    let asc = angle::LimitedTo360(nu.atan2(xi).to_degrees()).to_radians();
    let dec = et.atan2((xi*xi + nu*nu).sqrt());

    (asc, dec, EffectOfLightTime(x, y, z))
}

pub fn HeliocentricCoordsFromOrbitalElements(i: f64, sigma: f64, w: f64, v: f64, r: f64) -> (f64, f64) {
    let u = w + v;
    let x = r * (sigma.cos()*u.cos() - sigma.sin()*u.sin()*i.cos());
    let y = r * (sigma.sin()*u.cos() + sigma.cos()*u.sin()*i.cos());
    let z = r * i.sin() * u.sin();

    (y.atan2(x),
     z.atan2((x*x + y*y).sqrt()))
}
