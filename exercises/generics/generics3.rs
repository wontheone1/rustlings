// An imaginary magical school has a new report card generation system written in rust!
// Currently the system only supports creating report cards where the student's grade 
// is represented numerically (e.g. 1.0 -> 5.5). However, the school also issues alphabetical grades
// (A+ -> F-) and needs to be able to print both types of report card!

// Make the necessary code changes to support alphabetical report cards, thereby making the second
// test pass.

pub struct ReportCard<T: std::fmt::Display> {
    pub grade: T,
    pub student_name: String,
    pub student_age: u8,
}

// impl<T: std::fmt::Display> std::fmt::Display for ReportCard<T> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{} ({}) - achieved a grade of {}", &self.student_name, &self.student_age, &self.grade)
//     }
// }

// impl std::fmt::Display for ReportCard<String> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{} ({}) - achieved a grade of {}", &self.student_name, &self.student_age, &self.grade)
//     }
// }
//
// impl std::fmt::Display for ReportCard<f32> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{} ({}) - achieved a grade of {}", &self.student_name, &self.student_age, &self.grade)
//     }
// }

// for methods that need `Display` trait
impl<T: std::fmt::Display> ReportCard<T> {
    pub fn print(&self) -> String {
        format!("{} ({}) - achieved a grade of {}", &self.student_name, &self.student_age, &self.grade)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard {
            grade: 2.1, 
            student_name: "Tom Wriggle".to_string(), 
            student_age: 12,
        };
        assert_eq!(report_card.print(), "Tom Wriggle (12) - achieved a grade of 2.1");
    }

    #[test]
    fn generate_alphabetic_report_card() {
        // TODO: Make sure to change the grade here after you finish the exercise.
        let report_card = ReportCard {
            grade: "A+".to_string(),
            student_name: "Gary Plotter".to_string(), 
            student_age: 11,
        };
        assert_eq!(report_card.print(), "Gary Plotter (11) - achieved a grade of A+");
    }
}

// To find the best solution to this challenge you're going to need to think back to your
// knowledge of traits, specifically Trait Bound Syntax -  you may also need this: "use std::fmt::Display;"
//
// This is definitely harder than the last two exercises! You need to think about not only making the
// ReportCard struct generic, but also the correct property - you will need to change the implementation
// of the struct slightly too...you can do it!
