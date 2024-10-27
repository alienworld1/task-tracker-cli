# task-tracker-cli

## How to run

Clone the repository and open the folder

```Bash
git clone https://github.com/alienworld1/task-tracker-cli.git
cd task-tracker-cli
```

To run the project, use the `cargo run -q` command

```Bash
cargo run -q -- <arguments>

# For example
cargo run -q -- add "Buy groceries"
```

## Features

- Add, Update, and Delete tasks
- Mark a task as in progress or done
- List all tasks
- List all tasks that are done
- List all tasks that are not done
- List all tasks that are in progress

## Usage

These commands can be used on the binary, downloaded from the release page

```Bash
# Adding a new task
./task-cli add "Buy groceries"
# Output: Task added successfully (ID: 1)

# Updating and deleting tasks
./task-cli update 1 "Buy groceries and cook dinner"
./task-cli delete 1

# Marking a task as in progress or done
./task-cli mark-in-progress 1
./task-cli mark-done 1

# Listing all tasks
./task-cli list

# Listing tasks by status
./task-cli list done
./task-cli list todo
./task-cli list in-progress
```

This is a solution for the [Task Tracker Project](https://roadmap.sh/projects/task-tracker).
