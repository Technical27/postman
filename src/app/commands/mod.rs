mod debug;
mod new;
mod random;
mod rising;
mod test;
mod top;

/*#[derive(Debug)]
pub struct CommandError {
    msg: String,
}

impl CommandError {
    pub fn new(msg: &str) -> Self {
        Self {
            msg: msg.to_string(),
        }
    }
}

impl std::fmt::Display for CommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.msg)
    }
}
impl std::error::Error for CommandError {} */

use super::*;

pub use debug::*;
pub use new::*;
pub use random::*;
pub use rising::*;
pub use test::*;
pub use top::*;
