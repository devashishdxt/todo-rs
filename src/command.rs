use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "todo", about = "A simple command line todo app")]
/// A simple command line todo app
pub enum Command {
    #[structopt(name = "add")]
    /// Add a todo
    Add {
        /// Name of todo
        name: String,
    },
    #[structopt(name = "list")]
    /// List todos
    List {
        #[structopt(subcommand)]
        list_command: Option<ListCommand>
    },
    #[structopt(name = "done")]
    /// Mark todo as done
    Done {
        /// ID of todo
        id: u64,
    },
}

#[derive(StructOpt, Debug)]
pub enum ListCommand {
    #[structopt(name = "pending")]
    /// List pending todos
    Pending,
    #[structopt(name = "completed")]
    /// List completed todos
    Completed,
    #[structopt(name = "all")]
    /// List all todos
    All,
}