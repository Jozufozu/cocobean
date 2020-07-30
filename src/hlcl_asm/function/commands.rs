use crate::Tag;
use crate::function::Target;

#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    Tag(Target, TagArgs),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum TagArgs {
    Add(Tag),
    Remove(Tag),
    List,
}