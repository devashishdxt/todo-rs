use std::io::Result;

use super::command::{Command, ListCommand};
use super::service::{add, done, list_all, list_completed, list_pending};

pub fn handle(command: Command) -> Result<()> {
    match command {
        Command::Add { name } => add(name),
        Command::List { list_command } => match list_command {
            None => list_pending(),
            Some(list_command) => match list_command {
                ListCommand::Pending => list_pending(),
                ListCommand::Completed => list_completed(),
                ListCommand::All => list_all(),
            },
        },
        Command::Done { id } => done(id),
    }
}
