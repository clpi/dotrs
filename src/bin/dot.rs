use std::io;
use dot::cli::prompt;
use serde::{Serialize, Deserialize};

fn main() -> io::Result<()> {
    prompt()?;
    Ok(())
}
