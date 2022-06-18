# rs-todo
[![Rust Build Status](https://github.com/jbatch/rs-todo/actions/workflows/rust.yml/badge.svg?branch=master)](https://github.com/jbatch/rs-todo/actions/workflows/rust.yml)

## Installation

Clone the Repo 
```
git clone git@github.com:jbatch/rs-todo.git
cd rs-todo
```

### Cargo

* Install the rust toolchain in order to have cargo installed by following
  [this](https://www.rust-lang.org/tools/install) guide.
* build `cargo build`. Builds executable at `target/debug/todo`
* run `cargo run`

## Usage

Initalise a storage file for the tool to use.

```
todo init
Successfully initalised storage for todo
```

Print current incomplete todo list items
```
todo list
TODO List

     1. [ ] Walk the dog 
     3. [ ] Complete TODO list
```

Print all todo list items
```
todo list -a
TODO List

     1. [ ] Walk the dog 
     2. [X] Fold the laundry 
     3. [ ] Complete TODO list
```

Create a new todo list item
```
todo new "Update README.ms"
New item (4) added to to todo list.
```

Complete an item on the todo list
```
todo complete 3
Item 3 (Complete TODO list) completed.

todo list -a
TODO List

     1. [ ] Walk the dog 
     2. [X] Fold the laundry 
     3. [X] Complete TODO list
     4. [ ] Update README.md
```

## License

Licensed under
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
