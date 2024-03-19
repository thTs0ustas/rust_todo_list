use std::str::FromStr;
use std::{collections::HashMap, fs::read_to_string, io::Error};

struct Todo {
    map: HashMap<String, bool>,
}

impl Todo {
    fn insert(&mut self, key: String) {
        self.map.insert(key, true);
    }
    fn save_to_fs(self) -> Result<(), Error> {
        let mut data = String::new();
        self.map
            .iter()
            .for_each(|(key, value)| data.push_str(&format!("{}\t{}\n", key, value)));
        std::fs::write("todo.txt", data)
    }
    fn new() -> Result<Todo, Error> {
        let data = read_to_string("todo.txt")?;

        let map = data
            .lines()
            .map(|line| line.split('\t').collect::<Vec<&str>>())
            .map(|v| (v[0], v[1]))
            .map(|(key, value)| (String::from(key), bool::from_str(value).unwrap_or(false)))
            .collect();
        Ok(Todo { map })
    }
    fn complete(&mut self, key: &String) -> Option<()> {
        match self.map.get_mut(key) {
            Some(v) => Some(*v = false),
            None => None,
        }
    }
}

fn main() {
    let action = std::env::args().nth(1).expect("No action provided");
    let item = std::env::args().nth(2).expect("No item provided");

    let mut todo = Todo::new().expect("Initialisation failed");

    match action.as_str() {
        "add" => {
            todo.insert(item);
            todo.save_to_fs().expect("Save failed");
        }
        "complete" => {
            todo.complete(&item).expect("Item not found");
            todo.save_to_fs().expect("Save failed");
        }

        _ => println!("Invalid action"),
    };
}
