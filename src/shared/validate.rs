use std::fmt::Display;

pub fn validate_votes(nb_options: usize, grades: &[usize]) -> Result<(), ValidateErr> {
    if nb_options != grades.len() {
        return Err(ValidateErr::MissingGrade(nb_options));
    }

    let sum_grades = grades.iter().copied().sum();
    if sum_grades != nb_options * 3 {
        return Err(ValidateErr::BadSumOfGrades {
            expected: nb_options * 3,
            got: sum_grades,
        });
    }

    if let Some(idx) = grades.iter().find(|grade| **grade > 5) {
        return Err(ValidateErr::GradeSuperiorToFive(*idx));
    }

    Ok(())
}

pub enum ValidateErr {
    MissingGrade(usize /* Expected number of optins */),
    BadSumOfGrades { expected: usize, got: usize },
    GradeSuperiorToFive(usize /* At index */),
}

impl Display for ValidateErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingGrade(_) => {
                f.write_str("Some answer have not been answered. Please check again.")
            },
            Self::BadSumOfGrades { expected, got } => {
                f.write_str(&format!(
                    "Expected to find {expected} points used. Got: {got}"
                ))
            },
            Self::GradeSuperiorToFive(_) => f.write_str("I know what ur doing :)"),
        }
    }
}
