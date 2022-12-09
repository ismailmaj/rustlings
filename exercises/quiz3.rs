// quiz3.rs
// This quiz tests:
// - Generics
// - Traits
// An imaginary magical school has a new report card generation system written in Rust!
// Currently the system only supports creating report cards where the student's grade
// is represented numerically (e.g. 1.0 -> 5.5).
// However, the school also issues alphabetical grades (A+ -> F-) and needs
// to be able to print both types of report card!

// Make the necessary code changes in the struct ReportCard and the impl block
// to support alphabetical report cards. Change the Grade in the second test to "A+"
// to show that your changes allow alphabetical grades.

// Execute `rustlings hint quiz3` or use the `hint` watch subcommand for a hint.
use std::fmt::{self, Display};

#[derive(Debug)]
pub enum AlphabeticalGrade {
    A,
    B,
    C,
    D,
    E,
    F,
    Aplus,
    Bplus,
    Cplus,
    Dplus,
    Eplus,
    Fplus,
    Aminus,
    Bminus,
    Cminus,
    Dminus,
    Eminus,
    Fminus,
}

impl Display for AlphabeticalGrade {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AlphabeticalGrade::A => write!(f, "A"),
            AlphabeticalGrade::B => write!(f, "B"),
            AlphabeticalGrade::C => write!(f, "C"),
            AlphabeticalGrade::D => write!(f, "D"),
            AlphabeticalGrade::E => write!(f, "E"),
            AlphabeticalGrade::F => write!(f, "F"),
            AlphabeticalGrade::Aplus => write!(f, "A+"),
            AlphabeticalGrade::Bplus => write!(f, "B+"),
            AlphabeticalGrade::Cplus => write!(f, "C+"),
            AlphabeticalGrade::Dplus => write!(f, "D+"),
            AlphabeticalGrade::Eplus => write!(f, "E+"),
            AlphabeticalGrade::Fplus => write!(f, "F+"),
            AlphabeticalGrade::Aminus => write!(f, "A-"),
            AlphabeticalGrade::Bminus => write!(f, "B-"),
            AlphabeticalGrade::Cminus => write!(f, "C-"),
            AlphabeticalGrade::Dminus => write!(f, "D-"),
            AlphabeticalGrade::Eminus => write!(f, "E-"),
            AlphabeticalGrade::Fminus => write!(f, "F-"),
        }
    }
}

#[derive(Debug)]
pub enum Grade {
    Numerical(f32),
    Alphabetical(AlphabeticalGrade),
}
impl Display for Grade {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Grade::Numerical(grade) => write!(f, "{}", grade),
            Grade::Alphabetical(grade) => write!(f, "{}", grade),
        }
    }
}

pub struct ReportCard {
    pub grade: Grade,
    pub student_name: String,
    pub student_age: u8,
}

impl ReportCard {
    pub fn print(&self) -> String {
        format!(
            "{} ({}) - achieved a grade of {}",
            &self.student_name, &self.student_age, &self.grade
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard {
            grade: Grade::Numerical(2.1),
            student_name: "Tom Wriggle".to_string(),
            student_age: 12,
        };
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of 2.1"
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        // TODO: Make sure to change the grade here after you finish the exercise.
        let report_card = ReportCard {
            grade: Grade::Alphabetical(AlphabeticalGrade::Aplus),
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+"
        );
    }
}
