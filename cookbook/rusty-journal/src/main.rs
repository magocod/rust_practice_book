use structopt::StructOpt;

mod cli;
mod tasks;

use cli::{Action::*, CommandLineArgs};
use tasks::Task;

use std::path::PathBuf;

use anyhow::anyhow;

// cargo run -- -j test-journal.json add "buy milk"
// cargo run -- -j test-journal.json add "take the dog for a walk"
// cargo run -- -j test-journal.json list
// cargo run -- -j test-journal.json done 2

fn find_default_journal_file() -> Option<PathBuf> {
    home::home_dir().map(|mut path| {
        path.push(".rusty-journal.json");
        path
    })
}

// fn main() {
//     // Get the command-line arguments.
//     let CommandLineArgs {
//         action,
//         journal_file,
//     } = CommandLineArgs::from_args();
//
//     // Unpack the journal file.
//     // let journal_file = journal_file.expect("Failed to find journal file");
//
//     let journal_file = journal_file
//         .or_else(find_default_journal_file)
//         .expect("Failed to find journal file.");
//
//     // Perform the action.
//     match action {
//         Add { text } => tasks::add_task(journal_file, Task::new(text)),
//         List => tasks::list_tasks(journal_file),
//         Done { position } => tasks::complete_task(journal_file, position),
//     }
//         .expect("Failed to perform action")
// }

fn main() -> anyhow::Result<()> {
    let CommandLineArgs {
        action,
        journal_file,
    } = CommandLineArgs::from_args();

    let journal_file = journal_file
        .or_else(find_default_journal_file)
        .ok_or(anyhow!("Failed to find journal file."))?;

    match action {
        Add { text } => tasks::add_task(journal_file, Task::new(text)),
        List => tasks::list_tasks(journal_file),
        Done { position } => tasks::complete_task(journal_file, position),
    }?;
    Ok(())
}
