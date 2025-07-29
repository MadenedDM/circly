fn main() {
    capnpc::CompilerCommand
        ::new()
        .src_prefix("../schemas")
        .file("../schemas/all.capnp")
        .run()
        .expect("schema compiler command");
}
