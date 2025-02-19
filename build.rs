use std::io::Result;
fn main() -> Result<()> {
    tonic_build::configure()
        .build_client(true)
        .compile(&["drone.proto"], &["proto/"])?;
    Ok(())
}
