use std::fmt;


#[derive(Debug, PartialEq, Clone)]
pub struct Complex {
    pub r: f64,
    pub z: f64,
}


#[derive(Debug, PartialEq, Clone)]
pub enum Num {
    Float(f64),
    Complex(Box<Complex>)
}


impl Complex {
    fn from_two_float(r: f64, z: f64) -> Complex {
        Complex { r, z }
    }

    fn new() -> Complex {
        Self::from_two_float(0.0, 1.0)
    }

    fn is_float(&self) -> bool {
        self.z == 0.0
    }

    fn to_float(&self) -> Option<f64> {
        if self.is_float() {
            Some(self.r)
        } else {
            None
        }
    }
}


impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.z.is_sign_positive() {
            if (self.r, self.z) == (0.0, 0.0) {
                write!(f, "0")
            } else if (self.r, self.z) == (0.0, 1.0) {
                write!(f, "i")
            } else if self.r == 0.0 {
                write!(f, "{}i", self.z)
            } else if self.z == 0.0 {
                write!(f, "{}", self.r)
            } else if self.z == 1.0 {
                write!(f, "{} + i", self.r)
            } else {
                write!(f, "{} + {}i", self.r, self.z)
            }
        } else {
            if (self.r, self.z) == (0.0, 0.0) {
                write!(f, "0")
            } else if (self.r, self.z) == (0.0, -1.0) {
                write!(f, "-i")
            } else if self.r == 0.0 {
                write!(f, "{}i", self.z)
            } else if self.z == 0.0 {
                write!(f, "{}", self.r)
            } else if self.z == -1.0 {
                write!(f, "{} - i", self.r)
            } else {
                write!(f, "{} - {}i", self.r, -self.z)
            }
        }
    }
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


    pub fn from_two_float(r: f64, z: f64) -> Num {
        if z == 0.0 {
            Num::Float(r)
        } else {
            Self::from_two_float_to_complex(r, z)
        }
    }


    pub fn from_two_float_to_complex(r: f64, z: f64) -> Num {
        Num::Complex(Box::new(Complex::from_two_float(r, z)))
    }


    pub fn new_complex() -> Num {
        Self::from_two_float_to_complex(0.0, 1.0)
    }


    pub fn checked_value(&self) -> Result<&Num, String> {
        match self {
            Num::Float(n) => {
                if n.is_finite() {
                    Ok(&self)
                } else {
                    Err(format!("The calculation resulted in '{}'.", n))
                }
            },
            Num::Complex(b) => {
                if !b.r.is_finite() {
                    Err(format!("The calculation resulted in '{}'.", b.r))
                }
                else if !b.z.is_finite() {
                    Err(format!("The calculation resulted in '{}'.", b.z))
                } else {
                    Ok(&self)
                }
            },
        }
    }


    pub fn supported_add(&self, rhs: &Num) -> Result<Num, String> {
        match (self, rhs) {
            (Num::Float(l), Num::Float(r))
                => Ok(Num::Float(l + r)),
            (Num::Float(l), Num::Complex(r))
                => Ok(Num::from_two_float(l + r.r, r.z)),
            (Num::Complex(l), Num::Float(r))
                => Ok(Num::from_two_float(l.r + r, l.z)),
            (Num::Complex(l), Num::Complex(r))
                => Ok(Num::from_two_float(l.r + r.r, l.z + r.z)),
        }
    }


    pub fn supported_sub(&self, rhs: &Num) -> Result<Num, String> {
        match (self, rhs) {
            (Num::Float(l), Num::Float(r))
                => Ok(Num::Float(l - r)),
            (Num::Float(l), Num::Complex(r))
                => Ok(Num::from_two_float(l - r.r, -r.z)),
            (Num::Complex(l), Num::Float(r))
                => Ok(Num::from_two_float(l.r - r, l.z)),
            (Num::Complex(l), Num::Complex(r))
                => Ok(Num::from_two_float(l.r - r.r, l.z - r.z)),
        }
    }


    pub fn supported_mul(&self, rhs: &Num) -> Result<Num, String> {
        match (self, rhs) {
            (Num::Float(l), Num::Float(r))
                =>Ok(Num::Float(l * r)),
            (Num::Float(l), Num::Complex(r))
                => Ok(Num::from_two_float(l * r.r, l * r.z)),
            (Num::Complex(l), Num::Float(r))
                => Ok(Num::from_two_float(l.r * r, l.z * r)),
            (Num::Complex(l), Num::Complex(r))
                => Ok(Num::from_two_float(l.r * r.r - l.z * r.z, l.r * r.z + l.z * r.r)),
        }
    }


    pub fn supported_div(&self, rhs: &Num) -> Result<Num, String> {
        match (self, rhs) {
            (Num::Float(l), Num::Float(r))
                => Ok(Num::Float(l / r)),
            (Num::Float(l), Num::Complex(r)) => {
                let v = r.r * r.r + r.z * r.z;
                Ok(Num::from_two_float(l * r.r / v, - l * r.z / v))
            },
            (Num::Complex(l), Num::Float(r))
                => Ok(Num::from_two_float(l.r / r, l.z / r)),
            (Num::Complex(_), Num::Complex(r)) => {
                let v = r.r * r.r + r.z * r.z;
                let r2 = Num::from_two_float(r.r, -r.z);
                Ok(self.supported_mul(&r2).unwrap()
                        .supported_div(&Num::Float(v)).unwrap())
            },
        }
    }


    pub fn supported_rem(&self, rhs: &Num) -> Result<Num, String> {
        match (self, rhs) {
            (Num::Float(l), Num::Float(r))
                => Ok(Num::Float(l.rem_euclid(*r))),
            (Num::Complex(l), Num::Float(r))
                => Ok(Num::from_two_float(l.r.rem_euclid(*r), l.z.rem_euclid(*r))),
            _ => Err(format!("Unsupported operator ({}) % ({})", self, rhs))
        }
    }


    pub fn supported_pow(&self, rhs: &Num) -> Result<Num, String> {
        match (self, rhs) {
            (Num::Float(l), Num::Float(r)) => {
                Ok(Num::Float(l.powf(*r)))
            },
            _ => Err(format!("Unsupported operator ({}) ^ ({})", self, rhs))
        }
    }


    pub fn is_need_sign_reverse(&self) -> bool {
        match self {
            Num::Float(n) => n.is_sign_negative(),
            Num::Complex(n) => {
                (n.r == 0.0 && n.z.is_sign_negative())
                    || n.r.is_sign_negative()
            }
        }
    }

    pub fn is_need_paren_to_display(&self) -> bool {
        if self.is_need_sign_reverse() {
            true
        } else {
            match &self {
                Num::Float(_) => false,
                Num::Complex(n) => n.r != 0.0 && n.z != 0.0,
            }
        }
    }

    pub fn reverse_sign(&self) -> Num {
        match &self {
            Num::Float(n) => Num::Float(-n),
            Num::Complex(n) => Num::from_two_float_to_complex(-n.r, -n.z),
        }
    }
}


impl fmt::Display for Num {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Num::Float(n) => write!(f, "{}", n),
            Num::Complex(n) => write!(f, "{}", n),
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
    fn from_two_float_only_i() {
        let r = 0.0;
        let z = 1.0;
        assert_eq!(Num::from_two_float(r, z),
                   Num::Complex(Box::new(Complex { r, z })));
    }

    #[test]
    fn from_two_float_only_r() {
        let r = 1.0;
        let z = 0.0;
        assert_eq!(Num::from_two_float(r, z),
                   Num::Float(1.0));
    }

    #[test]
    fn from_two_float_only_r_minus() {
        let r = 1.0;
        let z = -0.0;
        assert_eq!(Num::from_two_float(r, z),
                   Num::Float(1.0));
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
    fn checked_value_complex_normal() {
        let n = Num::from_two_float(2.0, 1.0);
        assert_eq!(n.checked_value(),
                   Ok(&n));
    }

    #[test]
    fn checked_value_complex_inf() {
        let n = Num::from_two_float(2.0, f64::INFINITY);
        assert_eq!(n.checked_value(),
                   Err(format!("The calculation resulted in '{}'.", f64::INFINITY)));
    }

    #[test]
    fn checked_value_complex_nan() {
        let n = Num::from_two_float(f64::NAN, 1.0);
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
    fn supported_add_float_complex() {
        let lhs = Num::Float(1.0);
        let rhs = Num::from_two_float(-2.0, 1.0);
        assert_eq!(lhs.supported_add(&rhs),
                   Ok(Num::from_two_float(-1.0, 1.0)));
    }

    #[test]
    fn supported_add_complex_float() {
        let lhs = Num::from_two_float(-2.0, 1.0);
        let rhs = Num::Float(1.0);
        assert_eq!(lhs.supported_add(&rhs),
                   Ok(Num::from_two_float(-1.0, 1.0)));
    }

    #[test]
    fn supported_add_complex_complex() {
        let lhs = Num::from_two_float(-2.0, 1.0);
        let rhs = Num::from_two_float(3.0, 1.0);
        assert_eq!(lhs.supported_add(&rhs),
                   Ok(Num::from_two_float(1.0, 2.0)));
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
    fn supported_sub_float_complex() {
        let lhs = Num::Float(1.0);
        let rhs = Num::from_two_float(-2.0, 1.0);
        assert_eq!(lhs.supported_sub(&rhs),
                   Ok(Num::from_two_float(3.0, -1.0)));
    }

    #[test]
    fn supported_sub_complex_float() {
        let lhs = Num::from_two_float(-2.0, 1.0);
        let rhs = Num::Float(1.0);
        assert_eq!(lhs.supported_sub(&rhs),
                   Ok(Num::from_two_float(-3.0, 1.0)));
    }

    #[test]
    fn supported_sub_complex_complex() {
        let lhs = Num::from_two_float(-2.0, 1.0);
        let rhs = Num::from_two_float(3.0, 1.0);
        assert_eq!(lhs.supported_sub(&rhs),
                   Ok(Num::from_two_float(-5.0, 0.0)));
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
    fn supported_mul_float_complex() {
        let lhs = Num::Float(3.0);
        let rhs = Num::from_two_float(-2.0, 1.0);
        assert_eq!(lhs.supported_mul(&rhs),
                   Ok(Num::from_two_float(-6.0, 3.0)));
    }

    #[test]
    fn supported_mul_complex_float() {
        let lhs = Num::from_two_float(-2.0, 1.0);
        let rhs = Num::Float(3.0);
        assert_eq!(lhs.supported_mul(&rhs),
                   Ok(Num::from_two_float(-6.0, 3.0)));
    }

    #[test]
    fn supported_mul_complex_complex() {
        let lhs = Num::from_two_float(-2.0, 1.0);
        let rhs = Num::from_two_float(3.0, 1.0);
        assert_eq!(lhs.supported_mul(&rhs),
                   Ok(Num::from_two_float(-7.0, 1.0)));
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
    fn supported_div_float_complex() {
        let lhs = Num::Float(3.0);
        let rhs = Num::from_two_float(-2.0, 1.0);
        assert_eq!(lhs.supported_div(&rhs),
                   Ok(Num::from_two_float(-6.0 / 5.0, -3.0 / 5.0)));
    }

    #[test]
    fn supported_div_complex_float() {
        let lhs = Num::from_two_float(-2.0, 1.0);
        let rhs = Num::Float(3.0);
        assert_eq!(lhs.supported_div(&rhs),
                   Ok(Num::from_two_float(-2.0 / 3.0, 1.0 / 3.0)));
    }

    #[test]
    fn supported_div_complex_complex() {
        let lhs = Num::from_two_float(-2.0, 1.0);
        let rhs = Num::from_two_float(3.0, 1.0);
        assert_eq!(lhs.supported_div(&rhs),
                   Ok(Num::from_two_float(-5.0 / 10.0, 5.0 / 10.0)));
    }

    #[test]
    fn supported_div_complex_float_zero() {
        let lhs = Num::from_two_float(-2.0, 1.0);
        let rhs = Num::Float(0.0);
        assert_eq!(lhs.supported_div(&rhs),
                    Ok(Num::from_two_float(-f64::INFINITY, f64::INFINITY)));
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
    fn supported_rem_complex_float() {
        let lhs = Num::from_two_float(5.0, 6.0);
        let rhs = Num::Float(4.0);
        assert_eq!(lhs.supported_rem(&rhs),
                    Ok(Num::from_two_float(1.0, 2.0)));
    }

    #[test]
    fn supported_rem_complex_float_r_minus() {
        let lhs = Num::from_two_float(5.0, 6.0);
        let rhs = Num::Float(-4.0);
        assert_eq!(lhs.supported_rem(&rhs),
                    Ok(Num::from_two_float(1.0, 2.0)));
    }

    #[test]
    fn supported_rem_complex_float_l_minus() {
        let lhs = Num::from_two_float(-5.0, -6.0);
        let rhs = Num::Float(4.0);
        assert_eq!(lhs.supported_rem(&rhs),
                    Ok(Num::from_two_float(3.0, 2.0)));
    }

    #[test]
    fn supported_rem_complex_float_r_l_minus() {
        let lhs = Num::from_two_float(-5.0, -6.0);
        let rhs = Num::Float(-4.0);
        assert_eq!(lhs.supported_rem(&rhs),
                    Ok(Num::from_two_float(3.0, 2.0)));
    }

    #[test]
    fn supported_rem_complex_float_zero() {
        let lhs = Num::from_two_float(-5.0, -6.0);
        let rhs = Num::Float(0.0);
        assert_eq!(lhs.supported_rem(&rhs).unwrap_or(Num::Float(0.0)).checked_value(),
                   Err(format!("The calculation resulted in '{}'.", f64::NAN)));
    }

    #[test]
    fn supported_rem_error_float_complex() {
        let lhs = Num::Float(0.0);
        let rhs = Num::from_two_float(2.0, 3.0);
        assert_eq!(lhs.supported_rem(&rhs),
                   Err(format!("Unsupported operator ({}) % ({})", lhs, rhs)));
    }

    #[test]
    fn supported_rem_error_complex_complex() {
        let lhs = Num::from_two_float(1.0, 3.0);
        let rhs = Num::from_two_float(2.0, 3.0);
        assert_eq!(lhs.supported_rem(&rhs),
                   Err(format!("Unsupported operator ({}) % ({})", lhs, rhs)));
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

    #[test]
    fn supported_pow_error_complex_float() {
        let lhs = Num::Float(0.0);
        let rhs = Num::from_two_float(2.0, 3.0);
        assert_eq!(lhs.supported_pow(&rhs),
                   Err(format!("Unsupported operator ({}) ^ ({})", lhs, rhs)));
    }

    #[test]
    fn supported_pow_error_float_complex() {
        let lhs = Num::Float(0.0);
        let rhs = Num::from_two_float(2.0, 3.0);
        assert_eq!(lhs.supported_pow(&rhs),
                   Err(format!("Unsupported operator ({}) ^ ({})", lhs, rhs)));
    }

    #[test]
    fn supported_pow_error_complex_complex() {
        let lhs = Num::from_two_float(1.0, 3.0);
        let rhs = Num::from_two_float(2.0, 3.0);
        assert_eq!(lhs.supported_pow(&rhs),
                   Err(format!("Unsupported operator ({}) ^ ({})", lhs, rhs)));
    }

    #[test]
    fn fmt_float_plus() {
        let num = Num::Float(2.0);
        assert_eq!(format!("{}", num), "2".to_string());
    }

    #[test]
    fn fmt_float_minus() {
        let num = Num::Float(-2.0);
        assert_eq!(format!("{}", num), "-2".to_string());
    }

    #[test]
    fn fmt_complex_zero_zero() {
        let num = Num::from_two_float_to_complex(0.0, 0.0);
        assert_eq!(format!("{}", num), "0".to_string());
    }

    #[test]
    fn fmt_complex_zero_one() {
        let num = Num::from_two_float_to_complex(0.0, 1.0);
        assert_eq!(format!("{}", num), "i".to_string());
    }

    #[test]
    fn fmt_complex_zero_n() {
        let num = Num::from_two_float_to_complex(0.0, 2.0);
        assert_eq!(format!("{}", num), "2i".to_string());
    }

    #[test]
    fn fmt_complex_n_zero() {
        let num = Num::from_two_float_to_complex(2.0, 0.0);
        assert_eq!(format!("{}", num), "2".to_string());
    }

    #[test]
    fn fmt_complex_n_one() {
        let num = Num::from_two_float_to_complex(2.0, 1.0);
        assert_eq!(format!("{}", num), "2 + i".to_string());
    }

    #[test]
    fn fmt_complex_n_m() {
        let num = Num::from_two_float_to_complex(2.0, 3.0);
        assert_eq!(format!("{}", num), "2 + 3i".to_string());
    }

    #[test]
    fn fmt_complex_neg_zero_zero() {
        let num = Num::from_two_float_to_complex(0.0, -0.0);
        assert_eq!(format!("{}", num), "0".to_string());
    }

    #[test]
    fn fmt_complex_neg_zero_one() {
        let num = Num::from_two_float_to_complex(0.0, -1.0);
        assert_eq!(format!("{}", num), "-i".to_string());
    }

    #[test]
    fn fmt_complex_neg_zero_n() {
        let num = Num::from_two_float_to_complex(0.0, -2.0);
        assert_eq!(format!("{}", num), "-2i".to_string());
    }

    #[test]
    fn fmt_complex_neg_n_zero() {
        let num = Num::from_two_float_to_complex(2.0, -0.0);
        assert_eq!(format!("{}", num), "2".to_string());
    }

    #[test]
    fn fmt_complex_neg_n_one() {
        let num = Num::from_two_float_to_complex(2.0, -1.0);
        assert_eq!(format!("{}", num), "2 - i".to_string());
    }

    #[test]
    fn fmt_complex_neg_n_m() {
        let num = Num::from_two_float_to_complex(2.0, -3.0);
        assert_eq!(format!("{}", num), "2 - 3i".to_string());
    }
}
