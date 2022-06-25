use crate::task::{Task, TaskList};
use colored::Colorize;

static CIRCLE: &str = "\u{25CF}";
static CHECK: &str = "\u{2713}";

pub struct Painter {}

impl Painter {
    pub fn paint_task_list(&self, task_list: &TaskList) -> String {
        let tasks_string = task_list
            .tasks
            .iter()
            .map(|task| self.paint_task(task))
            .collect::<Vec<String>>()
            .join("\n");

        if let Some(name) = &task_list.name {
            let sep = "=".repeat(name.len());
            format!(
                "{name}\n{sep}\n{tasks}",
                name = name,
                sep = sep,
                tasks = tasks_string
            )
        } else {
            format!("{}", tasks_string)
        }
    }

    pub fn paint_task(&self, task: &Task) -> String {
        let title = task.title.blue().bold();
        let status = if task.done {
            CHECK.green().bold()
        } else {
            CIRCLE.bright_yellow().bold()
        };

        if let Some(description) = task.description.as_ref() {
            format!(
                "{index}. {title}: {description} {status}",
                index = task.index,
                title = title,
                description = description,
                status = status
            )
        } else {
            format!(
                "{index}. {title}: {status}",
                index = task.index,
                title = title,
                status = status
            )
        }
    }
}
