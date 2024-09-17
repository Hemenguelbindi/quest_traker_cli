mod comands;
mod quest;
mod quest_manager;
mod utils;

use clap::Parser;
use comands::QuestTracker;
use quest_manager::TaskManager;

fn main() {
    let args = QuestTracker::parse();
    let mut manager = TaskManager::new();

    
    manager.execute(args.commands);
}
