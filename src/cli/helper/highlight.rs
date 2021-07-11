use std::borrow::Cow;
use rustyline::{
    highlight::{MatchingBracketHighlighter,Highlighter},
};

pub struct DotHighlighter {
    cmd: DotCmdHighlighter,
    bracket: MatchingBracketHighlighter
}

pub struct DotCmdHighlighter;

impl Highlighter for DotHighlighter {
    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
        &'s self,
        prompt_str: &'p str,
        default: bool
    ) -> std::borrow::Cow<'b, str>
    {
        if default {
            return Cow::Owned(format!("\x1b[1;32m{}\x1b[m", prompt_str));
        } else {
            return Cow::Borrowed(prompt_str);
        }
    }

    fn highlight_char(&self, line: &str, pos: usize) -> bool {
        let ch = line.chars().nth(pos);
        match ch {
            Some('{') | Some('}') => self.bracket.highlight_char(line, pos),
            _ => false,
        }
    }

    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        Cow::Owned(format!("\x1b[1m{}\x1b[m", hint))
    }
}
