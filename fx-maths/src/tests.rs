use crate::*;

#[test]
fn test_float_macro() {
    assert_eq!(
        Float {
            mantissa: [Digit(0), Digit(0), Digit(0), Digit(0), Digit(0), Digit(0), Digit(0), Digit(0), Digit(0), Digit(0), Digit(0), Digit(0), Digit(6), Digit(9), Digit(4)],
            exponent: 20,
            sign: false
        },
        float!((694) e 20)
    );
    assert_eq!(
        Float {
            mantissa: [Digit(0), Digit(0), Digit(0), Digit(0), Digit(0), Digit(0), Digit(0), Digit(0), Digit(0), Digit(0), Digit(0), Digit(0), Digit(0), Digit(1), Digit(2)],
            exponent: -3,
            sign: true
        },
        float!(-(12) e -3)
    );
    assert_eq!(
        Float {
            mantissa: [Digit(0); 15],
            exponent: -3,
            sign: true
        },
        float!(-(0) e -3)
    );
}

#[test]
fn float_add() {
    let a = float!((5) e 1); //   50
    let b = float!((6) e 0); // +  6
                             // ----
                             //   56
    assert_eq!(a + b, Some(float!((56) e 0)));

    let a = float!((5) e 0); //    5
    let b = float!((5) e 0); // +  5
                             // ----
                             //   10
    assert_eq!(a + b, Some(float!((1) e 1)));
}

#[test]
fn float_mul() {
    let a = float!((2) e 2); //   200
    let b = float!((1) e 1); // ×  10
                             // -----
                             //  2000
    assert_eq!(a * b, Some(float!((2) e 3)));

    let a = float!((5) e 0); //    5
    let b = float!((4) e 0); // ×  4
                             // ----
                             //   20
    assert_eq!(a * b, Some(float!((2) e 1)));

    let a = float!((3) e 0);
    let b = float!((41111111111111) e -14);
    assert_eq!(a * b, Some(float!((123333333333333) e -14)));
}
