#[warn(dead_code)]
use std::collections::HashMap;
use std::str::FromStr;
use std::io::Read;

fn main() {
    let action  = std::env::args()
        .nth(1).expect("Please specify an action");
    let item    = std::env::args()
        .nth(2).expect("Please select and item");
    println!("{:?}, {:?}", action, item);

    let mut todo = Todo::new().expect("Initialization of db failed");
    if action == "add" {
        todo.insert(item);
        match todo.save() {
            Ok(_) => println!("Todo saved."),
            Err(why) => println!("An error occurred: {}", why)
        }
    } else if action == "complete" {
        match todo.complete(&item) {
            None => println!("The task: '{}' isn't in the list", item),
            Some(_) => match todo.save() {
                Ok(_) => println!("Todo saved."),
                Err(why) => println!("An error occurred: {}", why)
            }
        }
    }

    println!("Hello, world!");
}

struct Todo {
    tasks: HashMap<String, bool>
}

impl Todo {
    fn new() -> Result<Todo, std::io::Error> {
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("db.txt")?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        // allocate an empty HashMap
        let mut tasks = HashMap::new();
        for entries in content.lines() {
            let mut values  = entries.split('\t');
            let key         = values.next().expect("No key");
            let value       = values.next().expect("No value");
            tasks.insert(String::from(key),
                        bool::from_str(value).unwrap());
        }
        Ok(Todo { tasks })
    }
    fn insert(&mut self, key: String) {
        self.tasks.insert(key, false);
    }
    fn save(self) -> Result<(), std::io::Error> {
        let mut content = String::new();
        for (k, v) in self.tasks {
            let record = format!("{}\t{}\n", k, v);
            content.push_str(&record);
        }
        // If a semicolon is added, Result will complain
        std::fs::write("db.txt", content)
    }
    fn complete(&mut self, key: &String) -> Option<()> {
        match self.tasks.get_mut(key) {
            Some(v) => Some(*v = true),
            None    => None
        }
    }
}
