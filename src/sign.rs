use std::cmp::Ordering;

#[inline]
#[must_use]
pub fn sign(x: f64) -> f64 {
    match x.partial_cmp(&0.0) {
        Some(Ordering::Less) => -1.0,
        Some(Ordering::Greater) => 1.0,
        Some(Ordering::Equal) => 0.0,
        None => f64::NAN,
    }
}

#[cfg(test)]
#[path = "sign_test.rs"]
mod test;
