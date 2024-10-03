use std::path::Path;

use semver::Version;
use valhall_docs::RustdocBuilder;

fn main() {
    let source = Path::new("./storage").to_owned();
    let output = Path::new("./generated_docs").to_owned();

    let doc_builder = RustdocBuilder::init(source, output);

    doc_builder.add_to_queue("third_test".into(), Version::new(0, 2, 0));

    doc_builder.builder_thread_handle.join().unwrap();
}
