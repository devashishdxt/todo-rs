use std::io::Result;

use structopt::StructOpt;

use self::command::Command;
use self::handler::handle;

mod command;
mod service;
mod handler;

fn main() -> Result<()> {
    handle(Command::from_args())
}
