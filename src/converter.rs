#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
/// Convert f64 to usize
pub fn f64_to_usize(x: f64) -> usize {
    assert!(x >= 0.0, "Value cannot be negative.");
    assert!(
        !((x as usize == usize::MAX) && ((usize::MAX as f64) < x)),
        "Value cannot be bigger than usize::MAX."
    );
    assert!(x.round() == x, "Value can only be a whole number.");
    x as usize
}

/// Convert usize to i32
#[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
pub fn usize_to_i32(x: usize) -> i32 {
    assert!(i32::try_from(x).is_ok(), "Value is too big.");
    x as i32
}

/// Convert i32 to usize
#[allow(clippy::cast_sign_loss)]
pub fn i32_to_usize(x: i32) -> usize {
    assert!(x >= 0, "Value cannot be negative.");
    x as usize
}

/// Convert f64 to i32
#[allow(clippy::cast_possible_truncation)]
pub fn f64_to_i32(x: f64) -> i32 {
    assert!(
        !((x as i32 == i32::MIN) && (f64::from(i32::MIN) > x)),
        "Value cannot be smaller than i32::MIN."
    );
    assert!(
        !((x as i32 == i32::MAX) && (f64::from(i32::MAX) < x)),
        "Value cannot be bigger than i32::MAX."
    );
    assert!(x.round() == x, "Value can only be a whole number.");
    x as i32
}

#[cfg(test)]
mod f64_to_usize {
    use crate::converter::f64_to_usize;

    #[test]
    fn correct() {
        assert_eq!(f64_to_usize(100.0), 100);
        assert_eq!(f64_to_usize(0.0), 0);
    }

    #[test]
    #[should_panic = "Value cannot be negative"]
    fn negative() {
        f64_to_usize(-3.0);
    }

    #[test]
    #[should_panic = "Value cannot be negative"]
    fn negative_edge_case() {
        f64_to_usize(-f64::MIN_POSITIVE);
    }

    #[test]
    #[should_panic = "Value can only be a whole number."]
    fn fraction() {
        f64_to_usize(1.5);
    }

    #[test]
    #[should_panic = "Value cannot be bigger than usize::MAX."]
    fn bigger() {
        f64_to_usize(18_446_744_073_709_551_616.0);
    }

    #[test]
    fn bigger_limit() {
        assert_eq!(
            f64_to_usize(18_446_744_073_709_551_615.0),
            18_446_744_073_709_551_615
        );
    }
}

#[cfg(test)]
mod usize_to_i32 {
    use crate::converter::usize_to_i32;

    #[test]
    fn correct() {
        assert_eq!(usize_to_i32(5), 5);
    }

    #[test]
    fn almost_too_big() {
        assert_eq!(usize_to_i32(2_147_483_647), 2_147_483_647);
    }

    #[test]
    #[should_panic = "Value is too big."]
    fn too_big() {
        usize_to_i32(2_147_483_648);
    }
}

#[cfg(test)]
mod i32_to_usize {
    use crate::converter::i32_to_usize;

    #[test]
    fn correct() {
        assert_eq!(i32_to_usize(5), 5);
    }

    #[test]
    #[should_panic = "Value cannot be negative."]
    fn negative() {
        i32_to_usize(-1);
    }
}

#[cfg(test)]
mod f64_to_i32 {
    use crate::converter::f64_to_i32;
    #[test]
    fn correct() {
        assert_eq!(f64_to_i32(5.0), 5);
    }

    #[test]
    #[should_panic = "Value can only be a whole number."]
    fn fraction() {
        f64_to_i32(2.5);
    }

    #[test]
    #[should_panic = "Value cannot be smaller than i32::MIN."]
    fn smaller() {
        f64_to_i32(-2_147_483_649.0);
    }

    #[test]
    fn smaller_limit() {
        assert_eq!(f64_to_i32(-2_147_483_648.0), -2_147_483_648);
    }

    #[test]
    #[should_panic = "Value cannot be bigger than i32::MAX."]
    fn bigger() {
        f64_to_i32(2_147_483_648.0);
    }

    #[test]
    fn bigger_limit() {
        assert_eq!(f64_to_i32(2_147_483_647.0), 2_147_483_647);
    }
}
