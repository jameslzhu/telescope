extern crate lalrpop;

fn main() {
    // Compile lalrpop grammar files
    lalrpop::process_root().unwrap();
}
