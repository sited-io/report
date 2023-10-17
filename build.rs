use std::io::Result;

fn main() -> Result<()> {
    const MEDIA_PROTOS: &[&str] =
        &["service-apis/proto/peoplesmarkets/report/v1/report.proto"];

    const INCLUDES: &[&str] = &["service-apis/proto"];

    tonic_build::configure()
        .out_dir("src/api")
        .protoc_arg("--experimental_allow_proto3_optional")
        .file_descriptor_set_path("src/api/FILE_DESCRIPTOR_SET")
        .build_client(false)
        .build_server(true)
        .compile(MEDIA_PROTOS, INCLUDES)?;

    Ok(())
}
