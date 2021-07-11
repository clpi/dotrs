use std::io::{self, Write};
use rustyline::highlight::Highlighter;
use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub(crate) mod print {

    pub(crate) fn pyellow(input: &str) -> io::Result<()> {
        let mut stdout = StandardStream::stdout(ColorChoice::Always);
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))?;
        stdout.write(input.as_bytes())?;
        Ok(())
        //write!(&mut stdout, "{}", input)
    }

    pub(crate) fn pgreen(input: &str) -> io::Result<()> {
        let mut stdout = StandardStream::stdout(ColorChoice::Always);
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
        stdout.write(input.as_bytes())?;
        Ok(())
    }

    pub(crate) fn err(input: &str) -> io::Result<()> {
        let bufwtr = BufferWriter::stderr(ColorChoice::Always);
        let mut buffer = bufwtr.buffer();
        buffer.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;
        buffer.write(input.as_bytes())?;
        // write!(&mut buffer, "{}", input)?;
        Ok(())
    }

}

pub(crate) fn clear() -> io::Result<()> {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.reset()?;
    Ok(())
}
