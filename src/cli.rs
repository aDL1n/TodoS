use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "todo")]

pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {

    #[command(arg_required_else_help = true)]
    #[command(about = "Create new TODO list")]
    Create {
        todo_name: String
    },

    #[command(arg_required_else_help = true)]
    #[command(about = "Add task to TODO list")]
    Add {
        todo_name: String,

        value: String,

        #[arg(required = false)]
        task_name: Option<String>
    },

    #[command(arg_required_else_help = true)]
    #[command(about = "Complete task in TODO list")]
    Complete {
        todo_name: String,

        #[arg(required = false)]
        task_name: Option<String>
    },

    #[command(arg_required_else_help = true)]
    #[command(about = "Remove TODO list or task")]
    Remove {
        todo_name: String,

        #[arg(required = false)]
        task_name: Option<String>,
    },

    #[command(arg_required_else_help = true)]
    #[command(about = "Show all TODO list or task")]
    List {
        todo_name: String,

        #[arg(required = false)]
        task_name: Option<String>
    }
}