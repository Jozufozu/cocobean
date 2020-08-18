use std::ops::Range;
use std::slice::SliceIndex;

use coco_asm::selector::Selector;
use coco_ast_merger::AstFinalizer;
use coco_parse::ParsingSession;

/// TODO: Collect names and item signatures
/// TODO: Perform sanity check
/// TODO: Desugar/lower AST
/// TODO: Type checking oh god
///
/// TODO: ERROR MESSAGES
/// TODO: Attributes
/// TODO: Pretty print ast (fmt command)
///
/// long term
/// TODO: STANDARD LIBRARY
/// TODO: better project management (like Cargo.toml)
/// TODO: DOCUMENTATION (both language support and documenting the compiler)
///
fn main() {
    let session = ParsingSession::new("./library", "library").unwrap();

    let (mods, mut project) = session.parse_project().unwrap();

    let program = AstFinalizer::unify(mods, &mut project);

    let map = &project.sources;
}
