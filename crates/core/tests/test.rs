#[cfg(test)]
#[test]
fn basic() {
    let f = |x: f32, y: f32| x.powf(y);
    assert_eq!(f(2.0, 3.0), 8.0)
}

#[test]
fn test_timestamp() {
    let ts: acme_core::TimeStamp = acme_core::utils::timestamp();
    assert_eq!(&ts, &ts)
}