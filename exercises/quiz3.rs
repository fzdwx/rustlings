// quiz3.rs
// This quiz tests:
// - Generics
// - Traits
/// 一所想象中的魔法学校有一个用Rust编写的新的成绩单生成系统!
/// 目前，该系统只支持创建以数字表示学生成绩的成绩单（例如：1.0 -> 5.5）。
/// 然而，学校也会发放按字母顺序排列的成绩单（A+ -> F-），
/// 并且需要能够打印这两种类型的成绩单!

/// 在结构 ReportCard 和 impl 块中做必要的代码修改，
/// 以支持按字母顺序排列的成绩单。
/// 将第二个测试中的成绩改为 "A+"，以表明你的修改允许按字母顺序排列成绩。

// Execute `rustlings hint quiz3` or use the `hint` watch subcommand for a hint.


pub struct ReportCard<T> {
    pub grade: T,
    pub student_name: String,
    pub student_age: u8,
}

impl<T: std::fmt::Display> ReportCard<T> {
    pub fn print(&self) -> String {
        format!("{} ({}) - achieved a grade of {}",
                &self.student_name, &self.student_age, &self.grade)
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
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of 2.1"
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        // TODO: Make sure to change the grade here after you finish the exercise.
        let report_card = ReportCard {
            grade: "A+",
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+"
        );
    }
}
