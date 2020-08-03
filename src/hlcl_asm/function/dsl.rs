
use crate::function::*;
use crate::function::commands::{Command, TagArgs};
use crate::{SelectorId, Tag};

#[inline]
pub fn tag(target: Target, args: TagArgs) -> Command {
    Command::Tag(target, args)
}