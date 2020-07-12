extern crate lalrpop;

fn main() {
    lalrpop::Configuration::new()
        .generate_in_source_tree()
        .emit_rerun_directives(true)
        .emit_whitespace(false)
        .process_file("hlcl.lalrpop")
        .unwrap();
}
