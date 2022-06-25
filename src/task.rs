use crate::utils::timestamp;

pub struct Task {
    pub title: String,
    pub description: Option<String>,
    pub create_timestamp: u64,
    pub done_timestamp: u64,
    pub done: bool,

    /// index of the task in its task list
    pub index: u64,

    /// primary key, id is None if and only if it is just added by 'add' command, 
    /// which will be automatically assigned by sqlite.
    pub id: Option<u64>,

    /// foreign key
    pub task_list_id: u64,
}

impl Task {
    /// save changes back to database file
    pub fn commit(&mut self, conn: &rusqlite::Connection) {
        if let Some(id) = self.id {
            conn.execute(
                "UPDATE tasks
                 SET title = (?1),
                     description = (?2),
                     create_timestamp = (?3),
                     done_timestamp = (?4),
                     done = (?5),
                     idx = (?6)
                 WHERE id = (?7)
                ",
                rusqlite::params![
                    self.title,
                    self.description,
                    self.create_timestamp,
                    self.done_timestamp,
                    self.done,
                    self.index,
                    id
                ],
            )
            .expect("unable to execute sql command");
        } else {
            conn.execute(
                "INSERT INTO tasks (task_list_id, title, description, create_timestamp, done_timestamp, done, idx)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                rusqlite::params![self.task_list_id, self.title, self.description, self.create_timestamp, self.done_timestamp, self.done, self.index],
            )
            .expect("unable to execute sql command");
            self.id = Some(conn.last_insert_rowid() as u64);
        }
    }
}

pub struct TaskList {
    pub tasks: Vec<Task>,
    pub create_timestamp: u64,
    pub name: Option<String>,

    /// primary key, id is None if and only if it is just added by 'create' command, 
    /// which will be automatically assigned by sqlite.
    pub id: Option<u64>,
}

impl TaskList {
    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    /// remove a task from database, this will issued by 'remove' command.
    pub fn remove_task(&mut self, index: usize, conn: &rusqlite::Connection) {
        if index <= 0 || index > self.tasks.len() {
            panic!("index out of range");
        }

        let task = self.tasks.remove(index - 1);
        conn.execute("DELETE FROM tasks WHERE id = ?", [task.id.unwrap()])
            .expect("unable to delete task from database");
    }

    /// done a task, this will issued by 'done' command.
    pub fn done_task(&mut self, index: usize) {
        if index <= 0 || index > self.tasks.len() {
            panic!("index out of range");
        }
        self.tasks[index - 1].done = true;
        self.tasks[index - 1].done_timestamp = timestamp();
    }

    /// undone a task, this will issued by 'undone' command.
    pub fn undone_task(&mut self, index: usize) {
        if index <= 0 || index > self.tasks.len() {
            panic!("Index not available.");
        }
        self.tasks[index - 1].done = false;
        self.tasks[index - 1].done_timestamp = 0;
    }

    /// set current task list as active task list
    pub fn set_active(&self, conn: &rusqlite::Connection) {
        conn.execute(
            "UPDATE active
             SET task_list_id = ?",
            [self.id.unwrap()],
        )
        .expect("unable to update database in set_active()");
    }

    /// save changes back to database file
    pub fn commit(&mut self, conn: &rusqlite::Connection) {
        if let Some(id) = self.id {
            conn.execute(
                "UPDATE task_lists
                 SET name = (?1),
                     create_timestamp = (?2)
                 WHERE id = (?3)
                ",
                rusqlite::params![self.name, self.create_timestamp, id],
            )
            .expect("unable to execute sql command");
        } else {
            conn.execute(
                "INSERT INTO task_lists (name, create_timestamp) VALUES (?1, ?2)",
                rusqlite::params![self.name, self.create_timestamp],
            )
            .expect("unable to execute sql command");
            self.id = Some(conn.last_insert_rowid() as u64);
        }

        for task in self.tasks.iter_mut() {
            task.commit(conn);
        }
    }
}
