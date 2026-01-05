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
    Create {
        todo_name: String
    },

    #[command(arg_required_else_help = true)]
    Add {
        todo_name: String,

        value: String,

        #[arg(required = false)]
        task_name: Option<String>
    },

    #[command(arg_required_else_help = true)]
    Complete {
        todo_name: String,

        #[arg(required = false)]
        task_name: Option<String>
    },

    #[command(arg_required_else_help = true)]
    Remove {
        todo_name: String,

        #[arg(required = false)]
        task_name: Option<String>,
    },

    List {
        todo_name: String,

        #[arg(required = false)]
        task_name: Option<String>
    }
}