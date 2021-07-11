use rustyline::{
    Context, Helper, Result as RlResult, completion::Completer, highlight::{Highlighter, MatchingBracketHighlighter}, hint::{Hinter, HistoryHinter}, validate::{MatchingBracketValidator, ValidationContext, ValidationResult, Validator}};
pub mod highlight;
pub mod complete;
pub mod hint;
pub mod validate;
use self::{
    hint::DotHinter,
    complete::DotCompleter,
    validate::DotValidator,
    highlight::DotHighlighter,
};

pub struct DotHelper {
    cmd_completer: DotCompleter,
    highlighter: DotHighlighter,
    validator: DotValidator,
    cmd_hinter: DotHinter,
    prompt_str: String,
}
impl Completer for DotHelper {

    type Candidate = String;

    fn complete(&self, ln: &str, pos: usize, ctx: &Context<'_>) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        self.cmd_completer.complete(ln, pos, ctx)
    }
}
impl Hinter for DotHelper {
    type Hint = String;
    fn hint(&self, line: &str, pos: usize, ctx: &Context<'_>) -> Option<Self::Hint> {
        self.hinter.hint(line, pos, ctx)
    }
}

impl Highlighter for DotHelper {
    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(&'s self, prompt: &'p str, default: bool) -> std::borrow::Cow<'b, str> {
        self.highlighter.highlight_prompt(prompt, default)
    }
    fn highlight<'l>(&self, line: &'l str, pos: usize) -> std::borrow::Cow<'l, str> {
        self.highlighter.highlight(line, pos)
    }

    fn highlight_hint<'h>(&self, hint: &'h str) -> std::borrow::Cow<'h, str> {
        self.highlighter.highlight_hint(hint)
    }


    fn highlight_candidate<'c>(&self, candidate: &'c str, completion: rustyline::CompletionType) -> std::borrow::Cow<'c, str> {
        self.highlighter.highlight_candidate(candidate, completion)
    }

}

impl Validator for DotHelper {

    fn validate(&self, ctx: &mut ValidationContext) -> RlResult<ValidationResult> {
        self.validator.validate(ctx)
    }
}
