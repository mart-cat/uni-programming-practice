use std::io;

fn read_trimmed_line(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input line");
    input.trim().to_string()
}

fn parse_year(text: &str) -> u32 {
    text.trim()
        .parse::<u32>()
        .expect("year must be a positive integer")
}

fn calculate_age(birth_year: u32, current_year: u32) -> u32 {
    current_year.saturating_sub(birth_year)
}

fn build_student_card(name: &str, group: &str, birth_year: u32, current_year: u32) -> String {
    let age = calculate_age(birth_year, current_year);
    format!(
        "--- Student card ---\nName: {name}\nGroup: {group}\nBirth year: {birth_year}\nCurrent year: {current_year}\nAge: {age}"
    )
}

fn main() {
    let name = read_trimmed_line("Enter student name:");
    let group = read_trimmed_line("Enter group:");
    let birth_year_text = read_trimmed_line("Enter birth year:");
    let current_year_text = read_trimmed_line("Enter current year:");

    let birth_year = parse_year(&birth_year_text);
    let current_year = parse_year(&current_year_text);

    let card = build_student_card(&name, &group, birth_year, current_year);
    println!("{}", card);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_age_for_normal_years() {
        assert_eq!(calculate_age(2006, 2026), 20);
    }

    #[test]
    fn parses_year_with_spaces() {
        assert_eq!(parse_year(" 2026 "), 2026);
    }

    #[test]
    fn builds_card_with_expected_lines() {
        let card = build_student_card("Anna", "RUST-101", 2006, 2026);
        assert!(card.contains("Name: Anna"));
        assert!(card.contains("Group: RUST-101"));
        assert!(card.contains("Age: 20"));
    }
}
