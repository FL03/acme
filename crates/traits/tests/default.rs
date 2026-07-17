/*
    Appellation: default <test>
    Contrib: @FL03
*/

#[test]
fn compiles() {
    fn add<A, B, C>(a: A, b: B) -> C
    where
        A: core::ops::Add<B, Output = C>,
    {
        a + b
    }

    assert_eq! { add(1, 100), 101 }
    assert_eq! { add(1.0, 100.0), 101.0 }
}
