#[warn(dead_code)]
use std::collections::HashMap;
use serde_json;
//use std::str::FromStr;
//use std::io::Read;

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
        let file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("db.json")?;
        match serde_json::from_reader(file) {
            Ok(tasks) => Ok(Todo { tasks }),
            Err(e) if e.is_eof() => Ok(Todo {
                tasks: HashMap::new()
            }),
            Err(e) => panic!("An error occurred: {}", e)
        }
        //let mut content = String::new();
        //file.read_to_string(&mut content)?;
        //let tasks: HashMap<String, bool> = content
        //    .lines()
        //    .map(|line| line.splitn(2, '\t').collect::<Vec<&str>>())
        //    .map(|vector| (vector[0], vector[1]))
        //    .map(|(key, value)| (
        //            String::from(key),
        //            bool::from_str(value).unwrap()))
        //    .collect();
        //Ok(Todo { tasks })
    }
    fn insert(&mut self, key: String) {
        self.tasks.insert(key, false);
    }
    fn save(self) -> Result<(), Box<dyn std::error::Error>> {
        let file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open("db.json")?;
        serde_json::to_writer_pretty(file, &self.tasks)?;
        Ok(())
    }
    //fn save(self) -> Result<(), std::io::Error> {
    //    let mut content = String::new();
    //    for (k, v) in self.tasks {
    //        let record = format!("{}\t{}\n", k, v);
    //        content.push_str(&record);
    //    }
    //    // If a semicolon is added, Result will complain
    //    std::fs::write("db.txt", content)
    //}
    fn complete(&mut self, key: &String) -> Option<()> {
        match self.tasks.get_mut(key) {
            Some(v) => Some(*v = true),
            None    => None
        }
    }
}
