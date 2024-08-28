use std::io;

#[derive(Debug, PartialEq)]
struct TODO {
  id: i32,
  title: String,
  description: String,
  done: bool,
}

#[derive(Debug)]
struct TODOList {
  todos: Vec<TODO>,
}

impl TODOList {
  fn new() -> TODOList {
    TODOList { todos: Vec::new() }
  }
  fn add_todo(&mut self, todo: TODO) {
    self.todos.push(todo);
  }
  fn has_todo(&self, id: i32) -> bool {
    self.todos.iter().any(|todo| todo.id == id)
  }
  fn get_todo(&self, id: i32) -> Option<&TODO> {
    self.todos.iter().find(|todo| todo.id == id)
  }
  fn remove_todo(&mut self, id: i32) {
    if !self.has_todo(id) {
      println!("No TODO found with the id {}", id);
      return;
    }

    self.todos.retain(|todo| todo.id != id);
  }
  fn toggle_todo(&mut self, id: i32) {
    self.todos.iter_mut().for_each(|todo| {
      if todo.id == id {
        todo.done = !todo.done;
      }
    });
  }
}

fn main() {
  let mut todo_list = TODOList::new();

  loop {
    println!("\n-------\n");
    println!(
            "TODOS\nSelect a choice\n\n 1. Add TODO\n 2. List TODOs\n 3. Remove TODO\n 4. Toggle TODO\n 5. Exit"
        );

    let mut choice = String::new();
    io::stdin()
      .read_line(&mut choice)
      .expect("Failed to read line");

    let choice = match choice.trim().parse::<i32>() {
      Ok(num) => num,
      Err(_) => {
        println!("Please enter a valid number");
        continue;
      }
    };

    match choice {
      1 => {
        println!("Enter the title of the TODO");
        let mut title = String::new();
        io::stdin()
          .read_line(&mut title)
          .expect("Failed to read line");

        println!("Enter the description of the TODO");
        let mut description = String::new();
        io::stdin()
          .read_line(&mut description)
          .expect("Failed to read line");

        let todo = TODO {
          id: 1,
          title: title.trim().to_string(),
          description: description.trim().to_string(),
          done: false,
        };

        todo_list.add_todo(todo);
      }
      2 => {
        let todos = &todo_list.todos;
        println!("Current todos: {:#?}", todos);
      }
      3 => {
        println!("Enter the id of the TODO to remove");
        let mut id = String::new();
        io::stdin().read_line(&mut id).expect("Failed to read line");

        let id = match id.trim().parse::<i32>() {
          Ok(num) => num,
          Err(_) => {
            println!("Please enter a valid number");
            continue;
          }
        };

        todo_list.remove_todo(id);
      }
      4 => {
        println!("Enter the id of the TODO to toggle");
        let mut id = String::new();
        io::stdin().read_line(&mut id).expect("Failed to read line");
        let id = match id.trim().parse::<i32>() {
          Ok(num) => num,
          Err(_) => {
            println!("Please enter a valid number");
            continue;
          }
        };

        if !todo_list.has_todo(id) {
          println!("No todo with id: {}", id);
          continue;
        }

        todo_list.toggle_todo(id);

        println!("TODO now is {:?}", todo_list.get_todo(id).unwrap());
      }
      5 => {
        println!("Exiting the program");
        break;
      }
      _ => {
        println!("Invalid choice");
      }
    }
  }
}
