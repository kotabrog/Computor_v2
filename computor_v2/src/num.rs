#[derive(Debug, PartialEq, Clone)]
pub enum Num {
    Float(f64),
}


impl Num {
    pub fn from_string_to_float(string: &String) -> Result<Num, String> {
        match string.parse() {
            Ok(n) => Ok(Num::Float(n)),
            Err(_) => {
                Err("Conversion Failure".to_string())
            }
        }
    }


    pub fn checked_value(&self) -> Result<&Num, String> {
        match self {
            Num::Float(n) => {
                if n.is_finite() {
                    return Ok(&self)
                } else {
                    Err(format!("The calculation resulted in '{}'.", n))
                }
            }
        }
    }


    pub fn supported_add(&self, rhs: &Num) -> Result<Num, String> {
        match (self, rhs) {
            (Num::Float(l), Num::Float(r)) => {
                Ok(Num::Float(l + r))
            },
        }
    }


    pub fn supported_sub(&self, rhs: &Num) -> Result<Num, String> {
        match (self, rhs) {
            (Num::Float(l), Num::Float(r)) => {
                Ok(Num::Float(l - r))
            },
        }
    }


    pub fn supported_mul(&self, rhs: &Num) -> Result<Num, String> {
        match (self, rhs) {
            (Num::Float(l), Num::Float(r)) => {
                Ok(Num::Float(l * r))
            },
        }
    }


    pub fn supported_div(&self, rhs: &Num) -> Result<Num, String> {
        match (self, rhs) {
            (Num::Float(l), Num::Float(r)) => {
                Ok(Num::Float(l / r))
            },
        }
    }


    pub fn supported_rem(&self, rhs: &Num) -> Result<Num, String> {
        match (self, rhs) {
            (Num::Float(l), Num::Float(r)) => {
                Ok(Num::Float(l.rem_euclid(*r)))
            },
        }
    }


    pub fn supported_pow(&self, rhs: &Num) -> Result<Num, String> {
        match (self, rhs) {
            (Num::Float(l), Num::Float(r)) => {
                Ok(Num::Float(l.powf(*r)))
            },
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_string_to_float_int() {
        let string = "1".to_string();
        assert_eq!(Num::from_string_to_float(&string),
                   Ok(Num::Float(1.0)));
    }

    #[test]
    fn from_string_to_float_float() {
        let string = "1.1".to_string();
        assert_eq!(Num::from_string_to_float(&string),
                   Ok(Num::Float(1.1)));
    }

    #[test]
    fn from_string_to_float_inf() {
        let string = "inf".to_string();
        assert_eq!(Num::from_string_to_float(&string),
                   Ok(Num::Float(f64::INFINITY)));
    }

    #[test]
    fn from_string_to_float_error() {
        let string = "hello".to_string();
        assert_eq!(Num::from_string_to_float(&string),
                   Err("Conversion Failure".to_string()));
    }

    #[test]
    fn checked_value_float_normal() {
        let n = Num::Float(1.0);
        assert_eq!(n.checked_value(),
                   Ok(&Num::Float(1.0)));
    }

    #[test]
    fn checked_value_float_inf() {
        let n = Num::Float(f64::INFINITY);
        assert_eq!(n.checked_value(),
                   Err(format!("The calculation resulted in '{}'.", f64::INFINITY)));
    }

    #[test]
    fn checked_value_float_nan() {
        let n = Num::Float(f64::NAN);
        assert_eq!(n.checked_value(),
                   Err(format!("The calculation resulted in '{}'.", f64::NAN)));
    }

    #[test]
    fn supported_add_float_float() {
        let lhs = Num::Float(1.0);
        let rhs = Num::Float(2.0);
        assert_eq!(lhs.supported_add(&rhs),
                   Ok(Num::Float(3.0)));
    }

    #[test]
    fn supported_add_float_float_inf() {
        let lhs = Num::Float(1.0);
        let rhs = Num::Float(f64::INFINITY);
        assert_eq!(lhs.supported_add(&rhs),
                   Ok(Num::Float(f64::INFINITY)));
    }

    #[test]
    fn supported_add_float_float_to_inf() {
        let lhs = Num::Float(1.7976931348623157e308);
        let rhs = Num::Float(1.7976931348623157e308);
        assert_eq!(lhs.supported_add(&rhs),
                   Ok(Num::Float(f64::INFINITY)));
    }

    #[test]
    fn supported_sub_float_float() {
        let lhs = Num::Float(1.0);
        let rhs = Num::Float(2.0);
        assert_eq!(lhs.supported_sub(&rhs),
                   Ok(Num::Float(-1.0)));
    }

    #[test]
    fn supported_sub_float_float_inf() {
        let lhs = Num::Float(1.0);
        let rhs = Num::Float(f64::INFINITY);
        assert_eq!(lhs.supported_sub(&rhs),
                   Ok(Num::Float(-f64::INFINITY)));
    }

    #[test]
    fn supported_mul_float_float() {
        let lhs = Num::Float(3.0);
        let rhs = Num::Float(2.0);
        assert_eq!(lhs.supported_mul(&rhs),
                   Ok(Num::Float(6.0)));
    }

    #[test]
    fn supported_mul_float_float_inf() {
        let lhs = Num::Float(1.0);
        let rhs = Num::Float(f64::INFINITY);
        assert_eq!(lhs.supported_mul(&rhs),
                   Ok(Num::Float(f64::INFINITY)));
    }

    #[test]
    fn supported_div_float_float() {
        let lhs = Num::Float(3.0);
        let rhs = Num::Float(2.0);
        assert_eq!(lhs.supported_div(&rhs),
                   Ok(Num::Float(1.5)));
    }

    #[test]
    fn supported_div_float_float_inf() {
        let lhs = Num::Float(1.0);
        let rhs = Num::Float(f64::INFINITY);
        assert_eq!(lhs.supported_div(&rhs),
                   Ok(Num::Float(0.0)));
    }

    #[test]
    fn supported_div_float_float_zero() {
        let lhs = Num::Float(1.0);
        let rhs = Num::Float(0.0);
        assert_eq!(lhs.supported_div(&rhs),
                   Ok(Num::Float(f64::INFINITY)));
    }

    #[test]
    fn supported_rem_float_float() {
        let lhs = Num::Float(5.0);
        let rhs = Num::Float(4.0);
        assert_eq!(lhs.supported_rem(&rhs),
                   Ok(Num::Float(1.0)));
    }

    #[test]
    fn supported_rem_float_float_r_minus() {
        let lhs = Num::Float(5.0);
        let rhs = Num::Float(-4.0);
        assert_eq!(lhs.supported_rem(&rhs),
                   Ok(Num::Float(1.0)));
    }

    #[test]
    fn supported_rem_float_float_l_minus() {
        let lhs = Num::Float(-5.0);
        let rhs = Num::Float(4.0);
        assert_eq!(lhs.supported_rem(&rhs),
                   Ok(Num::Float(3.0)));
    }

    #[test]
    fn supported_rem_float_float_r_l_minus() {
        let lhs = Num::Float(-5.0);
        let rhs = Num::Float(-4.0);
        assert_eq!(lhs.supported_rem(&rhs),
                   Ok(Num::Float(3.0)));
    }

    #[test]
    fn supported_rem_float_float_inf() {
        let lhs = Num::Float(1.0);
        let rhs = Num::Float(f64::INFINITY);
        assert_eq!(lhs.supported_rem(&rhs),
                   Ok(Num::Float(1.0)));
    }

    #[test]
    fn supported_rem_float_float_zero() {
        let lhs = Num::Float(1.0);
        let rhs = Num::Float(0.0);
        assert_eq!(lhs.supported_rem(&rhs).unwrap_or(Num::Float(0.0)).checked_value(),
                   Err(format!("The calculation resulted in '{}'.", f64::NAN)));
    }

    #[test]
    fn supported_pow_float_float() {
        let lhs = Num::Float(5.0);
        let rhs = Num::Float(3.0);
        assert_eq!(lhs.supported_pow(&rhs),
                   Ok(Num::Float(125.0)));
    }

    #[test]
    fn supported_pow_float_float_r_minus() {
        let lhs = Num::Float(5.0);
        let rhs = Num::Float(-1.0);
        assert_eq!(lhs.supported_pow(&rhs),
                   Ok(Num::Float(0.2)));
    }

    #[test]
    fn supported_pow_float_float_l_minus() {
        let lhs = Num::Float(-5.0);
        let rhs = Num::Float(3.0);
        assert_eq!(lhs.supported_pow(&rhs),
                   Ok(Num::Float(-125.0)));
    }

    #[test]
    fn supported_pow_float_float_r_l_minus() {
        let lhs = Num::Float(-5.0);
        let rhs = Num::Float(-1.0);
        assert_eq!(lhs.supported_pow(&rhs),
                   Ok(Num::Float(-0.2)));
    }

    #[test]
    fn supported_pow_float_float_inf() {
        let lhs = Num::Float(1.0);
        let rhs = Num::Float(f64::INFINITY);
        assert_eq!(lhs.supported_pow(&rhs),
                   Ok(Num::Float(1.0)));
    }

    #[test]
    fn supported_pow_float_float_zero() {
        let lhs = Num::Float(1.0);
        let rhs = Num::Float(0.0);
        assert_eq!(lhs.supported_pow(&rhs),
                   Ok(Num::Float(1.0)));
    }
}
