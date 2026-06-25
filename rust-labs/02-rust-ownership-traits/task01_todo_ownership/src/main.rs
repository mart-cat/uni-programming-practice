#[derive(Debug, PartialEq, Eq)]
struct Task {
    id: u32,
    title: String,
    completed: bool,
}

impl Task {
    fn new(id: u32, title: String) -> Self {
        Self {
            id,
            title,
            completed: false,
        }
    }

    fn mark_completed(&mut self) {
        self.completed = true;
    }

    fn status_label(&self) -> &'static str {
        if self.completed { "done" } else { "open" }
    }
}

fn add_task(tasks: &mut Vec<Task>, id: u32, title: String) {
    tasks.push(Task::new(id, title));
}

fn complete_task(tasks: &mut [Task], id: u32) -> bool {
    if let Some(task) = tasks.iter_mut().find(|task| task.id == id) {
        task.mark_completed();
        true
    } else {
        false
    }
}

fn remove_task(tasks: &mut Vec<Task>, id: u32) -> bool {
    if let Some(position) = tasks.iter().position(|task| task.id == id) {
        tasks.remove(position);
        true
    } else {
        false
    }
}

fn count_completed(tasks: &[Task]) -> usize {
    tasks.iter().filter(|task| task.completed).count()
}

fn count_open(tasks: &[Task]) -> usize {
    tasks.len() - count_completed(tasks)
}

fn format_tasks(tasks: &[Task]) -> String {
    tasks
        .iter()
        .map(|task| format!("[{}] {}: {}", task.status_label(), task.id, task.title))
        .collect::<Vec<_>>()
        .join("\n")
}

fn main() {
    let mut tasks = Vec::new();

    add_task(&mut tasks, 1, String::from("Create Cargo project"));
    add_task(&mut tasks, 2, String::from("Read Rust ownership chapter"));
    add_task(&mut tasks, 3, String::from("Solve borrowing exercise"));

    complete_task(&mut tasks, 2);
    remove_task(&mut tasks, 1);

    println!("Todo list");
    println!("{}", format_tasks(&tasks));
    println!("Completed: {}", count_completed(&tasks));
    println!("Open: {}", count_open(&tasks));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_task_by_taking_title_ownership() {
        let mut tasks = Vec::new();
        let title = String::from("Learn ownership");
        add_task(&mut tasks, 1, title);
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].title, "Learn ownership");
    }

    #[test]
    fn completes_existing_task_through_mutable_slice() {
        let mut tasks = vec![
            Task::new(1, String::from("A")),
            Task::new(2, String::from("B")),
        ];
        assert!(complete_task(&mut tasks, 2));
        assert!(tasks[1].completed);
        assert_eq!(count_completed(&tasks), 1);
    }

    #[test]
    fn removes_task_from_vector() {
        let mut tasks = vec![
            Task::new(1, String::from("A")),
            Task::new(2, String::from("B")),
        ];
        assert!(remove_task(&mut tasks, 1));
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].id, 2);
    }
}
