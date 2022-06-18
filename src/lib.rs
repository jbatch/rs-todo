use chrono::{
    serde::{ts_seconds, ts_seconds_option},
    DateTime, Local, Utc,
};

use serde_derive::{Deserialize, Serialize};
use std::{
    error::Error,
    fs::{self, OpenOptions},
    io::ErrorKind,
    path::PathBuf,
};

use clap::{Parser, Subcommand};

#[derive(Subcommand, Debug)]
pub enum Command {
    #[clap(display_order = 1)]
    /// Initalise storage for todo command to use for persistance.
    Init {},
    /// Add new item to todo list.
    #[clap(display_order = 2)]
    New {
        #[clap(value_parser)]
        todo: String,
    },
    #[clap(display_order = 3)]
    /// Mark item <id> as done.
    Complete {
        #[clap(value_parser)]
        id: i32,
    },
    #[clap(display_order = 4)]
    /// Print todo list.
    List {
        #[clap(short, long, value_parser)]
        all: bool,
        #[clap(short, long, value_parser)]
        verbose: bool,
    },
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Command to run
    #[clap(subcommand)]
    pub command: Command,
}

/// Represents an item on the todo list and it's state.
#[derive(Deserialize, Serialize, Debug)]
pub struct ToDoItem {
    /// Id of the item. Used to identify which item to complete.
    pub id: i32,
    /// Text describing what the thing to do is.
    pub text: String,
    /// Whether the item has been completed.
    pub done: bool,
    /// When the item was added to the todo list.
    #[serde(with = "ts_seconds")]
    pub created_date: DateTime<Utc>, // Unix date.
    /// When the item was competed. Only present when done is `true`.
    #[serde(with = "ts_seconds_option")]
    pub completed_date: Option<DateTime<Utc>>, // Unix date.
}

impl ToDoItem {
    fn format_verbose_details(self: &Self, verbose: bool) -> String {
        if !verbose {
            return "".to_string();
        }

        let completed_details = if let Some(d) = self.completed_date {
            format!(
                " completed: {}",
                d.with_timezone(&Local)
                    .format("%Y-%m-%d %H:%M:%S")
                    .to_string()
            )
        } else {
            "".to_string()
        };
        format!(
            "(created: {}{})",
            self.created_date
                .with_timezone(&Local)
                .format("%Y-%m-%d %H:%M:%S")
                .to_string(),
            completed_details
        )
    }

    /**
    Returns string representation of the ToDoItem

    # Parameters
    * `verbose: bool` - when true returns a more detailed version of the [ToDoItem] string.

    # Examples
    ```rust
    use chrono::{DateTime,Utc};
    use todo::ToDoItem;

    let item = ToDoItem {
      id: 1,
      text: String::from("Walk the dog"),
      done: false,
      created_date: DateTime::parse_from_rfc3339("2022-01-01T00:00:00Z")
        .unwrap()
        .with_timezone(&Utc),
      completed_date: None,
    };
    assert_eq!(item.to_string(false), "  1. [ ] Walk the dog ");
    ```
    */
    pub fn to_string(self: &Self, verbose: bool) -> String {
        let padded_id = format!("{}.", self.id);
        let done = if self.done { "X" } else { " " };
        let verbose_details = self.format_verbose_details(verbose);
        format!(
            "{: >4} [{}] {} {}",
            padded_id, done, self.text, verbose_details
        )
    }
}

/**
Read contents of `~/.todo/todo.json` into [Vec]<[ToDoItem]>.

# Errors
Returns [None] when ~/.todo/todo.json cannot be opened.

# Panics
Panics when contents of `~/.todo/todo.json` cannot be deserialised into [Vec]<[ToDoItem]>.
*/
pub fn load_todo_list_from_storage() -> Option<Vec<ToDoItem>> {
    match fs::read_to_string(dirs::home_dir().unwrap().join(".todo").join("todo.json")) {
        Ok(contents) => match serde_json::from_str::<Vec<ToDoItem>>(&contents) {
            Ok(todo_list) => Some(todo_list),
            Err(error) => panic!("Bad {}", error),
        },
        Err(_) => None,
    }
}

/**
Writes a [Vec]<[ToDoItem]> to storage file at `~/.todo/todo.json` as JSON.

# Errors
Returns [Err] when writing to `~/.todo/todo.json` returns an [std::io::Error]

Returns [Err] when [Vec]<[ToDoItem]> can't be serialized to JSON.
*/
pub fn save_todo_list_to_storage(todo_list: Vec<ToDoItem>) -> Result<(), Box<dyn Error>> {
    let string_to_write: String = serde_json::to_string(&todo_list)?;
    fs::write(
        dirs::home_dir().unwrap().join(".todo").join("todo.json"),
        string_to_write,
    )?;
    Ok(())
}

pub fn handle_to_do_command(args: Command) {
    match args {
        Command::Init {} => handle_init(),
        Command::New { todo } => handle_new(todo),
        Command::Complete { id } => handle_complete(id),
        Command::List { all, verbose } => handle_list(all, verbose),
    }
}

fn create_storage_file(path: PathBuf) {
    let display = path.display();
    match OpenOptions::new().write(true).create_new(true).open(&path) {
        Ok(_) => println!("Successfully initalised storage for todo"),
        Err(why) => println!("Couldn't create storage file {}: {}", display, why),
    }
}

fn handle_init() {
    let storage_dir_path = dirs::home_dir().unwrap().join(".todo");
    println!("path: {}", storage_dir_path.display());
    let storage_file_path = storage_dir_path.clone().join("todo.txt");
    let create_dir_result = fs::create_dir(storage_dir_path);
    match create_dir_result {
        Ok(()) => create_storage_file(storage_file_path),
        Err(error) => match error.kind() {
            ErrorKind::AlreadyExists => {
                // Create storage if not exists yet.
                create_storage_file(storage_file_path);
            }
            kind => println!(
                "Unexpected error occured initalising storage for todo: {}",
                kind
            ),
        },
    }
}

fn handle_list(all: bool, verbose: bool) {
    let todo_list = load_todo_list_from_storage();
    match todo_list {
        Some(todo_list) => {
            let todo_list: Vec<String> = todo_list
                .iter()
                .filter(|i| all || !i.done)
                .map(|i| i.to_string(verbose))
                .collect();
            println!("TODO List\n");
            for item in todo_list {
                println!("   {item}");
            }
        }
        None => println!("Couldn't load todo list from storage"),
    }
}

fn handle_new(todo_text: String) {
    let todo_list = load_todo_list_from_storage();
    match todo_list {
        Some(mut todo_list) => {
            let next_id = todo_list.iter().map(|i| i.id).max().unwrap_or(0) + 1;
            let new_item = ToDoItem {
                id: next_id,
                text: todo_text,
                done: false,
                created_date: Utc::now(),
                completed_date: None,
            };
            todo_list.push(new_item);
            match save_todo_list_to_storage(todo_list) {
                Ok(_) => println!("New item ({}) added to to todo list.", next_id),
                Err(_) => println!("Error: failed to write todo list to storage"),
            }
        }
        None => println!("Couldn't load todo list from storage"),
    }
}

fn handle_complete(id: i32) {
    let todo_list = load_todo_list_from_storage();
    match todo_list {
        Some(mut todo_list) => {
            if let Some(item_to_complete) = todo_list.iter_mut().find(|i| i.id == id && !i.done) {
                item_to_complete.done = true;
                item_to_complete.completed_date = Some(Utc::now());
                let completed_text = item_to_complete.text.clone();
                match save_todo_list_to_storage(todo_list) {
                    Ok(_) => println!("Item {} ({}) completed.", id, completed_text),
                    Err(_) => println!("Error: failed to write todo list to storage"),
                }
            } else {
                println!("Error: item {} not found.", id);
            }
        }
        None => println!("Couldn't load todo list from storage"),
    }
}
