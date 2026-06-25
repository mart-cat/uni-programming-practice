#[derive(Clone, Debug, PartialEq, Eq)]
struct Pair<T> {
    left: T,
    right: T,
}

impl<T> Pair<T> {
    fn new(left: T, right: T) -> Self {
        Self { left, right }
    }

    fn left(&self) -> &T {
        &self.left
    }

    fn right(&self) -> &T {
        &self.right
    }
}

impl Pair<u32> {
    fn sum(&self) -> u32 {
        self.left + self.right
    }
}

trait ReportLine {
    fn report_line(&self) -> String;
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct StudentResult {
    name: String,
    scores: Pair<u32>,
}

impl StudentResult {
    fn new(name: &str, first_score: u32, second_score: u32) -> Self {
        Self {
            name: name.to_string(),
            scores: Pair::new(first_score, second_score),
        }
    }

    fn total(&self) -> u32 {
        self.scores.sum()
    }
}

impl ReportLine for StudentResult {
    fn report_line(&self) -> String {
        format!(
            "{}: {} + {} = {}",
            self.name,
            self.scores.left(),
            self.scores.right(),
            self.total()
        )
    }
}

fn build_report<T: ReportLine>(items: &[T]) -> String {
    items
        .iter()
        .map(|item| item.report_line())
        .collect::<Vec<_>>()
        .join("\n")
}

fn best_result(items: &[StudentResult]) -> Option<&StudentResult> {
    items.iter().max_by_key(|item| item.total())
}

fn main() {
    let results = vec![
        StudentResult::new("Anna", 42, 48),
        StudentResult::new("Boris", 38, 40),
        StudentResult::new("Olga", 50, 47),
    ];

    println!("Progress report");
    println!("{}", build_report(&results));
    if let Some(best) = best_result(&results) {
        println!("Best: {} with {}", best.name, best.total());
    }

    let copied_sample = results[0].clone();
    println!("Cloned sample: {}", copied_sample.report_line());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pair_stores_two_values() {
        let pair = Pair::new(10, 15);
        assert_eq!(*pair.left(), 10);
        assert_eq!(*pair.right(), 15);
        assert_eq!(pair.sum(), 25);
    }

    #[test]
    fn report_line_contains_total_score() {
        let result = StudentResult::new("Anna", 42, 48);
        assert_eq!(result.report_line(), "Anna: 42 + 48 = 90");
    }

    #[test]
    fn generic_report_collects_lines() {
        let results = vec![
            StudentResult::new("Anna", 42, 48),
            StudentResult::new("Boris", 38, 40),
        ];
        let report = build_report(&results);
        assert!(report.contains("Anna: 42 + 48 = 90"));
        assert!(report.contains("Boris: 38 + 40 = 78"));
    }

    #[test]
    fn finds_best_result() {
        let results = vec![
            StudentResult::new("Anna", 42, 48),
            StudentResult::new("Olga", 50, 47),
        ];
        let best = best_result(&results).expect("sample list is not empty");
        assert_eq!(best.name, "Olga");
        assert_eq!(best.total(), 97);
    }
}
