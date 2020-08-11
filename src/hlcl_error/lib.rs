use annotate_snippets::snippet;
use crossbeam_channel::{Receiver, Sender};
use hlcl_span::sourcemap::SourceMap;

pub type Error = String;

pub struct Handler {
    send: Sender<Error>,
    source: SourceMap,
}

impl Handler {

}
