

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true)
        .build_client(false)
        .compile(
            &["../proto/image_data_handler.proto"],
            &["../proto/"],
        )?;

    tonic_build::configure()
        .build_server(false)
        .build_client(true)
        .compile(
            &["../proto/visualization_service.proto"],
            &["../proto/"],
        )?;
    Ok(())
}