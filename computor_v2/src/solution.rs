use crate::equation::{Equation, Term};


impl Equation {
    pub fn solution(&self) -> Result<String, String> {
        if !self.expr.is_empty() && self.expr[0].degree < 0 {
            return Ok(format!("Negative integer powers are not supported."))
        }
        match self.degree {
            0 => Ok(self.degree_0_solution()),
            1 => self.degree_1_solution(),
            2 => self.degree_2_solution(),
            _ => Ok(format!("The polynomial degree is strictly greater than 2, I can't solve."))
        }
    }

    fn degree_0_solution(&self) -> String {
        if self.expr.is_empty() || self.expr[0].coefficient == 0.0 {
            format!("The solution is an arbitrary real number.")
        } else {
            format!("There is no solution.")
        }
    }

    fn make_terms_no_gaps(terms: &Vec<Term>, degree: i64) -> Vec<Term> {
        let mut vec = Vec::new();
        let mut index = 0;
        for i in 0..=degree {
            if terms[index].degree == i {
                vec.push(terms[index].clone());
                index += 1;
            } else {
                vec.push(Term { coefficient: 0.0, degree: i });
            }
        }
        vec
    }

    fn degree_1_solution(&self) -> Result<String, String> {
        // ax + b = 0
        let vec = Self::make_terms_no_gaps(&self.expr, self.degree);
        let a = vec[1].coefficient;
        let b = - vec[0].coefficient;
        let mut value = b / a;
        if !value.is_finite() {
            Err(format!("The calculation resulted in '{}'.", value))
        } else {
            if value == 0.0 {
                value = 0.0;
            }
            Ok(format!("Solution:\n{}", value))
        }
    }

    fn degree_2_solution(&self) -> Result<String, String> {
        let vec = Self::make_terms_no_gaps(&self.expr, self.degree);
        let discriminant = Self::degree_2_discriminant(&vec)?;
        if discriminant == 0.0 {
            Self::degree_2_solution_one(&vec)
        } else if discriminant.is_sign_positive() {
            Self::degree_2_solution_two(&vec, discriminant)
        } else {
            Self::degree_2_solution_complex(&vec, discriminant)
        }
    }

    fn degree_2_discriminant(terms: &Vec<Term>) -> Result<f64, String> {
        let c = &terms[0].coefficient;
        let b = &terms[1].coefficient;
        let a = &terms[2].coefficient;
        let value = b * b - 4.0 * a * c;
        if value.is_finite() {
            Ok(value)
        } else {
            Err(format!("The calculation resulted in '{}'.", value))
        }
    }

    fn degree_2_solution_one(terms: &Vec<Term>) -> Result<String, String> {
        let b = &terms[1].coefficient;
        let a = &terms[2].coefficient;
        let value = (-b) / (2.0 * a);
        if value.is_finite() {
            Ok(format!("Only one solution on R:\n{}", value))
        } else {
            Err(format!("The calculation resulted in '{}'.", value))
        }
    }

    fn degree_2_solution_two(terms: &Vec<Term>, discriminant: f64) -> Result<String, String> {
        let b = &terms[1].coefficient;
        let a = &terms[2].coefficient;
        let sqrt_d = discriminant.sqrt();
        let value1 = (-b + sqrt_d) / (2.0 * a);
        let value2 = (-b - sqrt_d) / (2.0 * a);
        if value1.is_finite() && value2.is_finite() {
            Ok(format!("Two solutions on R:\n{}\n{}", value1, value2))
        } else if value2.is_finite(){
            Err(format!("The calculation resulted in '{}'.", value1))
        } else {
            Err(format!("The calculation resulted in '{}'.", value2))
        }
    }

    fn degree_2_solution_complex(terms: &Vec<Term>, discriminant: f64) -> Result<String, String> {
        let b = &terms[1].coefficient;
        let a = &terms[2].coefficient;
        let sqrt_d = (-discriminant).sqrt();
        let r_value = (-b) / (2.0 * a);
        let z_value = (sqrt_d / (2.0 * a)).abs();
        if r_value.is_finite() && z_value.is_finite() {
            if r_value == 0.0 && z_value == 1.0{
                Ok(format!("Two solutions on C:\n± i"))
            } else if r_value == 0.0 {
                Ok(format!("Two solutions on C:\n± {}i", z_value))
            } else if z_value == 1.0 {
                Ok(format!("Two solutions on C:\n{} ± i", r_value))
            } else {
                Ok(format!("Two solutions on C:\n{} ± {}i", r_value, z_value))
            }
        } else if z_value.is_finite(){
            Err(format!("The calculation resulted in '{}'.", r_value))
        } else {
            Err(format!("The calculation resulted in '{}'.", z_value))
        }
    }
}
