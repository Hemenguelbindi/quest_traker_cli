use clap::{Parser, Subcommand};
use serde::{Serialize, Deserialize};
use std::fs;
use std::path::Path;
use std::collections::HashMap;

const FILE_PATH: &str = "tasks.json"; // Используй одно имя файла для сохранения и загрузки

#[derive(Parser)]
#[command(name ="task_traker", version = "0.1", author = "HemenguelBindi")]
struct TaskTracker{
    #[command(subcommand)]
    commands: Commnads
}

#[derive(Subcommand)]
enum Commnads {
    Add {
        #[arg()]
        task: String,
    },
    Complete {
        #[arg()]
        id: u32,
    },
    Remove {
        #[arg()]
        id: u32,
    },
    MarkInProgress{
        #[arg()]
        id: u32,
    },
    ShowAll,
    ShowDone,
    ShowInProgress,
    ShowTodo,
}

#[derive(Serialize, Deserialize, Debug)]
struct Quest {
    id: u32,
    task: String,
    description: String,
    prize: String,
    completed: bool,
    progress: bool,
}

fn load_task() -> HashMap<u32, Quest> {
    if Path::new(FILE_PATH).exists() {
        let data = fs::read_to_string(FILE_PATH).unwrap();
        let tasks: HashMap<u32, Quest> = serde_json::from_str(&data).unwrap();
        tasks
    } else {
        HashMap::new()
    }
}

fn save_task(tasks: &HashMap<u32, Quest>) {
    let data = serde_json::to_string_pretty(tasks).expect("Не удалось сериализовать");
    fs::write(FILE_PATH, data).expect("Ну удалось записать задачи в файл.")
}

fn main() {
    let args = TaskTracker::parse();
    let mut tasks = load_task();

    match &args.commands {
        Commnads::Add { task } => {
            let new_id = tasks.len() as u32 + 1;
            let new_task = Quest {
                id: new_id,
                task: task.to_string(),
                description: "".to_string(),
                prize: "".to_string(),
                completed: false,
                progress: false,
            };
            tasks.insert(new_id, new_task);
            save_task(&tasks);
        }
        Commnads::Complete { id } => {
            if let Some(task) = tasks.get_mut(id) {
                task.completed = true;
                if task.progress {
                    task.progress = false;
                }
                save_task(&tasks);
                println!("Задача с id {} успешно выполнена", id);
            } else {
                println!("Задача с id {} не найдена", id);
            }
        }
        Commnads::Remove { id } => {
            if let Some(task) = tasks.remove(id) {
                println!("Удалена задача: {}", task.task);
                save_task(&tasks);
            } else {
                println!("Задача с id {} не найдена", id);
            }
        }
        Commnads::ShowAll => {
            if tasks.is_empty() {
                println!("Список задач пуст.");
            } else {
                for task in tasks.values() {
                    let status = if task.completed { "✔" } else { "❌" };
                    println!("[{}] {}: {}", status, task.id, task.task);
                }
            }
        }
        Commnads::ShowDone => {
            if tasks.is_empty() {
                println!("Список задач пуст.")
            } else {
                for task in tasks.values() {
                    if task.completed{
                        println!("[✔] {}: {}",  task.task, task.prize);
                    }
                }
            }
        }
        Commnads::MarkInProgress { id } =>{
            if let Some(task) = tasks.get_mut(id){
                if task.completed {
                    println!("[{}], Задача {} выполнена и не может быть взята в работу.", id, task.task);
                }else{
                    task.progress = true;
                    save_task(&tasks);
                    println!("Задача с id {} взята в работу", id);
                }
            } else {
                println!("Задача с id {} не найдена", id);
            }
        }
        Commnads::ShowInProgress => {
            if tasks.is_empty(){
                println!("Список задач пуст.")
            } else {
                for task in tasks.values() {
                    if task.progress {
                        println!("[?] {}: {}", task.task, task.prize);
                    } 
                    else {
                        println!("Нет задачь в работе.")
                    }
            }
        }
    }

        Commnads::ShowTodo => {
            if tasks.is_empty() {
                println!("Список задач пуст.");
            }
            else {
                for task in tasks.values() {
                    if !task.completed && !task.progress {
                        println!("[ ] {}: {}", task.task, task.prize);
                    } else {
                        println!("К сожалению заметок нет!");
                    }
                }
            }
        }
    }

}

