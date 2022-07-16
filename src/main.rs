mod app;
mod painter;
mod task;
mod utils;

use app::{
    AddArguments, Application, CommitArguments, DoneArguments, NewArguments, RemoveArguments,
    UndoneArguments,
};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
/// Default action with no subcommand is to list all tasks of active task list
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new task
    Add {
        /// Title of the task
        title: String,

        /// Optional description of the task
        description: Option<String>,
    },
    /// Remove an existing task
    Remove {
        /// Index of the task to remove
        index: usize,
    },
    /// Mark an existing task as done
    Done {
        /// Index of the task to mark as done
        index: usize,
    },
    /// Mark an existing task as undone
    Undone {
        /// Index of the task to mark as undone
        index: usize,
    },
    /// Create a new task list and replace it as active task list.
    New {
        /// Create a new task list with optional name
        name: Option<String>,
    },
    /// Commit the current task list, set active task list to none.
    Commit {},
}

fn main() {
    let cli = Cli::parse();
    let app = Application::init();
    match cli.command {
        // Insert a new task to current task list
        Some(Commands::Add { title, description }) => {
            app.add(AddArguments { title, description });
        }

        Some(Commands::Remove { index }) => {
            app.remove(RemoveArguments { index });
        }

        Some(Commands::Done { index }) => {
            app.done(DoneArguments { index });
        }

        Some(Commands::Undone { index }) => {
            app.undone(UndoneArguments { index });
        }

        Some(Commands::New { name }) => {
            app.new(NewArguments { name });
        }

        Some(Commands::Commit {}) => {
            app.commit(CommitArguments {});
        }

        // Default action = List all the task status
        _ => app.default(),
    }
}
