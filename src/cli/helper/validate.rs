use std::borrow::Cow;
use rustyline::{Context, Helper, Result as RlResult, validate::{MatchingBracketValidator, ValidationContext, ValidationResult, Validator}};

pub struct DotValidator {
    cmd: DotCmdValidator,
    key: DotKeyValidator,
    bracket: MatchingBracketValidator,
}
#[derive(Debug)]
pub(crate) struct DotCmdValidator;

#[derive(Debug)]
pub(crate) struct DotKeyValidator {

}

impl Validator for DotValidator {

    fn validate(&self, ctx: &mut ValidationContext) -> RlResult<ValidationResult> {
        match ctx.input() {
            "{" | "}" => Ok(ValidationResult::Valid(None)),
            "y" =>  Ok(ValidationResult::Valid(None)),
            "n" => Ok(ValidationResult::Invalid(None)),
            _ => Ok(ValidationResult::Incomplete)
        }
    }
}
