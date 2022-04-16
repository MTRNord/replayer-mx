fn main() {
    capnpc::CompilerCommand::new()
        .src_prefix("schema")
        .import_path("schema")
        .file("schema/utils.capnp")
        .file("schema/events.capnp")
        .file("schema/matrix.capnp")
        .run()
        .expect("schema compiler command");
}
