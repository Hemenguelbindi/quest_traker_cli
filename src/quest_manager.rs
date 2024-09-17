use std::collections::HashMap;
use std::fs;

use std::path::Path;
use chrono::Utc;

use crate::quest::Quest;
use crate::comands::Commands;
use crate::utils::{print_header_table, print_table, input, ask_user};

const FILE_PATH: &str = "tasks.json";

pub struct TaskManager {
    tasks: HashMap<u32, Quest>,
}

impl TaskManager {
    pub fn new() -> TaskManager {
        TaskManager {
            tasks: TaskManager::load_tasks(),
        }
    }

    pub fn execute(&mut self, command: Commands) {
        match command {
            Commands::Add { task } => self.add_task(task),
            Commands::Update { id } => self.update_task(id),
            Commands::Complete { id } => self.complete_task(id),
            Commands::Remove { id } => self.remove_task(id),
            Commands::MarkInProgress { id } => self.mark_in_progress(id),
            Commands::ShowAll => self.show_all_tasks(),
            Commands::ShowDone => self.show_done_tasks(),
            Commands::ShowInProgress => self.show_in_progress_tasks(),
            Commands::ShowToDo => self.show_todo_tasks(),
        }
    }

    fn load_tasks() -> HashMap<u32, Quest> {
        if Path::new(FILE_PATH).exists() {
            let data = fs::read_to_string(FILE_PATH).unwrap();
            serde_json::from_str(&data).unwrap()
        } else {
            HashMap::new()
        }
    }

    fn save_tasks(&self) {
        let data = serde_json::to_string_pretty(&self.tasks).expect("Не удалось сериализовать");
        fs::write(FILE_PATH, data).expect("Не удалось записать задачи в файл.");
    }

    fn add_task(&mut self, task: String) {
        let new_id = self.tasks.len() as u32 + 1;
        let mut new_task = Quest::new(new_id, task);
        new_task.created_at = Utc::now().format("%H:%M:%S %d-%m-%Y").to_string();
        new_task.status = "not defined".to_string();
        println!();
        if ask_user("Хотите добавить описание квеста: Y/N> ") {
            new_task.description = input("Введите описание квеста >>> ");
        }
        if ask_user("Хотите установить награду за вполнение? Y/N>") {
            new_task.prize = input("Введите награду за вполнение >>> ");
        }
        self.tasks.insert(new_id, new_task);
        self.save_tasks();
    }

    fn update_task(&mut self, id: u32) {
        let task = self.tasks.get_mut(&id).unwrap();
        task.update_at = Utc::now().format("%H:%M:%S %d-%m-%Y").to_string();
        self.save_tasks();
    }

    fn complete_task(&mut self, id: u32) {
        let task = self.tasks.get_mut(&id).unwrap();
        task.created_at = Utc::now().format("%H:%M:%S %d-%m-%Y").to_string();
        task.status = "Done".to_string();
        self.save_tasks();
    }
    fn remove_task(&mut self, id: u32) {
        self.tasks.remove(&id);
        self.save_tasks();
    }

    fn mark_in_progress(&mut self, id: u32) {
        let task = self.tasks.get_mut(&id).unwrap();
        task.status = "InProgress".to_string();
        task.update_at = Utc::now().format("%H:%M:%S %d-%m-%Y").to_string();
        self.save_tasks();
    }

    fn show_all_tasks(&self) {
        print_header_table("ВСЕ КВЕСТЫ");
        if self.tasks.is_empty() {
            println!("Нет квестов");
        } else {
            print_table();
            for task in self.tasks.values() {
                let status = if task.status == "Done" { "✔"} else {"❌"};
                println!("[{}] {}, {}, {}", status, task.id, task.task, task.status);
            }
        }
    }

    fn show_done_tasks(&self) {
        print_header_table("ЗАВЕРШЕННЫЕ КВЕСТЫ");
        if self.tasks.is_empty() {
            println!("Нет квестов");
        } else {
            print_table();
            for task in self.tasks.values() {
                if task.status == "Done" {
                    println!("[✔] {}, {}, {}", task.id, task.task, task.status);
                }
            }
        }
    }

    fn show_in_progress_tasks(&self) {
        print_header_table("КВЕСТЫ НА ВЫПОЛНЕНИЕ");
        if self.tasks.is_empty() {
            println!("Нет квестов")
        }
        else {
            print_table();
            for task in self.tasks.values() {
                if task.status == "InPrigress".to_string() {
                    println!("[ ] {} {} {}", task.id, task.task, task.status);
                }    
            }
        }
    }

    fn show_todo_tasks(&self) {
        print_header_table("СВОБОДНЫЕ КВЕТЫ");
        print_table();
        for task in self.tasks.values() {
            if task.status == "not defined".to_string() {
                println!("[ ] {}, {}, {}", task.id, task.task, task.status);
            }
        }
    }
}



