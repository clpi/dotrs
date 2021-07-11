use rustyline::{
    completion::{Candidate, Completer},
    Context,
    Result as RlResult,
};

#[derive(Debug)]
pub struct DotCompleter {

}

impl Completer for DotCompleter {
    type Candidate = String;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        ctx: &Context<'_>
    ) -> RlResult<(usize, Vec<Self::Candidate>)>
    {
        Ok((0, vec![]))
    }

}
