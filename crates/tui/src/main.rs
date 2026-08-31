use std::io;

use ratatui::DefaultTerminal;

mod backups;
mod conf;

fn main() -> io::Result<()> {
    ratatui::run(run_app)?;
    Ok(())
}

/*
- store the backups and the settings inside local data directory
- if deltarune path could not be found ask the user where the directory is located with the deltarune local data
    - store the location afterwards
*/

fn run_app(term: &mut DefaultTerminal) -> io::Result<()> {
    Ok(())
}
