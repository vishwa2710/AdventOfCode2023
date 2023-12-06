pub fn roots(a: i64, b: i64, c: i64) -> Option<(f64, f64)> {
    let discriminant = b * b - 4 * a * c;
    if discriminant < 0 {
        return None;
    }
    let sqrt_discriminant = (discriminant as f64).sqrt();
    let root1 = (-b as f64 - sqrt_discriminant) / (2.0 * a as f64);
    let root2 = (-b as f64 + sqrt_discriminant) / (2.0 * a as f64);
    Some((root1, root2))
}
