/// Définition exacte IAU 2012 B1, en mètres (JPL Astrodynamic Parameters).
pub const AU: f64 = 149_597_870_700.0;
pub const DAY: f64 = 86_400.0;
pub const JULIAN_CENTURY: f64 = 36_525.0 * DAY;
pub const J2000: f64 = 2_451_545.0;
pub const G: f64 = 6.67430e-11;
pub const SUN_MU: f64 = 1.3271244004127942e20;
pub fn au_to_m(au: f64) -> f64 {
    au * AU
}
pub fn m_to_au(m: f64) -> f64 {
    m / AU
}
pub fn format_distance(m: f64) -> String {
    let m = m.max(0.0);
    if m >= AU * 0.1 {
        format!("{:.3} UA", m / AU)
    } else if m >= 1e9 {
        format!("{:.2} millions de km", m / 1e9)
    } else if m >= 1e3 {
        format!("{:.1} km", m / 1e3)
    } else {
        format!("{m:.1} m")
    }
}
