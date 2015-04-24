#![macro_use]

macro_rules! assert_eq_f32 {
    ($left:expr , $right:expr) => ({
        let (left, right) = ($left, $right);
        let diff = (left - right).abs();
        if diff > 0.000001 {
            panic!("assertion failed: `(left == right)` \
                    (left: `{:?}`, right: `{:?}`)", left, right)
        }
    })
}

macro_rules! tryln {
    ($expr:expr) => (try!(
        ($expr).map_err({ |e|
            ($crate::std::convert::From::from(e), file!(), line!())
        })
    ))
}