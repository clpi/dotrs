use rustyline::{Context, hint::{Hint, Hinter, HistoryHinter}};

pub struct DotHinter {
    history: HistoryHinter,
    cmd: DotCmdHinter,
}

pub struct DotCmdHinter;

impl Hinter for DotHinter {
    type Hint = String;

    fn hint(&self, line: &str, pos: usize, ctx: &Context<'_>) -> Option<String> {
        if line == "h" {
            self.history.hint(line, pos, ctx)
        } else {
            Some(format!("HINT {}, {}", line, pos))
        }
    }
}

impl Hinter for DotCmdHinter {
    type Hint = String;

    fn hint(&self, line: &str, pos: usize, ctx: &Context<'_>) -> Option<String> {
        Some(format!("CMD HINT {}, {}", line, pos))
    }
}

