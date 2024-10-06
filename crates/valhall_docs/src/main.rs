use valhall_docs::builder::DocBuilder;

fn main() {
    tracing_subscriber::fmt::init();

    let mut doc_builder = DocBuilder::init().unwrap();

    doc_builder.build_package("test_project", "0.5.0");
}
