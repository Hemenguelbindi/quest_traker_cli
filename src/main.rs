use clap::{Parser, Subcommand};
use serde::{Serialize, Deserialize};
use std::fs;
use std::path::Path;
use std::collections::HashMap;
use chrono::Utc;


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
    Update{
        #[arg()]
        id: u32,
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
    status: String,
    crate_at: String,
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
            let now = Utc::now();
            let crate_at = now.format("%Y-%m-%d %H:%M:%S").to_string();

            let new_id = tasks.len() as u32 + 1;
            let new_task = Quest {
                id: new_id,
                task: task.to_string(),
                description: "".to_string(),
                prize: "".to_string(),
                status: "".to_string(),
                crate_at: crate_at,
            };
            tasks.insert(new_id, new_task);
            save_task(&tasks);
        }
        Commnads::Update { id } => {
            let now = Utc::now();
            let update_at = now.format("%Y-%m-%d %H:%M:%S").to_string();
            if let Some(task) = tasks.get_mut(id) {
                println!("Введите новые данные для задачи с id {}", id);
                println!("[ ]: {},", task.task);
                println!("Введите новую задачу:");
                let mut new_task = String::new();
                std::io::stdin().read_line(&mut new_task).expect("Не удалось прочитать ввод");
                task.task = new_task.trim().to_string();
                task.crate_at = update_at;
                save_task(&tasks);

            }
        }
        Commnads::Complete { id } => {
            if let Some(task) = tasks.get_mut(id) {
                task.status = "done".to_string();
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
                    let status = if task.status == "done" { "✔" } else { "❌" };
                    println!("[{}] {}: {}", status, task.id, task.task);
                }
            }
        }
        Commnads::ShowDone => {
            if tasks.is_empty() {
                println!("Список задач пуст.")
            } else {
                for task in tasks.values() {
                    if task.status == "done"{
                        println!("[✔] {}: {}",  task.task, task.prize);
                    }
                }
            }
        }
        Commnads::MarkInProgress { id } =>{
            if let Some(task) = tasks.get_mut(id){
                if task.status == "done" {
                    println!("[{}], Задача {} выполнена и не может быть взята в работу.", id, task.task);
                }else{
                    task.status = "in progress".to_string();
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
                    if task.status == "in progress"{
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
                    if task.status == ""{
                        println!("[ ] {}: {}", task.task, task.prize);
                    } else {
                        println!("К сожалению заметок нет!");
                    }
                }
            }
        }
    }

}

