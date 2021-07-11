use rustyline::{
    complete::Completer,
};
#[derive(Debug)]
pub struct DotCmdCompleter {

}

impl Completer for DotCmdCompleter {

    type Candidate = String;


}
