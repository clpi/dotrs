pub mod helper;
pub mod args;

use crate::{fmt, config::DotConfig, error::DotCliResult,};
use helper::DotHelper;
use rustyline::{
    Editor,
    config::Configurer,
    error::ReadlineError,
};
use termcolor::{Color, ColorSpec, WriteColor, ColorChoice, StandardStream};
use std::{
    env, io::{self, Write}, borrow::Cow,
};

#[derive(Default, Debug)]
pub(crate) struct DotArgs {
    verbose: bool,
}

impl DotArgs {

    pub fn from_args() -> DotCliResult<Self> {
        let mut dargs = Self::default();
        for arg in env::args().into_iter() {
            fmt::print::pyellow(&format!("Got arg: {}", &arg))?;
            match arg.as_str() {
                "-v" | "--verbose" => { dargs.verbose = true; },
                _ => { continue; }
            }
        }
        fmt::print::pgreen(&format!("{:#?}", dargs));
        Ok(dargs)
    }
}

pub fn prompt() -> io::Result<()> {

    let mut rl = Editor::<()>::new();
    rl.save_history("dot/assets/dothist")
        .expect("Error saving out to history");
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    rl.set_edit_mode(rustyline::EditMode::Vi);
    if rl.load_history("dot/assets/dothist").is_err() {
        fmt::print::pyellow("No previous history to load.")?;
    }
    fmt::print::pgreen("dot. service v0.1.0-development 2021")?;
    loop {
        let readline = rl.readline("dot. > ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(&line);
                println!("[out]: {}", line);
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    rl.save_history("assets/dothist").unwrap();
    Ok(())
}
