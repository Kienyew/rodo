# Rodo
**Rodo**: A command line todo application written in rust.


## Commands

1. `rodo`: without subcommand, simply list all task in current task list.
2. `rodo add <title> [description]`: add a new task to the current task list with optional [description].
3. `rodo remove <index>` remove the task with the specified index from active task list.
4. `rodo done <index>`: mark a task as completed.
5. `rodo undone <index>`: mark a task as non-complete (*i.e.* undo the 'done' command).
6. `rodo create [name]`: create a new task list with optional [name] and set the active task list to it.
7. `rodo commit`: set the active task list to none, so when next time calling `rodo`, nothing will show up.

Also see `rodo --help`, which provides no more information than here.


## Example


https://user-images.githubusercontent.com/31496021/175759273-59489b56-bcef-4766-8bb7-9c7d1b5c71f9.mp4


## Database structure

The back end database of this program is stored in one file (default to `~/.local/share/rodo/rodo.sqlite`), it has three tables

### 1. tasks

All individual tasks are stored in this table.

schema:

```sql
CREATE TABLE IF NOT EXISTS tasks (
	id INTEGER PRIMARY KEY,
	task_list_id INTEGER NOT NULL,
	title TEXT NOT NULL,
	idx INTEGER NOT NULL,
	description TEXT,
	create_timestamp INTEGER NOT NULL,
	done_timestamp INTEGER,
	done BOOL,

	FOREIGN KEY (task_list_id) REFERENCES task_lists (id)
)
```



### 2. task_lists

Each row in this table indicating a task list.

schema:

```sql
CREATE TABLE IF NOT EXISTS active (
	id INTEGER PRIMARY KEY,
	task_list_id INTEGER,
	FOREIGN KEY (task_list_id) REFERENCES task_lists (id)
)
```



### 3. active

This table has only one row with `id = 1`, indicates the current active task list.

| id   | task_list_id (Foreign key) |
| ---- | -------------------------- |
| 1    | ID or NULL                 |

Only one or zero task list can be active (when you call `rodo` without arguments, it lists all the tasks from active task list)

schema:

```sql
CREATE TABLE IF NOT EXISTS active (
	id INTEGER PRIMARY KEY,
	task_list_id INTEGER,
	FOREIGN KEY (task_list_id) REFERENCES task_lists (id)
)
```


## Dependencies

1. [clap](https://github.com/clap-rs/clap): **C**ommand **L**ine **A**rgument **P**arser.
2. [colored](https://crates.io/crates/colored): Colored output for terminal.
3. [directories](https://crates.io/crates/directories): Find data directory. (eg. `~/.local/share/`)
4. [rusqlite](https://github.com/rusqlite/rusqlite): Interface to sqlite.


## Notes

Statistics and historical informations not available yet.
