use crate::painter::Painter;
use crate::task::{Task, TaskList};
use crate::utils::timestamp;
use directories::ProjectDirs;
use std::path::PathBuf;

pub struct Application {
    conn: rusqlite::Connection,
}

pub struct AddArguments {
    /// the title of the new task
    pub title: String,

    /// the optional description of the new task
    pub description: Option<String>,
}

pub struct RemoveArguments {
    /// the index of task to remove, count from 1
    pub index: usize,
}

pub struct DoneArguments {
    /// the index of task to done, count from 1
    pub index: usize,
}

pub struct UndoneArguments {
    /// the index of task to undone, count from 1
    pub index: usize,
}

pub struct CreateArguments {
    /// the optional name of the task list
    pub name: Option<String>,
}

pub struct CommitArguments {}

/// Get the system standard data directory for this application (eg. ~/.local/share/rodo)
/// the directory will be created if not exist.
fn data_dir() -> PathBuf {
    let dir = if let Some(dir) = ProjectDirs::from("", "", "rodo") {
        dir.data_dir().to_path_buf()
    } else {
        // If somehow cannot get system standard directory,
        // use ${current_dir}/rodo as it.
        std::env::current_dir().unwrap().join("rodo")
    };

    std::fs::create_dir_all(&dir).expect("error creating data dir");
    dir
}

pub fn database_file_path() -> PathBuf {
    data_dir().join("rodo.sqlite")
}

fn init_database() -> rusqlite::Connection {
    // create table 'tasks'
    let conn = rusqlite::Connection::open(database_file_path()).expect("cannot open database file");
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks (
                id INTEGER PRIMARY KEY,
                task_list_id INTEGER NOT NULL,
                title TEXT NOT NULL,
                idx INTEGER NOT NULL,
                description TEXT,
                create_timestamp INTEGER NOT NULL,
                done_timestamp INTEGER,
                done BOOL,

                FOREIGN KEY (task_list_id) REFERENCES task_lists (id)
            )",
        [],
    )
    .expect("sqlite error");

    // create table 'task_lists'
    conn.execute(
        "CREATE TABLE IF NOT EXISTS task_lists (
                id INTEGER PRIMARY KEY,
                create_timestamp INTEGER NOT NULL,
                name TEXT
            )",
        [],
    )
    .expect("sqlite error");

    // create table 'active', this whole table stores only one value, which is the active task list
    conn.execute(
        "CREATE TABLE IF NOT EXISTS active (
            id INTEGER PRIMARY KEY,
            task_list_id INTEGER,
            FOREIGN KEY (task_list_id) REFERENCES task_lists (id)
        )",
        [],
    )
    .expect("sqlite error");

    // check if there is an active task list
    let active: bool = conn
        .prepare("SELECT task_list_id FROM active WHERE id = 1")
        .unwrap()
        .query_row([], |_| Ok(()))
        .is_ok();

    // initialize active table if there's no active task list
    if !active {
        conn.execute(
            "INSERT OR REPLACE INTO
                 active (id, task_list_id)
                 VALUES (1, NULL)",
            [],
        )
        .unwrap();
    }

    conn
}

impl Application {
    pub fn new() -> Self {
        Application {
            conn: init_database(),
        }
    }

    /// Read from json file containing the active task list
    fn get_active_task_list(&self) -> Option<TaskList> {

        // select the active task list
        let (task_list_id, create_timestamp, name): (u64, u64, Option<String>) = {
            let mut stmt = self
                .conn
                .prepare("SELECT task_list_id FROM active")
                .expect("sqlite error");

            let task_list_id: Option<u64> =
                stmt.query_row([], |row| row.get(0)).expect("sql error");

            if task_list_id.is_none() {
                return None;
            }

            let task_list_id = task_list_id.unwrap();
            let mut stmt = self
                .conn
                .prepare("SELECT create_timestamp, name FROM task_lists WHERE id = ?")
                .expect("sqlite error");

            let (create_timestamp, name) = stmt.query_row([task_list_id], |row| {
                Ok((row.get(0).unwrap(), row.get(1).unwrap()))
            })
            .unwrap();

            (task_list_id, create_timestamp, name)
        };

        // load all tasks into the task list
        let tasks: Vec<Task> = self.conn
            .prepare("SELECT id, title, description, create_timestamp, done_timestamp, done, idx FROM tasks WHERE task_list_id = ? ORDER BY idx")
            .unwrap()
            .query_map([task_list_id], |row|
                Ok(Task {
                    task_list_id,
                    id: row.get(0).unwrap(),
                    title: row.get(1).unwrap(),
                    description: row.get(2).unwrap(),
                    create_timestamp: row.get(3).unwrap(),
                    done_timestamp: row.get(4).unwrap(),
                    done: row.get(5).unwrap(),
                    index: row.get(6).unwrap(),
                })
            )
            .unwrap()
            .map(|r| r.unwrap())
            .collect();

        Some(TaskList {
            id: Some(task_list_id),
            name,
            create_timestamp,
            tasks,
        })
    }

    /// Process the add command
    pub fn add(&self, args: AddArguments) {
        let mut task_list = self
            .get_active_task_list()
            .expect("unable to read active task list");

        let index = if task_list.tasks.is_empty() {
            1
        } else {
            task_list.tasks.last().unwrap().index + 1
        };

        task_list.add_task(Task {
            id: None,
            create_timestamp: timestamp(),
            description: args.description,
            title: args.title,
            done: false,
            done_timestamp: 0,
            index,
            task_list_id: task_list.id.unwrap(),
        });

        task_list.commit(&self.conn);
    }

    /// Process the remove command
    pub fn remove(&self, args: RemoveArguments) {
        let mut task_list = self
            .get_active_task_list()
            .expect("unable to read active task list");
        task_list.remove_task(args.index, &self.conn);
        task_list.commit(&self.conn);
    }

    /// Process the done command
    pub fn done(&self, args: DoneArguments) {
        let mut task_list = self
            .get_active_task_list()
            .expect("unable to read active task list");
        task_list.done_task(args.index);
        task_list.commit(&self.conn);
    }

    /// Process the undone command
    pub fn undone(&self, args: UndoneArguments) {
        let mut task_list = self
            .get_active_task_list()
            .expect("unable to read active task list");
        task_list.undone_task(args.index);
        task_list.commit(&self.conn);
    }

    /// Process the create command
    pub fn create(&self, args: CreateArguments) {
        let mut task_list = TaskList {
            id: None,
            name: args.name,
            create_timestamp: timestamp(),
            tasks: vec![],
        };

        task_list.commit(&self.conn);
        task_list.set_active(&self.conn);
    }

    /// Process the commit command, 
    pub fn commit(&self, _args: CommitArguments) {
        self.conn.execute("DELETE FROM active WHERE id = 1", []).ok();
    }

    /// Default behaviour when no command provided
    pub fn default(&self) {
        if let Some(task_list) = self.get_active_task_list() {
            let painter = Painter {};
            let output = painter.paint_task_list(&task_list);
            println!("{}", output);
        } else {
            println!("no active task list")
        }
    }
}
