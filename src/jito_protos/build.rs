fn main() -> Result<(), Box<dyn std::error::Error>> {
    const PROTOC_ENVAR: &str = "PROTOC";
    if std::env::var(PROTOC_ENVAR).is_err() {
        #[cfg(not(windows))]
        std::env::set_var(PROTOC_ENVAR, protobuf_src::protoc());
    }

    tonic_build::configure()
        .build_server(false)
        .compile(
            &[
                "protos/auth.proto",
                "protos/block_engine.proto",
                "protos/bundle.proto",
                "protos/relayer.proto",
                "protos/searcher.proto",
                "protos/shredstream.proto",
            ],
            &["protos"],
        )?;
    Ok(())
}
