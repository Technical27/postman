mod test;
mod random;
mod top;
mod new;

#[derive(Debug)]
pub struct CommandError {
    msg: String
}

impl CommandError {
    pub fn new(msg: &str) -> Self {
        Self { msg: msg.to_string() }
    }

    pub fn boxed(msg: &str) -> Box<Self> {
       Box::new(Self::new(msg))
    }
}

impl std::fmt::Display for CommandError {
    fn fmt (&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> { write!(f, "{}", self.msg) }
}
impl std::error::Error for CommandError {}

use super::*;

pub use test::*;
pub use random::*;
pub use top::*;
pub use new::*;
