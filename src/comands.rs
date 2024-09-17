use clap::{Parser, Subcommand};



#[derive(Parser)]
#[command(author="Hemenguelbindi", version="0.1",)]
pub struct QuestTracker{
    #[command(subcommand)]
    pub commands: Commands,
}


#[derive(Subcommand)]
pub enum Commands {
    Add { task: String},
    Update { id: u32},
    Complete { id: u32},
    Remove { id: u32},
    MarkInProgress {id: u32},
    ShowAll,
    ShowDone,
    ShowInProgress,
    ShowToDo,
}